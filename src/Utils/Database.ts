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
        types?: string[],
        textSearch?: string[],
    }
    export class query {
        parameters: queryParamteters;

        constructor (param?: queryParamteters) {
            this.parameters = {
                types: [],
                textSearch: []
            };
            if (param)
                Object.assign(this.parameters, param);
        }

        withTypes(_types?: string[]): query {
            if(!_types)
                return this;

            this.parameters.types = _types;
            return this;
        }

        withSearch(search?: string[]): query {
            if (!search)
                return this;

            this.parameters.textSearch = search;
            return this;
        }

        build(): string {
            const searchConditions = [];
            for(const text of this.parameters.textSearch) {
                const safeText = SqlString.escape(`%${text}%`);
                const cn = `name LIKE ${safeText}`
                const cd = `description LIKE ${safeText}`
                const cp = `path LIKE ${safeText}`
                searchConditions.push(cn, cd, cp);
            }
            const typeConditions = [];
            for(const type of this.parameters.types) {
                const safeText = SqlString.escape(`%${type}%`);
                const ct = `type LIKE ${safeText}`
                typeConditions.push(ct);
            }
            const conditions = [];
            if (searchConditions.length)
                conditions.push("(" + searchConditions.join(" OR ") + ")");
            if(typeConditions.length)
                conditions.push("(" + typeConditions.join(" OR ") + ")");

            return `SELECT * FROM project WHERE ${conditions.join(" AND ")}`
        }
    }
}