import "./App.css";
import { AddDownload } from "./components/AddDownload";
import { QueueList } from "./components/QueueList";

function App() {
  return (
    <div className="container">
      <header style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
        <div>
          <h1 style={{ margin: 0, fontSize: "1.5rem", fontWeight: "700", letterSpacing: "-0.025em" }}>SteamDL</h1>
          <p style={{ margin: 0, color: "var(--muted-foreground)", fontSize: "0.875rem" }}>
            Rust-powered SteamCMD GUI
          </p>
        </div>
        <div style={{ display: "flex", gap: "10px" }}>
          {/* Status indicators could go here */}
          <span style={{ fontSize: "0.75rem", padding: "4px 8px", background: "var(--muted)", borderRadius: "var(--radius)", color: "var(--muted-foreground)" }}>
            v0.1.0
          </span>
        </div>
      </header>

      <div style={{
        display: "grid",
        gridTemplateColumns: "300px 1fr",
        gap: "1.5rem",
        flex: 1,
        minHeight: 0
      }}>
        <aside style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
          <AddDownload />
          {/* Future: Settings, History, etc. accessible here */}
        </aside>

        <main className="card" style={{ display: "flex", flexDirection: "column", overflow: "hidden" }}>
          <div className="card-header">
            <h2 className="card-title">Downloads</h2>
            <p className="card-description">Manage and monitor your active content queue.</p>
          </div>
          <div style={{ flex: 1, overflow: "hidden", display: "flex", flexDirection: "column" }}>
            <QueueList />
          </div>
        </main>
      </div>
    </div>
  );
}

export default App;
