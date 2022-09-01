import type { Result } from "$utils/Result";
import { Ok, Err } from "$utils/Result";
import { typeFromPath, joinPath, nameFromPath } from "$utils/Path";
import { invoke } from "@tauri-apps/api";

export class Project {
    name: string;
    description: string;
    path: string;
    image: string;
    tags: string[];
    technologies: string[];
    date: string;
    type: string;

    config_folder_path = (): string => {
        return this.path + "/.ppa";
    }

    config_path = (): string => {
        return this.path + "/.ppa/config.json";
    }
    
    constructor() {
        this.name = "";
        this.description = "";
        this.path = "";
        this.image = "";
        this.tags = [];
        this.technologies = [];
        this.date = "";
        this.type= "";
    }

    async load_image(): Promise<void> {
        const img_path = joinPath(this.config_folder_path(), "icon.png");
        try {
            if(!await invoke("file_exists", {path: img_path}))
                return Err("Image not found").asRejected();
    
            this.image = await invoke("load_image", { path: img_path })
            return Ok(null).asResolved();
        }
        catch (err) {
            return Err(err).asRejected();
        }
    }

    static Builder = class {
        p: Project;
        target_path = (tree: string): string => {
            return this.buildFormattedPath(tree).value;
        }
        constructor() {
            this.p = new Project();
        }

        static get() {
            return new this();
        }

        withName(name: string) {
            this.p.name = name;
            return this;
        }

        formattedName() {
            return this.formattedString(this.p.name);
        }

        withDescription(description: string) {
            this.p.description = description;
            return this;
        }

        withPath(path: string) {
            this.p.path = path;
            return this;
        }

        withImageB64(image: string) {
            this.p.image = image;
            return this;
        }

        withTags(tags: string[]) {
            this.p.tags = tags;
            return this;
        }

        withTechnologies(technologies: string[]) {
            this.p.technologies = technologies;
            return this;
        }

        withDate(date: string) {
            this.p.date = date;
            return this;
        }

        withType(type: string) {
            this.p.type = type;
            return this;
        }

        formattedType() {
            return this.formattedString(this.p.type);
        }

        formattedString(str: string) {
            return str.replace(/\s+/g, "-").toLowerCase();
        }

        buildFormattedPath(tree: string): Result<string, string> {
            const form_type = this.formattedType();
            const form_name = this.formattedName();

            if (form_type == "") {
                return Err("No type provided");
            } else if (form_name == "") {
                return Err("No name provided");
            }

            return Ok(`${tree}/${form_type}/${form_name}`);
        }

        tryBuildPath(tree: string): Result<typeof this, string> {
            const path = this.buildFormattedPath(tree);
            if (path.is_err()) {
                return Err(path.value);
            }
            return Ok(this.withPath(path.unwrap()));
        }

        build(): Result<Project, string> {
            if (this.p.name == "") {
                return Err("No name provided");
            } else if (this.p.path == "") {
                return Err("No path provided");
            }

            return Ok(this.p);
        }
    }

    static Folder = class {
        p: Project;
        constructor(p_: Project) {
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

        static async readFromFolder(path_: string): Promise<Project> {
            const exists: boolean = await invoke("folder_exists", { path: path_ });
            if (!exists) {
                return Err("Folder does not exist: " + path_).asRejected();
            }

            const config_path = joinPath(path_, ".ppa");
            let project;
            await Project.Folder.readFromConfig(config_path).then(p => {
                project = p;
            }).catch(_ => {
                project = new Project()
                project.name = nameFromPath(path_);
            }).finally(() => {
                project.path = path_;
                project.type = typeFromPath(path_);
            });

            return Ok(project).asResolved();
        }

        static async readFromConfig(path_: string): Promise<Project> {
            const file_path = joinPath(path_, "config.json");
            try {
                const exists: boolean = await invoke("file_exists", { path: file_path });
                if (!exists) {
                    return Err("Config does not exist").asRejected();
                }

                const content: string = await invoke("read_file", { path: file_path });
                return Ok(Object.assign(new Project(), JSON.parse(content))).asResolved();
            }
            catch (err) {
                return Err(err).asRejected();
            }
        }
    }

}
