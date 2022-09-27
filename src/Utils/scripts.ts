import { invoke } from "@tauri-apps/api/tauri";

export namespace terminal {
    export function terminal_here(path: string) {
        invoke("terminal_at", { path });
    }

    export function vscode_here(path: string) {
        invoke("vscode_at", { path });
    }
}