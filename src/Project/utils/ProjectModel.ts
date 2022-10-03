import { fs } from "$src/utils/Path";
import { Ok, Err } from "$utils/Result";
import { invoke } from "@tauri-apps/api";

export class ProjectModel {
    name: string;
    description: string;
    path: string;
    image: string;
    git: {
        url: string;
        branch: string;
    };
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
        this.git = {
            url: "",
            branch: ""
        };
    }

    async load_image(): Promise<void> {
        const img_path = fs.joinPath(this.config_folder_path(), "icon.png");
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

    // ToDo Purge Invoke. Use fs instead
    async config_exists(): Promise<boolean> {
        return await invoke("file_exists", {path: this.config_path()});
    }
}