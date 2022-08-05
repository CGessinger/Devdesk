import { path as path_api } from '@tauri-apps/api';
export const seperator = path_api.sep;

export function joinPath(...paths: string[]): string {
    return paths.join(seperator);
}

export function typeFromPath(path: string): string {
    return path.split(seperator).at(-2);
}

export function nameFromPath(path: string): string {
    return path.split(seperator).at(-1);
}
