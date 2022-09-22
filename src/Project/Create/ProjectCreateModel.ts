import { ModelBuilder } from "../utils/ModelBuilder";
import { focused_portfolio, new_project } from '$src/store';
import type { ProjectModel } from '../utils/ProjectModel';
import { ProjectFileHandler } from '../utils/ProjectFileHandler';
import { ViewModel } from "$src/utils/ViewModel";

import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog'
import type { PortfolioModel } from "$src/Portfolio/utils/PortfolioModel";

export class Model extends ViewModel{
    public Name = "ProjectCreate";
    public builder: ModelBuilder;
    private focus: PortfolioModel;

    constructor(edit?: ProjectModel) {
        super();
        this.subscribeStores();

        this.builder = new ModelBuilder();
        this.builder.withType(this.focus.focused_type == -1 ? this.focus.types[0] : this.focus.get_focused_type());
        super.ViewDataChange("path_preview", this.builder.target_path(this.focus.path));
    }

    subscribeStores() {
        focused_portfolio.subscribe((value) => {
            this.focus = value;
            super.ViewDataChange("types", this.focus.types);
            super.ViewDataChange("focused_type", this.focus.focused_type);
            super.ViewDataChange("path_preview", this.focus.path);
        });
    }

    create_project() {
        const res = this.builder.tryBuildPath(this.focus.path);
        if (res.is_err()) {
            console.log("error: ", res);
            return;
        }
        const built = this.builder.build();
        if (built.is_err()) {
            console.log("error: ", built);
            return;
        }
        const fb = new ProjectFileHandler(built.unwrap() as ProjectModel);
        fb.createConfigFolder().then(() => {
            fb.writeToConfig();
        }).then(() => {
            new_project.update(np => np = undefined);
            this.focus
                .load_projects_from_type()
                .then(() => focused_portfolio.update((p) => (p = p)));
        });
    }

    async change_icon() {
        const selected = await open({
            multiple: false,
            title: "Change Icon",
            filters: [
                { name: "Images", extensions: ["png", "jpg", "jpeg"] }
            ],
        });
        this.builder.withImageB64(await invoke("load_image", { path: selected.toString() }));
    }
    
    change_type(type: string) {
        this.builder.withType(type);
        super.ViewDataChange("path_preview", this.builder.target_path(this.focus.path));
    }

    change_name(name: string) {
        this.builder.withName(name);
        super.ViewDataChange("path_preview", this.builder.target_path(this.focus.path));
    }

}