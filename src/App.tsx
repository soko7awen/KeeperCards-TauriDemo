import { listen } from '@tauri-apps/api/event';
import { useState, useEffect } from 'react';
import Color from "color";
import "./App.css";

function App() {
  let [cardColor, setCardColor] = useState<string>("#000");

  useEffect(() => {
    const promise = listen<string>('color-changed', (event) => {
      console.log('got payload:', event.payload);
      let unamplifiedColor = Color(event.payload);
      let amplifiedColor = unamplifiedColor.lighten(10);
      setCardColor(amplifiedColor.string());
    });
    return () => {
      promise.then(unlisten => unlisten());
    };
  }, []);


  return (
    <main className="container">
      <h1>KeeprCards Showcase</h1>
      <div
        id="indicator"
        style={{ backgroundColor: cardColor }}
      />
    </main>
  );
}
export default App;