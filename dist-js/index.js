import { Channel, invoke } from '@tauri-apps/api/core';

function downloadFileByID({ fileId, onDownloadEvent }) {
    const onEvent = new Channel();
    onEvent.onmessage = onDownloadEvent;
    return invoke('plugin:drive|download_file', {
        fileId,
        onEvent,
    });
}

export { downloadFileByID };
