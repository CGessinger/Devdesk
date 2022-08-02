import { Portfolio } from "./Portfolio";

import { open } from "@tauri-apps/api/dialog";
import { documentDir, appDir, BaseDirectory } from "@tauri-apps/api/path";
import { readTextFile, writeTextFile, createDir } from "@tauri-apps/api/fs";

export class settings {
    portfolios: Portfolio[];

    constructor() {
        this.portfolios = [];
    }

    public async add_portfolio() {
        const selected = await open({
            directory: true,
            multiple: false,
            defaultPath: await documentDir(),
        });
        this.portfolios = [...this.portfolios, new Portfolio(selected.toString())];
    }

    public async safe_settings() {
        // const settings_dir = await appDir() + "config/";
        await createDir("config", {
            dir: BaseDirectory.App,
            recursive: true,
        });
        const settings_json = JSON.stringify(this);
        await writeTextFile({path: await appDir() + "config/settings.json", contents: settings_json});
    }

    public static async get_settings_from_config(): Promise<settings> {
        let sett: settings;
        await readTextFile("config/settings.json", { dir: BaseDirectory.App }).then(read => {
            sett = JSON.parse(read);
        }).catch(err => {
            console.log("Error reading settings file: " + err);
            sett = new settings();
        });
        
        return Object.assign(new settings, sett);
    }

}