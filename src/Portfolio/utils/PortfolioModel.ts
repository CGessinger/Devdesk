import { ProjectFileHandler } from '$src/Project/utils/ProjectFileHandler';
import type { ProjectModel } from '$src/Project/utils/ProjectModel';
import { projectdb } from '$src/utils/Database';
import { fs } from '$src/utils/Path';

export class PortfolioModel {
    uid: string;
    path: string;
    focused_type: number;
    types = ["All", "Concept", "Sandbox", "Release"];

    constructor(path: string) {
        this.uid = path;
        this.path = path;
        this.focused_type = 0;
    }

    getFocusedTypeString(): string {
        return this.focused_type >= this.types.length ? "" : this.types[this.focused_type];
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

        const loadOrEmpty = async (path): Promise<ProjectModel[]> => {
            return await this.loadProjectsInPath(path).then(projects => {
                return projects
            }).catch(e => {
                console.log(e);
                return [];
            });
        }

        const projects = await (async (): Promise<ProjectModel[]> => {
            const type = this.getFocusedTypeString();
            let projects = [];
            if(type == "All") {
                for(const t of this.types) {
                    const path = fs.joinPath(this.path, t);
                    console.log(path);
                    projects = [...projects, ...await loadOrEmpty(path)];
                }
            } else {
                const path = fs.joinPath(this.path, type);
                projects = [...projects, ...await loadOrEmpty(path)];
            }
            return projects;
        })();
        
        console.log("loaded: ", projects);

        // Clear db
        await projectdb.clear_db();
        // Write Projects to db
        await projectdb.insert_projects(projects);
    }

    async getProjects(filter?: string): Promise<ProjectModel[]> {
        // ToDo create sqlite query builder
        const f: string = (() => {
            if (filter) {
                if (this.getFocusedTypeString() != "All")
                    return filter + ` AND type LIKE '${this.getFocusedTypeString()}'`;

                return filter;
            } else {
                if (this.getFocusedTypeString() != "All")
                    return `type LIKE '${this.getFocusedTypeString()}'`

                return null;
            }
        })();

        const where = f ? "SELECT * FROM project WHERE " + f : null;

        return await projectdb.get_projects(where);
    }

    toJSON(): any {
        const {focused_type, ...omitted} = this;
        return omitted;
    }
}