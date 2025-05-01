import { invoke, Channel } from '@tauri-apps/api/core'

type DownloadStarted = {
  event: 'started'
  data: {
    downloadId: string
    filename: string
    contentLength: number
  }
}
type DownloadProgress = {
  event: 'progress'
  data: {
    downloadId: string
    chunkLength: number
  }
}
type DownloadFinished = {
  event: 'finished'
  data: {
    downloadId: string
  }
}
type DownloadEvent = DownloadStarted | DownloadProgress | DownloadFinished

type OptionsType = {
  fileId: string
  onDownloadEvent: (event: DownloadEvent) => void
}

export function downloadFileByID({ fileId, onDownloadEvent }: OptionsType): Promise<ArrayBuffer> {
  const onEvent = new Channel<DownloadEvent>()
  onEvent.onmessage = onDownloadEvent

  return invoke<ArrayBuffer>('plugin:drive|download_file', {
    fileId,
    onEvent,
  })
}
