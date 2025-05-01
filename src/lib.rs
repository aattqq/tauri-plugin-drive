mod commands;
mod error;

pub use google_drive3::yup_oauth2::authorized_user::AuthorizedUserSecret;
use imp::Drive;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use error::{Error, Result};

mod imp {
    use google_drive3::{
        api::File,
        hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
        hyper_util::{
            self,
            client::legacy::{connect::HttpConnector, Client},
        },
        yup_oauth2::{authorized_user::AuthorizedUserSecret, AuthorizedUserAuthenticator},
        DriveHub,
    };

    /// Access to the drive APIs.
    pub struct Drive(tauri::async_runtime::Mutex<DriveHub<HttpsConnector<HttpConnector>>>);

    impl Drive {
        pub async fn get_file_by_id(&self, file_id: &str) -> crate::Result<File> {
            let drive = self.0.lock().await;
            let (_, file) = drive
                .files()
                .get(file_id)
                .supports_all_drives(true)
                .param("fields", "id, name, size")
                .doit()
                .await?;
            Ok(file)
        }

        pub async fn download_file_by_id(
            &self,
            file_id: &str,
        ) -> crate::Result<google_drive3::common::Response> {
            let drive = self.0.lock().await;
            let (response, ..) = drive
                .files()
                .get(file_id)
                .supports_all_drives(true)
                .param("alt", "media")
                .doit()
                .await?;

            Ok(response)
        }
    }

    pub(super) async fn init(authorized_user_secret: AuthorizedUserSecret) -> crate::Result<Drive> {
        let authenticator = match AuthorizedUserAuthenticator::builder(authorized_user_secret)
            .build()
            .await
        {
            Ok(authenticator) => authenticator,
            Err(e) => return Err(crate::Error::Io(e)),
        };

        let http_client = Client::builder(hyper_util::rt::TokioExecutor::new()).build(
            HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_or_http()
                .enable_http1()
                .build(),
        );

        let drive_hub = DriveHub::new(http_client, authenticator);
        Ok(Drive(tauri::async_runtime::Mutex::new(drive_hub)))
    }
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the drive APIs.
pub trait DriveExt<R: Runtime> {
    fn drive(&self) -> &Drive;
}

impl<R: Runtime, T: Manager<R>> crate::DriveExt<R> for T {
    fn drive(&self) -> &Drive {
        self.state::<Drive>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>(authorized_user_secret: AuthorizedUserSecret) -> TauriPlugin<R> {
    Builder::new("drive")
        .invoke_handler(tauri::generate_handler![commands::download_file])
        .setup(|app, _| {
            let app = app.app_handle().clone();
            tauri::async_runtime::spawn(async move {
                let drive = imp::init(authorized_user_secret)
                    .await
                    .expect("error on drive plugin setup");
                app.manage(drive);
            });
            Ok(())
        })
        .build()
}
