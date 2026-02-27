import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from 'react';
import './App.css';

type MouseInfo = {
    speed: number;
    // accel_enabled: boolean;
}

function App() {
    const [mouse, setMouse] = useState<MouseInfo | null>(null);

    async function fetchMouse() {
        setMouse(await invoke("get_mouse_info"));
    }

    async function updateSpeed(newSpeed: number) {
        await invoke<boolean>("set_mouse_speed", {speed: newSpeed});
        fetchMouse();
    }

    async function save_mouse_state() {
        try {
            const r = await invoke("save_mouse_state");
            fetchMouse();
            console.log("SAVE RESULT:", r);
        } catch (e) {
            console.error("INVOKE ERROR:", e);
        }
    }

    async function apply_mouse_state() {
        await invoke<boolean>("apply_mouse_state");
        fetchMouse();
    }

    useEffect(() => {
        fetchMouse();
    }, []);
    
    return (
        <div className="App">
            <h1>Current Sensitivity: {mouse?.speed}</h1>
            {/* <h1>Acceleration: {mouse?.accel_enabled ? "ON" : "OFF"}</h1> */}
            <h1>Device:</h1>
            <button onClick={save_mouse_state}>Save State</button>
            <button onClick={apply_mouse_state}>Apply State</button>
            <button onClick={fetchMouse}>Refresh</button>
            <input type="range" min="1" max="20" value={mouse?.speed ?? 6} onChange={(e) => updateSpeed(Number(e.target.value))}/>
            {/* <input type="checkbox" checked={mouse?.accel_enabled ?? false} onChange={(e) => } */}
        </div>
    );
}

export default App;