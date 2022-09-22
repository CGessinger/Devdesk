import { open } from "@tauri-apps/api/dialog";
import { documentDir, appDir, BaseDirectory } from "@tauri-apps/api/path";
import { readTextFile, writeTextFile, createDir } from "@tauri-apps/api/fs";
import { PortfolioModel } from "$src/Portfolio/utils/PortfolioModel";

export class SettingsModel {
    portfolios: PortfolioModel[];
    dark_mode: boolean;

    constructor() {
        this.portfolios = [];
        this.dark_mode = true;
    }

    public async add_portfolio() {
        const selected = await open({
            directory: true,
            multiple: false,
            defaultPath: await documentDir(),
        });
        this.portfolios = [...this.portfolios, new PortfolioModel(selected.toString())];
    }

    public async safe_settings() {
        await createDir("config", {
            dir: BaseDirectory.App,
            recursive: true,
        });
        const settings_json = JSON.stringify(this);
        await writeTextFile({path: await appDir() + "config/settings.json", contents: settings_json});
    }

    public static async get_settings_from_config(): Promise<SettingsModel> {
        let sett: SettingsModel;
        await readTextFile("config/settings.json", { dir: BaseDirectory.App }).then(read => {
            sett = JSON.parse(read, (key, value) => {
                if (key === "portfolios") {
                    return value.map(p => Object.assign(new PortfolioModel(""), p));
                }
                return value;
            });
        }).catch(err => {
            console.log("Error reading settings file: " + err);
            sett = new SettingsModel();
        });
        
        return Object.assign(new SettingsModel, sett);
    }

}