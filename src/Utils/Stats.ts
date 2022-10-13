import { invoke } from "@tauri-apps/api/tauri";

export namespace stats {
    export interface LanguageStats {
        total: number,
        languages: object
    }

    export function get_language_stats(path: string): Promise<LanguageStats> {
        return invoke("read_language_stats", { path });
    }
}