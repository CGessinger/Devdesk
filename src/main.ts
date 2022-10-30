import '$bootstrap/css/bootstrap.min.css'
import '$bootstrap_icons/bootstrap-icons.css'
import '$src/app.css'
import App from './App.svelte'
import { window as tauriWindow } from "@tauri-apps/api";

// CSS selector
const dragbar = document.querySelector(".dragbar");
dragbar.addEventListener("mousedown", async e => {
  console.log("mousedown");
    await tauriWindow.appWindow.startDragging();
});

dragbar.addEventListener("dblclick", async e => {
  console.log("dblclick");
  await tauriWindow.appWindow.toggleMaximize();
});

const app = new App({
  target: document.getElementById('app')
})

export default app
