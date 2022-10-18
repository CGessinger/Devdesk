import { invoke } from "@tauri-apps/api/tauri";
import { type } from '@tauri-apps/api/os';

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

export namespace commandDefaults {
    export async function defaultEditorCommand(): Promise<string> {
        const osType = await type();
        switch (osType) {
            case "Darwin": 
                return "open -a Visual Studio Code .";
            case "Windows_NT": 
                return "cmd /C start code .";
            case "Linux":
                return "code ."
            default:
                return ""
        }
    }

    export async function defaultTerminalCommand(): Promise<string> {
        const osType = await type();
        switch (osType) {
            case "Darwin": 
                return "open -a Terminal .";
            case "Windows_NT": 
                return "cmd /C start cmd /K cd .";
            case "Linux":
                return "gnome-terminal --working-directory ."
            default:
                return ""
        }
    }
}