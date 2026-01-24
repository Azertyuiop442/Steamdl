import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export function AddDownload() {
    const [input, setInput] = useState("");
    const [name, setName] = useState("");
    const [error, setError] = useState<string | null>(null);

    const handleAdd = async () => {
        if (!input) return;
        setError(null);

        try {
            const displayName = name || `App ${input}`;
            await invoke("add_download", { steamId: input, name: displayName });
            setInput("");
            setName("");
        } catch (e) {
            console.error("Failed to add download:", e);
            setError(String(e));
        }
    };

    return (
        <div className="card">
            <div className="card-header">
                <h3 className="card-title">Add Content</h3>
            </div>
            <div className="card-content" style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
                <div style={{ display: "flex", flexDirection: "column", gap: "0.5rem" }}>
                    <label style={{ fontSize: "0.875rem", fontWeight: 500 }}>Steam AppID / URL</label>
                    <input
                        className="input"
                        type="text"
                        placeholder="e.g. 740"
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                    />
                </div>

                <div style={{ display: "flex", flexDirection: "column", gap: "0.5rem" }}>
                    <label style={{ fontSize: "0.875rem", fontWeight: 500 }}>Display Name (Optional)</label>
                    <input
                        className="input"
                        type="text"
                        placeholder="e.g. CS:GO Server"
                        value={name}
                        onChange={(e) => setName(e.target.value)}
                    />
                </div>

                <button className="btn btn-primary" onClick={handleAdd} style={{ marginTop: "0.5rem", width: "100%" }}>
                    Add to Queue
                </button>
                {error && (
                    <div style={{ color: "var(--destructive)", fontSize: "0.8rem", marginTop: "0.5rem" }}>
                        Error: {error}
                    </div>
                )}
            </div>
        </div>
    );
}
