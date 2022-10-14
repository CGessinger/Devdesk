import { invoke } from "@tauri-apps/api/tauri";

export namespace terminal {
    export function terminal_here(path: string) {
        invoke("terminal_at", { path });
    }

    export function vscode_here(path: string) {
        invoke("vscode_at", { path });
    }

    export function make_here(path: string) {
        invoke("run_make", { path });
    }

    export function clone_git_repo(git: {url: string, branch: string}, path: string) {
        invoke("git_clone", { url: git.url, branch: git.branch, path });
    }
}