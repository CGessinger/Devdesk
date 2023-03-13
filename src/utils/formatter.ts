import { sep } from '@tauri-apps/api/path'

export class formatter {
    static formatName(name: string): string {
        name = name.split("-").join(" ");
        name = name.split("_").join(" ");
        return name;
    }

    static formatScriptName(name: string): string {
        let lastPointIndex = name.lastIndexOf(".");
        name = name.slice(0, lastPointIndex);
        return this.formatName(name);
    }

    static breadcrumpsFrom(path: string, fallbackPath: string = ""): string[] {
        if (path) {
            return path.split(sep);
        }
        return fallbackPath.split(sep);
    }

    static fileNameFrom(path: string): string {
        return path.split(sep).at(-1);
    }
}