import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
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

  const [copiedId, setCopiedId] = useState(null);

  const handleSelect = async (item) => {
    try {
      await invoke("select_item", { content: item.content });
      setCopiedId(item.id);
      setTimeout(() => setCopiedId(null), 2000);
    } catch (e) {
      console.error("Failed to select item:", e);
    }
  };

  const handleDelete = async (e, id) => {
    try {
      e.stopPropagation();
      await invoke("delete_item", { id });
      refreshHistory();
    } catch (e) {
      console.error("Failed to delete item:", e);
    }
  };

  const handleTogglePin = async (e, id) => {
    try {
      e.stopPropagation();
      await invoke("toggle_pin", { id });
      refreshHistory();
    } catch (e) {
      console.error("Failed to toggle pin:", e);
    }
  };

  return (
    <div className="container">
      <div className="header">PastePlus History</div>
      <div className="list">
        {items.length === 0 ? (
          <div className="empty">No history yet. Go copy something!</div>
        ) : (
          items.map((item) => (
            <div 
              key={item.id} 
              className={`item ${copiedId === item.id ? 'copied' : ''}`}
              onClick={() => handleSelect(item)}
            >
              <div className="content">{item.content}</div>
              {copiedId === item.id && <div className="copied-badge">Copied!</div>}
              <div className="actions">
                <button 
                  className={`action-btn pin-btn ${item.pinned ? 'active' : ''}`}
                  onClick={(e) => handleTogglePin(e, item.id)}
                  title={item.pinned ? "Unpin" : "Pin"}
                >
                  📌
                </button>
                <button 
                  className="action-btn delete-btn"
                  onClick={(e) => handleDelete(e, item.id)}
                  title="Delete"
                >
                  🗑️
                </button>
              </div>
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
          padding: 8px 16px;
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
        .item.copied {
          background: #313244;
          border-left: 3px solid #a6e3a1;
        }
        .copied-badge {
          font-size: 0.7rem;
          background: #a6e3a1;
          color: #11111b;
          padding: 2px 6px;
          border-radius: 4px;
          margin-right: 8px;
        }
        .content {
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
          max-width: 70%;
        }
        .actions {
          display: flex;
          gap: 8px;
          opacity: 0;
          transition: opacity 0.1s ease;
        }
        .item:hover .actions {
          opacity: 1;
        }
        .action-btn {
          background: transparent;
          border: none;
          cursor: pointer;
          font-size: 1rem;
          padding: 4px;
          border-radius: 4px;
          display: flex;
          align-items: center;
          justify-content: center;
          filter: grayscale(100%);
          transition: all 0.1s ease;
        }
        .action-btn:hover {
          background: #45475a;
          filter: grayscale(0%);
        }
        .pin-btn.active {
          filter: grayscale(0%);
        }
      `}</style>
    </div>
  );
}

export default App;