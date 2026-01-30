import { useState } from "react";
import "./App.css";
import { DownloadTab } from "./components/DownloadTab";
import { HistoryTab } from "./components/HistoryTab";

type Tab = "download" | "history";

function App() {
  const [activeTab, setActiveTab] = useState<Tab>("download");

  return (
    <div className="container">
      <header style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
        <div>
          <h1 style={{ margin: 0, fontSize: "1.5rem", fontWeight: "700", letterSpacing: "-0.025em" }}>SteamDL</h1>
          <p style={{ margin: 0, color: "var(--muted-foreground)", fontSize: "0.875rem" }}>
            Rust-powered SteamCMD GUI
          </p>
        </div>
        <div style={{ display: "flex", gap: "10px", alignItems: "center" }}>
          {/* Navigation Tabs */}
          <div className="tabs-container" style={{ display: "flex", background: "var(--card)", padding: "4px", borderRadius: "var(--radius)", border: "1px solid var(--border)" }}>
             <button
                className={`btn tab-btn`}
                style={{ 
                    height: "2rem", 
                    backgroundColor: activeTab === "download" ? "var(--primary)" : "transparent",
                    color: activeTab === "download" ? "var(--primary-foreground)" : "var(--muted-foreground)" 
                }}
                onClick={() => setActiveTab("download")}
             >
               Downloads
             </button>
             <button
                className={`btn tab-btn`}
                style={{ 
                    height: "2rem", 
                    backgroundColor: activeTab === "history" ? "var(--primary)" : "transparent",
                    color: activeTab === "history" ? "var(--primary-foreground)" : "var(--muted-foreground)" 
                }}
                onClick={() => setActiveTab("history")}
             >
               History
             </button>
           </div>

           <span className="version-badge" style={{ fontSize: "0.75rem", padding: "4px 8px", background: "var(--muted)", borderRadius: "var(--radius)", color: "var(--muted-foreground)" }}>
             v0.1.1
           </span>
        </div>
      </header>

      <div style={{ flex: 1, minHeight: 0, overflow: "hidden" }}>
        {activeTab === "download" && <DownloadTab />}
        {activeTab === "history" && <HistoryTab />}
      </div>
    </div>
  );
}

export default App;
