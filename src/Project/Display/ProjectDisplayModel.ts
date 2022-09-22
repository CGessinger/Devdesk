import { new_project } from "$src/store";
import { ViewModel } from "$src/utils/ViewModel";
import type { ProjectModel } from "../utils/ProjectModel";

export class Model extends ViewModel {
    public Name = "ProjectDisplay";
    private display: ProjectModel;

    constructor(p: ProjectModel) {
        super();
        this.display = p;
        
        super.ViewDataChange("config_exists", false);

        this.display.config_exists().then((exists) => {
            super.ViewDataChange("config_exists", exists);
        });
    }

    edit_model(model: ProjectModel) {
        new_project.update(np => np = model);
    }

    init_model(model: ProjectModel) {
        new_project.update(np => np = model);
    }
}