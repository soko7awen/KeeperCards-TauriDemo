import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [statusMsg] = useState("");

  async function test() {
    await invoke("test_command", { port_name: '/dev/ttyACM0', color: '#F00' });
  }

  return (
    <main className="container">
      <h1>Initial Test</h1>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          test();
        }}
      >
        <button type="submit">Test</button>
      </form>
      <p>{statusMsg}</p>
    </main>
  );
}

export default App;
