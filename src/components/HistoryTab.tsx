import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { HistoryItem } from "../types";

interface HistoryRowProps {
    item: HistoryItem;
    onDelete: (id: string) => void;
    onRetry: (steamId: string, name: string) => void;
}

function HistoryRow({ item, onDelete, onRetry }: HistoryRowProps) {
    const [exists, setExists] = useState(false);

    useEffect(() => {
        invoke<boolean>("check_path_exists", { path: item.install_path })
            .then(setExists)
            .catch(() => setExists(false));
    }, [item.install_path]);

    const handleOpen = () => {
        invoke("open_folder", { path: item.install_path });
    };

    const handleRetry = () => {
        onRetry(item.steam_id, item.name);
    };

    return (
        <tr style={{ borderBottom: "1px solid var(--border)" }}>
            <td style={{ padding: "12px 16px" }}>{item.name}</td>
            <td style={{ padding: "12px 16px" }}>{formatDate(item.timestamp)}</td>
            <td style={{ padding: "12px 16px", display: "flex", gap: "8px", justifyContent: "flex-end" }}>
                {exists ? (
                    <button
                        className="btn"
                        style={{ height: "2rem", padding: "0 0.5rem", background: "transparent", border: "1px solid var(--border)" }}
                        onClick={handleOpen}
                        title="Open Folder"
                    >
                        📂
                    </button>
                ) : (
                    <button
                        className="btn"
                        style={{ height: "2rem", padding: "0 0.5rem", background: "var(--primary)", color: "white", border: "none" }}
                        onClick={handleRetry}
                        title="Re-download"
                    >
                        🔄 Re-download
                    </button>
                )}
                <button
                    className="btn"
                    style={{ height: "2rem", padding: "0 0.5rem", color: "var(--destructive)", background: "transparent", border: "1px solid var(--border)" }}
                    onClick={() => onDelete(item.id)}
                    title="Delete"
                >
                    🗑️
                </button>
            </td>
        </tr>
    );
}

function formatDate(timestamp: number) {
    if (!timestamp || timestamp === 0) return "Unknown";
    try {
        return new Date(timestamp * 1000).toLocaleString();
    } catch {
        return "Invalid date";
    }
}

export function HistoryTab() {
    const [history, setHistory] = useState<HistoryItem[]>([]);

    const fetchHistory = async () => {
        try {
            const items = await invoke<HistoryItem[]>("get_history");
            setHistory(items);
        } catch (e) {
            console.error("Failed to fetch history:", e);
        }
    };

    useEffect(() => {
        fetchHistory();
    }, []);

    const handleDelete = async (id: string) => {
        try {
            await invoke("remove_history_item", { id });
            fetchHistory();
        } catch (e) {
            console.error("Failed to delete item:", e);
        }
    };

    const handleClear = async () => {
        try {
            await invoke("clear_history");
            fetchHistory();
        } catch (e) {
            console.error("Failed to clear history:", e);
        }
    };

    const handleRetry = async (steamId: string, name: string) => {
        try {
            await invoke("retry_download", { steamId, name });
        } catch (e) {
            console.error("Failed to retry download:", e);
        }
    };

    return (
        <div className="card" style={{ display: "flex", flexDirection: "column", height: "100%", overflow: "hidden" }}>
            <div className="card-header" style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
                <div>
                    <h2 className="card-title">History</h2>
                    <p className="card-description">View past downloads.</p>
                </div>
                <button
                    className="btn"
                    style={{ backgroundColor: "var(--destructive)", color: "white" }}
                    onClick={handleClear}
                >
                    Clear All
                </button>
            </div>
            <div style={{ flex: 1, overflowY: "auto" }}>
                {history.length === 0 ? (
                    <div style={{ padding: "2rem", textAlign: "center", color: "var(--muted-foreground)" }}>
                        No history available.
                    </div>
                ) : (
                    <table style={{ width: "100%", borderCollapse: "collapse", fontSize: "0.875rem" }}>
                        <thead style={{ background: "var(--card)", borderBottom: "1px solid var(--border)", position: "sticky", top: 0 }}>
                            <tr>
                                <th style={{ textAlign: "left", padding: "12px 16px", color: "var(--muted-foreground)" }}>Name</th>
                                <th style={{ textAlign: "left", padding: "12px 16px", color: "var(--muted-foreground)" }}>Date</th>
                                <th style={{ textAlign: "right", padding: "12px 16px", color: "var(--muted-foreground)" }}>Actions</th>
                            </tr>
                        </thead>
                        <tbody>
                            {history.map((item) => (
                                <HistoryRow
                                    key={item.id}
                                    item={item}
                                    onDelete={handleDelete}
                                    onRetry={handleRetry}
                                />
                            ))}
                        </tbody>
                    </table>
                )}
            </div>
        </div>
    );
}
