import { ProjectFileHandler } from '$src/Project/utils/ProjectFileHandler';
import type { ProjectModel } from '$src/Project/utils/ProjectModel';
import { projectdb } from '$src/utils/Database';
import { fs } from '$src/utils/Path';

export class PortfolioModel {
    uid: string;
    path: string;
    projects: ProjectModel[];
    focused_type: number;
    types = ["Concept", "Sandbox", "Release"];

    constructor(path: string) {
        this.uid = path;
        this.path = path;
        this.focused_type = -1;
        this.projects = [];
    }

    private async loadProjectsInPath(path: string): Promise<ProjectModel[]> {
        const folders: string[] = await fs.read_dir(path);
        const projects: ProjectModel[] = [];
        for (const f of folders) {
            await ProjectFileHandler.readFromFolder(fs.joinPath(path, f)).then(p =>{
                projects.push(p);
            }).catch(err => {
                console.log(err);
            });      
        }

        return projects;
    }

    async loadProjects(): Promise<ProjectModel[]> {
        const t = this.getFocusedTypeString();
        const p = t == "all" ? this.path : fs.joinPath(this.path, t);
        this.projects = await this.loadProjectsInPath(p).then(projects => {
            return projects
        }).catch(_ => {
            return [];
        });

        // Clear db
        projectdb.clear_db();
        // Write Projects to db
        projectdb.insert_projects(this.projects);

        console.log("Loaded projects: ", await projectdb.get_projects());

        return this.projects;
    }

    getFocusedTypeString(): string {
        return this.focused_type == -1 ? "all" : this.types[this.focused_type];
    }

    toJSON(): any {
        const {projects, focused_type, ...omitted} = this;
        return omitted;
    }
}