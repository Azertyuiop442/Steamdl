import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { DownloadItem } from "../types";

function getLog(status: any) {
    if (typeof status === "object" && status.Failed) return status.Failed;
    return null;
}

function useIsMobile() {
    const [isMobile, setIsMobile] = useState(false);

    useEffect(() => {
        const checkMobile = () => setIsMobile(window.innerWidth < 600);
        checkMobile();
        window.addEventListener("resize", checkMobile);
        return () => window.removeEventListener("resize", checkMobile);
    }, []);

    return isMobile;
}

export function QueueList() {
    const [queue, setQueue] = useState<DownloadItem[]>([]);
    const [expandedId, setExpandedId] = useState<string | null>(null);
    const isMobile = useIsMobile();

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

    const gridColumns = isMobile ? "1fr auto" : "minmax(0, 1fr) auto";

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
                        gridTemplateColumns: gridColumns,
                        padding: isMobile ? "8px 12px" : "12px 16px",
                        gap: isMobile ? "8px" : "16px",
                        borderBottom: "1px solid var(--border)",
                        background: "var(--card)",
                        position: "sticky",
                        top: 0,
                        zIndex: 10,
                        fontWeight: 500,
                        color: "var(--muted-foreground)",
                        fontSize: isMobile ? "0.75rem" : "0.875rem"
                    }}>
                        <div>Name</div>
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
                                        gridTemplateColumns: gridColumns,
                                        padding: isMobile ? "8px 12px" : "12px 16px",
                                        gap: isMobile ? "8px" : "16px",
                                        alignItems: "center",
                                        cursor: log ? "pointer" : "default",
                                        background: isExpanded ? "var(--accent)" : "transparent",
                                        transition: "background 0.2s"
                                    }}
                                >
                                    <div 
                                        style={{ 
                                            fontWeight: 500, 
                                            fontSize: isMobile ? "0.8rem" : "0.875rem",
                                            overflow: "hidden",
                                            textOverflow: "ellipsis",
                                            whiteSpace: "nowrap",
                                            minWidth: 0
                                        }}
                                        title={item.name}
                                    >
                                        {item.name}
                                    </div>
                                    <div style={{ textAlign: "right" }}>{renderStatus(item.status, item.install_path || "", isMobile)}</div>
                                </div>

                                {isExpanded && log && (
                                    <div style={{
                                        padding: isMobile ? "8px 12px" : "12px 16px",
                                        background: "var(--flexoki-black)",
                                        borderTop: "1px solid var(--border)",
                                        fontSize: isMobile ? "0.7rem" : "0.8rem",
                                        fontFamily: "monospace",
                                        color: "var(--flexoki-red-primary)",
                                        overflowWrap: "break-word"
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

function renderStatus(status: any, path: string, isMobile: boolean) {
    const baseStyle = {
        padding: isMobile ? "2px 6px" : "2px 8px",
        borderRadius: "12px",
        fontSize: isMobile ? "0.7rem" : "0.75rem"
    };

    if (status === "Pending")
        return <span style={{ ...baseStyle, color: "var(--flexoki-yellow)", background: "rgba(173, 131, 1, 0.2)", border: "1px solid rgba(173, 131, 1, 0.3)" }}>Pending</span>;
    
    if (typeof status === "object" && "Downloading" in status) {
        return <span style={{ ...baseStyle, color: "var(--flexoki-orange)", background: "rgba(188, 82, 21, 0.2)", border: "1px solid rgba(188, 82, 21, 0.3)" }}>Downloading</span>;
    }
    
    if (status === "Completed")
        return (
            <div style={{ display: "flex", alignItems: "center", justifyContent: "flex-end", gap: isMobile ? "4px" : "8px" }}>
                <span style={{ ...baseStyle, color: "var(--flexoki-green)", background: "rgba(102, 128, 11, 0.2)", border: "1px solid rgba(102, 128, 11, 0.3)" }}>{isMobile ? "Done" : "Completed"}</span>
                <button
                    onClick={(e) => { e.stopPropagation(); if (path) invoke("open_folder", { path }); }}
                    style={{ background: "transparent", border: "none", cursor: "pointer", fontSize: isMobile ? "0.9rem" : "1rem", color: "var(--muted-foreground)", padding: 0, display: "flex" }}
                    title="Open Folder"
                >
                    📂
                </button>
            </div>
        );
    
    if (typeof status === "object" && status.Failed)
        return <span style={{ ...baseStyle, cursor: "pointer", color: "var(--flexoki-red-primary)", background: "rgba(175, 48, 41, 0.2)", border: "1px solid rgba(175, 48, 41, 0.3)" }}>
            Failed ▾
        </span>;
    
    return <span>{JSON.stringify(status)}</span>;
}
