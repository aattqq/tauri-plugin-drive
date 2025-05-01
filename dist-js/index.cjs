'use strict';

var core = require('@tauri-apps/api/core');

function downloadFileByID({ fileId, onDownloadEvent }) {
    const onEvent = new core.Channel();
    onEvent.onmessage = onDownloadEvent;
    return core.invoke('plugin:drive|download_file', {
        fileId,
        onEvent,
    });
}

exports.downloadFileByID = downloadFileByID;
