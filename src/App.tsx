import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ChromePicker } from "react-color";
import "./App.css";

function App() {
  async function test(newColor: string) {
    await invoke("test_command", { port_name: "/dev/ttyACM0", color: newColor });
  }

  return (
    <main className="container">
      <h1>Initial Test</h1>
      <ChromePicker
        disableAlpha
        color={"#F00"}
        onChange={(newColor) => {
          test(newColor.hex);
        }} 
      />
    </main>
  );
}

export default App;
