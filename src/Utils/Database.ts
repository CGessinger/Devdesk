import type { ProjectModel } from "$src/Project/utils/ProjectModel";
import { invoke } from "@tauri-apps/api";

export namespace projectdb {
    export function insert_project (project: ProjectModel) {
        return invoke('insert_project', {project: project});
    }

    export function insert_projects (projects: ProjectModel[]) {
        return invoke('insert_projects', {projects: projects});
    }

    export function get_projects (filter?: string): Promise<ProjectModel[]> {
        if(filter)
            return invoke('get_projects_filter', {filter: filter});

        return invoke('get_projects');
    }

    export function clear_db () {
        return invoke('clear_db');
    }
}