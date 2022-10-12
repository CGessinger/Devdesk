import { ProjectFileHandler } from '$src/Project/utils/ProjectFileHandler';
import type { ProjectModel } from '$src/Project/utils/ProjectModel';
import { projectdb } from '$utils/Database';
import { fs } from '$utils/Path';

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
                    projects = [...projects, ...await loadOrEmpty(path)];
                }
            } else {
                const path = fs.joinPath(this.path, type);
                projects = [...projects, ...await loadOrEmpty(path)];
            }
            return projects;
        })();

        // Clear db
        await projectdb.clear_db();
        // Write Projects to db
        await projectdb.insert_projects(projects);
    }

    async getProjects(filter?: projectdb.query): Promise<ProjectModel[]> {
        const query = filter ?? new projectdb.query();
        const focusedType = this.getFocusedTypeString();

        if (focusedType == "All")
            query.withTypes(this.types);
        else
            query.withTypes([focusedType]);

        return await projectdb.get_projects(query.build());
    }

    toJSON(): any {
        const {focused_type, ...omitted} = this;
        return omitted;
    }
}