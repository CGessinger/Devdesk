import { invoke, path as path_api } from '@tauri-apps/api';

export namespace fs {
    export const seperator = path_api.sep;

    export function joinPath(...paths: string[]): string {
        return paths.join(seperator);
    }

    export function splitPath(path: string): string[] {
        return path.split(seperator);
    }

    export function typeFromPath(path: string): string {
        return path.split(seperator).at(-2);
    }

    export function nameFromPath(path: string): string {
        return path.split(seperator).at(-1);
    }

    export async function appDir(): Promise<string> {
        return path_api.appDir();
    }

    export async function documentDir(): Promise<string> {
        return path_api.documentDir();
    }

    export async function create_folder(path: string): Promise<void> {
        return invoke("create_folder", { path: path });
    }

    export async function read_dir(path: string): Promise<string[]> {
        return invoke("read_dir", { path: path });
    }
    
    export async function read_file(path: string): Promise<string> {
        return invoke("read_file", { path: path });
    }

    export async function is_dir(path: string): Promise<string> {
        return invoke("is_dir", { path: path });
    }

    export async function write_to_file(path: string, content: string): Promise<void> {
        return invoke("write_to_file", { path: path, content: content });
    }

    export async function folder_exists(path: string): Promise<boolean> {
        return invoke("folder_exists", { path: path });
    }

    export async function file_exists(path: string): Promise<boolean> {
        return invoke("file_exists", { path: path });
    }

    export async function makefile_exists(path: string): Promise<boolean> {
        return invoke("makefile_exists", { path: path });
    }

    export async function write_image(path: string, data: string): Promise<void> {
        return invoke("write_image", { path: path, data: data });
    }
}