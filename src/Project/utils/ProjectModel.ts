import { Ok, Err } from "$utils/Result";
import { joinPath } from "$utils/Path";
import { invoke } from "@tauri-apps/api";

export class ProjectModel {
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

    async config_exists(): Promise<boolean> {
        return await invoke("file_exists", {path: this.config_path()});
    }
}