export type DownloadStatus =
    | "Pending"
    | { Downloading: { progress: number } }
    | "Completed"
    | { Failed: string };

export interface DownloadItem {
    id: string;
    steam_id: string;
    name: string;
    status: DownloadStatus;
    install_path?: string;
    created_at: number;
}

export interface HistoryItem {
    id: string;
    steam_id: string;
    name: string;
    install_path: string;
    timestamp: number;
}
