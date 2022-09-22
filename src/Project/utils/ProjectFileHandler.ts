import { ProjectModel } from "./ProjectModel";
import { invoke } from "@tauri-apps/api";
import { joinPath, nameFromPath, typeFromPath } from "$utils/Path";
import { Ok, Err } from "$utils/Result";

export class ProjectFileHandler {
    p: ProjectModel;
    constructor(p_: ProjectModel) {
        this.p = p_;
    }

    private async createFolder(path_: string): Promise<this> {
        try {
            let exists = await invoke("folder_exists", { path: path_ });
            if (!exists) {
                invoke("create_folder", { path: path_ }).catch(err => {
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

        const path = joinPath(this.p.path, ".ppa");
        return this.createFolder(path);
    }

    async writeToConfig(): Promise<this> {
        if (this.p.path == "") return Err("No path provided").asRejected();

        const path = this.p.config_path();
        const config = {
            name: this.p.name,
            description: this.p.description,
            tags: this.p.tags,
            technologies: this.p.technologies,
        };

        if (this.p.image != "") {
            const img_path = joinPath(this.p.config_folder_path(), "icon.png");
            invoke("write_image", { path: img_path, data: this.p.image });
        }

        return invoke("write_to_file", { path: path, content: JSON.stringify(config) }).then(() => {
            return Ok(this).asResolved();
        }).catch(err => {
            return Err(err).asRejected();
        });
    }

    static async readFromFolder(path_: string): Promise<ProjectModel> {
        const exists: boolean = await invoke("folder_exists", { path: path_ });
        if (!exists) {
            return Err("Folder does not exist: " + path_).asRejected();
        }

        const config_path = joinPath(path_, ".ppa");
        let project;
        await this.readFromConfig(config_path).then(p => {
            project = p;
        }).catch(_ => {
            project = new ProjectModel()
            project.name = nameFromPath(path_);
        }).finally(() => {
            project.path = path_;
            project.type = typeFromPath(path_);
        });

        return Ok(project).asResolved();
    }

    static async readFromConfig(path_: string): Promise<ProjectModel> {
        const file_path = joinPath(path_, "config.json");
        try {
            const exists: boolean = await invoke("file_exists", { path: file_path });
            if (!exists) {
                return Err("Config does not exist").asRejected();
            }

            const content: string = await invoke("read_file", { path: file_path });
            return Ok(Object.assign(new ProjectModel(), JSON.parse(content))).asResolved();
        }
        catch (err) {
            return Err(err).asRejected();
        }
    }
}