import { ProjectFileHandler } from '$src/Project/utils/ProjectFileHandler';
import type { ProjectModel } from '$src/Project/utils/ProjectModel';
import { projectdb } from '$src/utils/Database';
import { fs } from '$src/utils/Path';

export class PortfolioModel {
    uid: string;
    path: string;
    focused_type: number;
    types = ["Concept", "Sandbox", "Release"];

    constructor(path: string) {
        this.uid = path;
        this.path = path;
        this.focused_type = -1;
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

    async loadProjects() {
        const t = this.getFocusedTypeString();
        const p = t == "all" ? this.path : fs.joinPath(this.path, t);
        const projects = await this.loadProjectsInPath(p).then(projects => {
            return projects
        }).catch(_ => {
            return [];
        });

        // Clear db
        await projectdb.clear_db();
        // Write Projects to db
        await projectdb.insert_projects(projects);
    }

    async getProjects(filter?: string): Promise<ProjectModel[]> {
        // ToDo create sqlite query builder
        const f: string = (() => {
            if (filter) {
                if (this.focused_type != -1)
                    return filter + ` AND type LIKE '${this.getFocusedTypeString()}'`;

                return filter;
            } else {
                if (this.focused_type != -1)
                    return `type LIKE '${this.getFocusedTypeString()}'`

                return null;
            }
        })();

        const where = f ? "SELECT * FROM project WHERE " + f : null;

        return await projectdb.get_projects(where);
    }

    getFocusedTypeString(): string {
        return this.focused_type == -1 ? "all" : this.types[this.focused_type];
    }

    toJSON(): any {
        const {focused_type, ...omitted} = this;
        return omitted;
    }
}