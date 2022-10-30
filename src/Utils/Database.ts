import type { ProjectModel } from "$src/Project/utils/ProjectModel";
import { invoke } from "@tauri-apps/api";
import * as SqlString from "sqlstring-sqlite"

export namespace projectdb {

    export async function load_recursive(path: string, maxrecursion: number = 1): Promise<{}> {
        console.log("load_recursive", path, maxrecursion);
        return invoke("load_recursive", { path: path, maxrecursion: maxrecursion });
    }

    export async function query_database(q: string): Promise<{path: string, isproject: boolean}[]> {
        return invoke("query_database", { query: q });
    }

    export async function clear_db() {
        return invoke("clear_db");
    }

    type queryParamteters = {
        dir?: string,
        textSearch?: string[],
        isproject?: boolean,
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

        projectsOnly(): query {
            this.parameters.isproject = true;
            return this;
        }

        foldersOnly(): query {
            this.parameters.isproject = false;
            return this;
        }

        build(): string {
            const conditions = [
                `(isproject = ${this.parameters.isproject.toString().toUpperCase()})`,
            ];

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

            return `SELECT * FROM entry WHERE ${conditions.join(" AND ")}`
        }
    }
}