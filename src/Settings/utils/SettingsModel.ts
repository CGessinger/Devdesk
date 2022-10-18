import { open } from "@tauri-apps/api/dialog";
import { PortfolioModel } from "$src/Portfolio/utils/PortfolioModel";
import { fs } from "$utils/Path";
import { commandDefaults } from "$utils/Scripts";

export class SettingsModel {
    portfolios: PortfolioModel[];
    dark_mode: boolean;
    runThree: boolean;
    experimental: boolean;
    editorCmd: string;
    terminalCmd: string;

    constructor() {
        this.portfolios = [];
        this.dark_mode = true;
        this.runThree = false;
        this.experimental = false;
        this.editorCmd = "";
        this.terminalCmd = "";
    }

    public async addPortfolio() {
        const selected = await open({
            directory: true,
            multiple: false,
            defaultPath: await fs.documentDir(),
        });
        this.portfolios = [...this.portfolios, new PortfolioModel(selected.toString())];
    }

    public async removePortfolioByPath(removeTargetPath: string) {
        this.portfolios = this.portfolios.filter(p => p.path != removeTargetPath);
    }

    public async safeSettings() {
        console.log("safeSettings")
        const folder_path = fs.joinPath(await fs.appDir(), "config");
        fs.create_folder(folder_path)
            .then(() => {
                const file_path = fs.joinPath(folder_path, "settings.json");
                const settings_json = JSON.stringify(this);
                fs.write_to_file(file_path, settings_json)
                    .catch(err => {
                        console.log(err);
                    });
            })
            .catch(err => {
                console.log(err);
            }
        );
    }

    public static async get_settings_from_config(): Promise<SettingsModel> {
        const file_path = fs.joinPath(await fs.appDir(), "config", "settings.json");
        let sett: SettingsModel;
        await fs.read_file(file_path).then(read => {
            sett = JSON.parse(read as string, (key, value) => {
                if (key === "portfolios") {
                    return value.map(p => Object.assign(new PortfolioModel(""), p));
                }
                return value;
            });
        }).catch(err => {
            console.log("Error reading settings file: " + err);
            sett = new SettingsModel();
        });

        if(!sett.editorCmd) {
            sett.editorCmd = await commandDefaults.defaultEditorCommand();
        }

        if(!sett.terminalCmd) {
            sett.terminalCmd = await commandDefaults.defaultTerminalCommand();
        }

        return Object.assign(new SettingsModel, sett);
    }

}