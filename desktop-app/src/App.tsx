import React from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Game from "./Game";

function App() {
  const openedRef = React.useRef(false);
  const [selectedBook, setSelectedBook] = React.useState(false);
  
  React.useEffect(() => {
    async function loadBook() {
      if (openedRef.current) {
        return;
      }
      openedRef.current = true;
      const selected = await open({ multiple: false });
      if (selected) {
        await invoke("open_book", { bookPath: selected });
        setSelectedBook(true);
      }
    }

    loadBook();
  }, []);

  return <main className="w-full h-screen bg-slate-900">
    {selectedBook && <Game/>}
  </main>;
}

export default App;
