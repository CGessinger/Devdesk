import type { PromisedResult, Result } from "$utils/Result";
import { Ok, Err, isErr, unwrap } from "$utils/Result";
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

    async load_image(): PromisedResult<void, string> {
        const img_path = joinPath(this.config_folder_path(), "icon.png");
        try {
            if(!await invoke("file_exists", {path: img_path}))
                return Err("Image not found");
    
            this.image = await invoke("load_image", { path: img_path })
            return Ok(null);
        }
        catch (err) {
            return Err(err);
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
            if (isErr(path)) {
                return Err(path.value);
            }
            return Ok(this.withPath(unwrap(path)));
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

        private async createFolder(path_: string): PromisedResult<typeof this, string> {
            try {
                let exists = await invoke("folder_exists", { path: path_ });
                if (!exists) {
                    invoke("create_folder", { path: path_ }).catch(err => {
                        return Err(err);
                    });
                    return Ok(this);
                }

                return Err("Folder already exists");
            }
            catch (err) {
                return Err(err);
            }
        }

        async createProjectFolder(): PromisedResult<typeof this, string> {
            const path = this.p.path;
            if (path == "") return Err("No path provided");

            return this.createFolder(path);
        }

        async createConfigFolder(): PromisedResult<typeof this, string> {
            if (this.p.path == "") return Err("No path provided");

            const path = joinPath(this.p.path, ".ppa");
            return this.createFolder(path);
        }

        async writeToConfig(): PromisedResult<typeof this, string> {
            if (this.p.path == "") return Err("No path provided");

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
                return Ok(this);
            }).catch(err => {
                return Err(err);
            });
        }

        static async readFromFolder(path_: string): PromisedResult<Project, string> {
            try {
                const exists: boolean = await invoke("folder_exists", { path: path_ });
                if (!exists) {
                    return Err("Folder does not exist");
                }

                const config_path = joinPath(path_, ".ppa");
                const config_result = await Project.Folder.readFromConfig(config_path);
                let project;
                if (isErr(config_result)) {
                    project = new Project()
                    project.name = nameFromPath(path_);
                } else {
                    project = unwrap(config_result);
                }

                project.path = path_;
                project.type = typeFromPath(path_);
                return Ok(project);
            }
            catch (err) {
                return Err(err);
            }
        }

        static async readFromConfig(path_: string): PromisedResult<Project, string> {
            const file_path = joinPath(path_, "config.json");
            try {
                const exists: boolean = await invoke("file_exists", { path: file_path });
                if (!exists) {
                    return Err("Config does not exist");
                }

                const content: string = await invoke("read_file", { path: file_path });
                return Ok(Object.assign(new Project(), JSON.parse(content)));
            }
            catch (err) {
                return Err(err);
            }
        }
    }

}
