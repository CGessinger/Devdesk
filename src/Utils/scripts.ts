import { invoke } from "@tauri-apps/api/tauri";

export namespace terminal {
    export function terminal_here(path: string, commandLine: string): Promise<string[]> {
        return invoke("terminal_at", { path, commandLine });
    }

    export function editorHere(path: string, commandLine: string): Promise<string[]> {
        return invoke("editor_at", { path, commandLine });
    }

    export function make_here(path: string): Promise<string[]> {
        return invoke("run_make", { path });
    }

    export function clone_git_repo(git: {url: string, branch: string}, path: string) {
        invoke("git_clone", { url: git.url, branch: git.branch, path });
    }
}
