import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";
import { fs } from "$utils/Path";
import { projectdb } from "$utils/Database";
import type { ProjectModel } from "$src/Project/utils/ProjectModel";
import { ProjectFileHandler } from "$src/Project/utils/ProjectFileHandler";

export namespace Portfolio {

    export interface Model {
        readonly path: string;
        readonly subDirFilter: string[];
    }

    export function subDirFilterString (portfolio: Model): string {
        const p = portfolio;
        return p.subDirFilter ? fs.joinPath(...p.subDirFilter) : "All";
    }

    export function modelFromPath(path: string): Model {
        return {
            path: path,
            subDirFilter: [],
        }
    }

    export function modelFrom(m: Model, changes: Partial<Model>): Model {
        return {
            path: changes.path || m.path,
            subDirFilter: changes.subDirFilter || m.subDirFilter,
        }
    }

    async function scanPathForProjects(path: string): Promise<ProjectModel[]> {
        const folders: string[] = await fs.read_dir(path);
        const projects: ProjectModel[] = [];
        for (const f of folders) {
            await ProjectFileHandler.readFromFolder(fs.joinPath(path, f)).then(p =>{
                projects.push(p);
            }).catch(err => {
                console.log(err);
            });      
        }

        return projects;
    }

    export async function loadProjectsToDatabase(portfolio: Portfolio.Model) {

        const loadOrEmpty = async (path): Promise<ProjectModel[]> => {
            return await scanPathForProjects(path).then(projects => {
                return projects
            }).catch(e => {
                console.log(e);
                return [];
            });
        }

        const projects = await (async (): Promise<ProjectModel[]> => {
            let projects = [];
            console.log(portfolio);
            const path = fs.joinPath(portfolio.path, ...portfolio.subDirFilter);
            projects = [...projects, ...await loadOrEmpty(path)];
            return projects;
        })();

        // Clear db
        await projectdb.clear_db();
        // Write Projects to db
        await projectdb.insert_projects(projects);
    }

    export async function getProjectsFromDatabase(portfolio: Model, filter?: projectdb.query): Promise<ProjectModel[]> {
        const query = filter ?? new projectdb.query();
        const focusedType = subDirFilterString(portfolio);

        query.withDir(fs.joinPath(portfolio.path, ...portfolio.subDirFilter));

        return await projectdb.get_projects(query.build());
    }
}

export namespace Settings {

    export interface Switches {
        readonly darkMode: boolean,
        readonly runThree: boolean,
        readonly experimental: boolean
    }

    export const DefaultSwitches: Switches = {
        darkMode: false,
        runThree: false,
        experimental: false
    }

    export interface Commands {
        readonly editorCmd: string,
        readonly terminalCmd: string,
    }

    export const DefaultCommands: Commands = {
        editorCmd: "",
        terminalCmd: ""
    }

    export interface Model {
        readonly switches: Switches;
        readonly commands: Commands;
    }

    export const DefaultSettings: Model = {
        switches: DefaultSwitches,
        commands: DefaultCommands,
    }

    type DeepPartial<T> = Partial<{ [P in keyof T]: DeepPartial<T[P]> }>;
    export function modelFrom(m: Model, changes: DeepPartial<Model>): Model {
        return {
            switches: switchesFrom(m.switches, changes.switches ?? {}),
            commands: commandsFrom(m.commands, changes.commands ?? {})
        }
    }

    export function switchesFrom(m: Switches, changes: Partial<Switches>): Switches {
        return {
            darkMode: changes.darkMode ?? m.darkMode,
            runThree: changes.runThree ?? m.runThree,
            experimental: changes.experimental ?? m.experimental
        }
    }

    export function commandsFrom(m: Commands, changes: Partial<Commands>): Commands {
        return {
            editorCmd: changes.editorCmd ?? m.editorCmd,
            terminalCmd: changes.terminalCmd ?? m.terminalCmd
        }
    }

    export async function addPortfolioFromDialog() {
        const selected = await open({
            directory: true,
            multiple: false,
            defaultPath: await fs.documentDir(),
        });
        return add_portfolio(Portfolio.modelFromPath(selected as string));
    }

    export async function changeSettings(settings: Model): Promise<Model> {
        return invoke("set_settings", {
            switches: settings.switches,
            commands: settings.commands
        });
    }

    export async function add_portfolio(portfolio: Portfolio.Model): Promise<Portfolio.Model[]> {
        return invoke("add_portfolio_to_settings", { portfolio })
    }

    export async function remove_portfolio(path: String): Promise<Portfolio.Model[]> {
        return invoke("remove_portfolio_from_settings", { path })
    }

    export async function update_portfolio(portfolio: Portfolio.Model): Promise<Portfolio.Model[]> {
        return invoke("update_portfolio_in_settings", { portfolio })
    }

    export async function get_portfolios(): Promise<Portfolio.Model[]> {
        return invoke("get_portfolios_from_settings")
    }

    export async function getSwitches(): Promise<Switches> {
        return invoke("get_switches_from_settings")
    }

    export async function getCommands(): Promise<Commands> {
        return invoke("get_commands_from_settings")
    }

    export async function getSettings(): Promise<Model> {
        return {
            switches: await getSwitches(),
            commands: await getCommands()
        }
    }
}