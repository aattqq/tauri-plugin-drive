type DownloadStarted = {
    event: 'started';
    data: {
        downloadId: string;
        filename: string;
        contentLength: number;
    };
};
type DownloadProgress = {
    event: 'progress';
    data: {
        downloadId: string;
        chunkLength: number;
    };
};
type DownloadFinished = {
    event: 'finished';
    data: {
        downloadId: string;
    };
};
type DownloadEvent = DownloadStarted | DownloadProgress | DownloadFinished;
type OptionsType = {
    fileId: string;
    onDownloadEvent: (event: DownloadEvent) => void;
};
export declare function downloadFileByID({ fileId, onDownloadEvent }: OptionsType): Promise<ArrayBuffer>;
export {};
