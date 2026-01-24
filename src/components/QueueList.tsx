import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { DownloadItem } from "../types";

function getLog(status: any) {
    if (typeof status === "object" && status.Failed) return status.Failed;
    return null;
}

export function QueueList() {
    const [queue, setQueue] = useState<DownloadItem[]>([]);
    const [expandedId, setExpandedId] = useState<string | null>(null);

    const fetchQueue = async () => {
        try {
            const items = await invoke<DownloadItem[]>("get_queue");
            setQueue(items);
        } catch (e) {
            console.error(e);
        }
    };

    useEffect(() => {
        fetchQueue();
        const unlisten = listen("queue-update", () => fetchQueue());
        return () => { unlisten.then(f => f()); };
    }, []);

    const toggleLog = (id: string) => {
        setExpandedId(expandedId === id ? null : id);
    }

    return (
        <div style={{ flex: 1, overflowY: "auto" }}>
            {queue.length === 0 ? (
                <div style={{
                    display: "flex",
                    flexDirection: "column",
                    alignItems: "center",
                    justifyContent: "center",
                    height: "100%",
                    color: "var(--muted-foreground)",
                    gap: "1rem"
                }}>
                    <div style={{ fontSize: "2rem", opacity: 0.2 }}>📥</div>
                    <p>No downloads in queue.</p>
                </div>
            ) : (
                <div style={{ display: "flex", flexDirection: "column" }}>
                    <div style={{
                        display: "grid",
                        gridTemplateColumns: "2fr 1fr 100px",
                        padding: "12px 16px",
                        borderBottom: "1px solid var(--border)",
                        background: "var(--card)",
                        position: "sticky",
                        top: 0,
                        zIndex: 10,
                        fontWeight: 500,
                        color: "var(--muted-foreground)",
                        fontSize: "0.875rem"
                    }}>
                        <div>Name</div>
                        <div>ID</div>
                        <div style={{ textAlign: "right" }}>Status</div>
                    </div>

                    {queue.map(item => {
                        const log = getLog(item.status);
                        const isExpanded = expandedId === item.id;

                        return (
                            <div key={item.id} style={{ borderBottom: "1px solid var(--border)" }}>
                                <div
                                    onClick={() => log && toggleLog(item.id)}
                                    style={{
                                        display: "grid",
                                        gridTemplateColumns: "2fr 1fr 100px",
                                        padding: "12px 16px",
                                        alignItems: "center",
                                        cursor: log ? "pointer" : "default",
                                        background: isExpanded ? "var(--accent)" : "transparent",
                                        transition: "background 0.2s"
                                    }}
                                >
                                    <div style={{ fontWeight: 500, fontSize: "0.875rem" }}>{item.name}</div>
                                    <div style={{ color: "var(--muted-foreground)", fontFamily: "monospace", fontSize: "0.80rem" }}>{item.steam_id}</div>
                                    <div style={{ textAlign: "right" }}>{renderStatus(item.status, item.steam_id)}</div>
                                </div>

                                {isExpanded && log && (
                                    <div style={{
                                        padding: "12px 16px",
                                        background: "var(--flexoki-black)",
                                        borderTop: "1px solid var(--border)",
                                        fontSize: "0.8rem",
                                        fontFamily: "monospace",
                                        color: "var(--flexoki-red-primary)"
                                    }}>
                                        <div style={{ marginBottom: "4px", fontWeight: "bold", textTransform: "uppercase", fontSize: "0.7rem", opacity: 0.7 }}>Error Log</div>
                                        {log}
                                    </div>
                                )}
                            </div>
                        );
                    })}
                </div>
            )}
        </div>
    );
}

function renderStatus(status: any, steamId: string) {
    if (status === "Pending")
        return <span style={{ color: "var(--flexoki-yellow)", background: "rgba(173, 131, 1, 0.2)", padding: "2px 8px", borderRadius: "12px", fontSize: "0.75rem", border: "1px solid rgba(173, 131, 1, 0.3)" }}>Pending</span>;
    if (status === "Downloading")
        return <span style={{ color: "var(--flexoki-orange)", background: "rgba(188, 82, 21, 0.2)", padding: "2px 8px", borderRadius: "12px", fontSize: "0.75rem", border: "1px solid rgba(188, 82, 21, 0.3)" }}>Downloading</span>;
    if (status === "Completed")
        return (
            <div style={{ display: "flex", alignItems: "center", justifyContent: "flex-end", gap: "8px" }}>
                <span style={{ color: "var(--flexoki-green)", background: "rgba(102, 128, 11, 0.2)", padding: "2px 8px", borderRadius: "12px", fontSize: "0.75rem", border: "1px solid rgba(102, 128, 11, 0.3)" }}>Completed</span>
                <button
                    onClick={(e) => { e.stopPropagation(); invoke("open_folder", { id: steamId }); }}
                    style={{ background: "transparent", border: "none", cursor: "pointer", fontSize: "1rem", color: "var(--muted-foreground)", padding: 0, display: "flex" }}
                    title="Open Folder"
                >
                    📂
                </button>
            </div>
        );
    if (typeof status === "object" && status.Failed)
        return <span style={{ cursor: "pointer", color: "var(--flexoki-red-primary)", background: "rgba(175, 48, 41, 0.2)", padding: "2px 8px", borderRadius: "12px", fontSize: "0.75rem", border: "1px solid rgba(175, 48, 41, 0.3)" }}>
            Failed ▾
        </span>;
    return <span>{JSON.stringify(status)}</span>;
}

