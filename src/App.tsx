import { listen } from '@tauri-apps/api/event';
import { useEffect } from 'react';
import "./App.css";

function App() {
  useEffect(() => {
    const promise = listen('color-changed', (payload) => {
      console.log('got payload:', payload);
    });
    return () => {
      promise.then(unlisten => unlisten());
    };
  }, []);
  return (
    <main className="container">
      <h1>Initial Test</h1>
      <div id="indicator"></div>
    </main>
  );
}

export default App;