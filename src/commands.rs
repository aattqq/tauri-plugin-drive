use http_body_util::BodyExt;
use serde::Serialize;
use tauri::{command, AppHandle, Runtime};

use crate::DriveExt;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum DownloadEvent<'a> {
    #[serde(rename_all = "camelCase")]
    Started { 
        download_id: &'a str,
        filename: &'a str,
        content_length: usize,
    },
    #[serde(rename_all = "camelCase")]
    Progress {
        download_id: &'a str,
        chunk_length: usize,
    },
    #[serde(rename_all = "camelCase")]
    Finished { download_id: &'a str },
}

#[command]
pub(crate) async fn download_file<R: Runtime>(
    app: AppHandle<R>,
    file_id: String,
    on_event: tauri::ipc::Channel<DownloadEvent<'_>>,
) -> crate::Result<tauri::ipc::Response> {
    let file = app.drive().get_file_by_id(&file_id).await?;
    let download_id = &file.id.unwrap_or("default".to_string());
    let content_length = file.size.unwrap_or(0) as usize;

    on_event.send(DownloadEvent::Started {
        download_id,
        filename: &file.name.unwrap_or("default".to_string()),
        content_length,
    })?;

    let mut data = vec![];
    let mut response = app.drive().download_file_by_id(&file_id).await?;
    dbg!(response.headers());
    while let Some(next) = response.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            data.extend(chunk);
            on_event.send(DownloadEvent::Progress {
                download_id,
                chunk_length: chunk.len(),
            })?;
        }
    }

    on_event.send(DownloadEvent::Finished { download_id })?;
    Ok(tauri::ipc::Response::new(data))
}
