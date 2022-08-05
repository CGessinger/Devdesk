import type { PromisedResult, Result } from "$utils/Result";
import { Ok, Err, isErr, unwrap } from "$utils/Result";
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

            const path = this.p.path + "/.ppa";
            return this.createFolder(path);
        }
    }

}
