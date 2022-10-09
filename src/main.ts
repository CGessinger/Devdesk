import '$bootstrap/css/bootstrap.min.css'
import '$bootstrap_icons/bootstrap-icons.css'
import * as bootstrap from "bootstrap"
import * as popper from "@popperjs/core"
import '$src/app.css'
import App from './App.svelte'

const app = new App({
  target: document.getElementById('app')
})

export default app
