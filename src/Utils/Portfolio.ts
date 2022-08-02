import { invoke } from '@tauri-apps/api/tauri';

export class Portfolio {
    uid: string;
    path: string;
    projects: string[];
    types = ["Concept", "Sandbox", "Release"];

    constructor(path: string) {
        this.uid = path;
        this.path = path;
        this.projects = ["loading..."];
    }

    async load_projects(): Promise<void> {
        let projects: string[] = await invoke("read_dir", {path: this.path});
        this.projects = projects;
    }

    toJSON(): any {
        const {projects, ...omitted} = this;
        return omitted;
    }
}