export type DownloadStatus =
    | "Pending"
    | "Downloading"
    | "Completed"
    | { Failed: string };

export interface DownloadItem {
    id: string;
    steam_id: string;
    name: string;
    status: DownloadStatus;
    created_at: number;
}
