import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [items, setItems] = useState([]);

  const refreshHistory = async () => {
    try {
      const data = await invoke("get_history");
      setItems(data);
    } catch (e) {
      console.error("Failed to fetch history:", e);
    }
  };

  useEffect(() => {
    refreshHistory();
    const unlisten = listen("db-updated", () => refreshHistory());
    return () => { unlisten.then(f => f()); };
  }, []);

  const handleSelect = async (content) => {
    await invoke("select_item", { content });
  };

  return (
    <div className="container">
      <div className="header">ClipFlow History</div>
      <div className="list">
        {items.length === 0 ? (
          <div className="empty">No history yet. Go copy something!</div>
        ) : (
          items.map((item) => (
            <div 
              key={item.id} 
              className="item" 
              onClick={() => handleSelect(item.content)}
            >
              <div className="content">{item.content}</div>
              {item.pinned && <span className="pin">📌</span>}
            </div>
          ))
        )}
      </div>
      <style jsx="true">{`
        .container {
          background: #1e1e2e;
          color: #cdd6f4;
          height: 100vh;
          font-family: sans-serif;
          display: flex;
          flex-direction: column;
          border: 1px solid #45475a;
          box-sizing: border-box;
        }
        .header {
          padding: 12px;
          font-size: 0.75rem;
          color: #6c7086;
          text-transform: uppercase;
          letter-spacing: 0.05em;
          border-bottom: 1px solid #313244;
          background: #181825;
        }
        .list {
          overflow-y: auto;
          flex: 1;
        }
        .empty {
          padding: 20px;
          text-align: center;
          color: #585b70;
          font-size: 0.9rem;
        }
        .item {
          padding: 12px 16px;
          border-bottom: 1px solid #313244;
          cursor: pointer;
          display: flex;
          justify-content: space-between;
          align-items: center;
          font-size: 0.9rem;
          transition: background 0.1s ease;
        }
        .item:hover {
          background: #313244;
        }
        .content {
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
          max-width: 90%;
        }
        .pin { font-size: 0.8rem; }
      `}</style>
    </div>
  );
}

export default App;