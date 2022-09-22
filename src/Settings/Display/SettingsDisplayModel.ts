import { cached_settings } from "$src/store";
import { ViewModel } from "$src/utils/ViewModel";
import type { SettingsModel } from "../utils/SettingsModel";

export class Model extends ViewModel {
    Name: string = "SettingsDisplay";
    settings: SettingsModel;
    constructor() {
        super();
        this.subscribeStores();
    }

    subscribeStores(): void {
        cached_settings.subscribe((value) => {
            this.settings = value;
            super.ViewDataChange("dark_mode", this.settings.dark_mode);
        });
    }

    toggle_dark_mode(): void {
        this.settings.dark_mode = !this.settings.dark_mode;
        cached_settings.update((_settings) => (_settings = this.settings));
        this.settings.safe_settings();
    }
}