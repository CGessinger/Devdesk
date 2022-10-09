import { ProjectModel } from "./ProjectModel";
import { fs } from "$utils/Path";
import { terminal } from "$utils/Scripts";
import { Ok, Err } from "$utils/Result";

export class ProjectFileHandler {
    p: ProjectModel;
    constructor(p_: ProjectModel) {
        this.p = p_;
    }

    private async createFolder(path_: string): Promise<this> {
        try {
            let exists = await fs.folder_exists(path_);
            if (!exists) {
                fs.create_folder(path_).catch(err => {
                    return Err(err);
                });
                return Ok(this).asResolved();
            }

            return Err("Folder already exists").asRejected();
        }
        catch (err) {
            return Err(err).asRejected();
        }
    }

    async createProjectFolder(): Promise<this> {
        const path = this.p.path;
        if (path == "") return Err("No path provided").asRejected();

        return this.createFolder(path);
    }

    async createConfigFolder(): Promise<this> {
        if (this.p.path == "") return Err("No path provided").asRejected();

        const path = fs.joinPath(this.p.path, ".ppa");
        return this.createFolder(path);
    }

    async writeToConfig(): Promise<this> {
        if (this.p.path == "") return Err("No path provided").asRejected();

        const path = this.p.config_path();
        const config = {
            name: this.p.name,
            description: this.p.description
        };

        return fs.write_to_file(path, JSON.stringify(config)).then(() => {
            return Ok(this).asResolved();
        }).catch(err => {
            return Err(err).asRejected();
        });
    }

    async cloneGit(): Promise<void> {
        if (!fs.folder_exists(this.p.path)) {
            return Err("Target Project does not exist. Cannot clone git repository").asRejected();
        } else if (this.p.git.url == "") {
            return Err("No git repository provided").asRejected();
        } else if (this.p.git.branch == "") {
            return Err("No git branch provided").asRejected();
        }

        terminal.clone_git_repo(this.p.git, this.p.path);
    }

    static async readFromFolder(path_: string): Promise<ProjectModel> {
        const exists: boolean = await fs.folder_exists(path_);
        if (!exists) {
            return Err("Folder does not exist: " + path_).asRejected();
        }

        const config_path = fs.joinPath(path_, ".ppa");
        let project;
        await this.readFromConfig(config_path).then(p => {
            project = p;
        }).catch(_ => {
            project = new ProjectModel()
            project.name = fs.nameFromPath(path_);
        }).finally(() => {
            project.path = path_;
            project.type = fs.typeFromPath(path_);
        });

        return Ok(project).asResolved();
    }

    static async readFromConfig(path_: string): Promise<ProjectModel> {
        const file_path = fs.joinPath(path_, "config.json");
        try {
            const exists: boolean = await fs.file_exists(file_path);
            if (!exists) {
                return Err("Config does not exist").asRejected();
            }

            const content: string = await fs.read_file(file_path);
            return Ok(Object.assign(new ProjectModel(), JSON.parse(content))).asResolved();
        }
        catch (err) {
            return Err(err).asRejected();
        }
    }
}