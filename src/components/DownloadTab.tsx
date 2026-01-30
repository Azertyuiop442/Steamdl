import { AddDownload } from "./AddDownload";
import { QueueList } from "./QueueList";

export function DownloadTab() {
  return (
    <div style={{
      display: "grid",
      gridTemplateColumns: "300px 1fr",
      gap: "1.5rem",
      flex: 1,
      minHeight: 0,
      height: "100%"
    }}>
      <aside style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
        <AddDownload />
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
  );
}
