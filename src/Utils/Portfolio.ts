import { invoke } from '@tauri-apps/api/tauri';
import { joinPath } from './Path';
import { Project } from './Project';

export class Portfolio {
    uid: string;
    path: string;
    projects: Project[];
    focused_type: number;
    types = ["Concept", "Sandbox", "Release"];

    constructor(path: string) {
        this.uid = path;
        this.path = path;
        this.focused_type = -1;
        this.projects = [];
    }

    async load_projects_image(): Promise<void> {
        for (let p of this.projects) {
            await p.load_image();
        }
    }

    private async load_projects_from(path_: string): Promise<Project[]> {
        let folders: string[] = await invoke("read_dir", {path: path_});
        let projects: Project[] = [];
        for (let f of folders) {
            await Project.Folder.readFromFolder(joinPath(path_, f)).then(p =>{
                projects.push(p);
            }).catch(err => {
                console.log(err);
            });      
        }

        return projects;
    }

    async load_projects(): Promise<void> {
        this.projects = await this.load_projects_from(this.path);
    }

    async load_projects_from_type(): Promise<void> {
        const t = this.get_focused_type();
        const p = t == "all" ? this.path : joinPath(this.path, t);
        this.projects = await this.load_projects_from(p).then(projects => {
            return projects
        }).catch(_ => {
            return [];
        });
    }

    get_focused_type(): string {
        return this.focused_type == -1 ? "all" : this.types[this.focused_type];
    }

    toJSON(): any {
        const {projects, focused_type, ...omitted} = this;
        return omitted;
    }
}