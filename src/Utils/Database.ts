import type { ProjectModel } from "$src/Project/utils/ProjectModel";
import { invoke } from "@tauri-apps/api";
import * as SqlString from "sqlstring-sqlite"

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

    type queryParamteters = {
        dir?: string,
        textSearch?: string[],
    }
    export class query {
        parameters: queryParamteters;

        constructor (param?: queryParamteters) {
            this.parameters = {
                dir: "",
                textSearch: []
            };
            if (param)
                Object.assign(this.parameters, param);
        }

        withDir(_dir: string): query {
            if(!_dir)
                return this;

            this.parameters.dir = _dir;
            return this;
        }

        withSearch(search?: string[]): query {
            if (!search)
                return this;

            this.parameters.textSearch = search;
            return this;
        }

        build(): string {
            const conditions = [];

            const searchConditions = [];
            for(const text of this.parameters.textSearch) {
                const safeText = SqlString.escape(`%${text}%`);
                const cn = `name LIKE ${safeText}`
                const cd = `description LIKE ${safeText}`
                const cp = `path LIKE ${safeText}`
                searchConditions.push(cn, cd, cp);
            }
            if (searchConditions.length)
                conditions.push("(" + searchConditions.join(" OR ") + ")");

            if (this.parameters.dir) {
                const safeText = SqlString.escape(`%${this.parameters.dir}%`);
                const ct = `path LIKE ${safeText}`
                conditions.push(ct);
            }

            return `SELECT * FROM project WHERE ${conditions.join(" AND ")}`
        }
    }
}