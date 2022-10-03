import { focused_portfolio } from "$src/store";
import { ViewModel } from "$src/utils/ViewModel";
import type { PortfolioModel } from "../utils/PortfolioModel";

export class Model extends ViewModel {
    Name: string = "PortfolioDisplay";
    private focus: PortfolioModel;
    constructor() {
        super();
        this.subscribeStores();
    }

    subscribeStores(): void {
        focused_portfolio.subscribe((value) => {
            this.focus = value;
            this.ViewDataChange("projects", this.focus.projects);
            this.ViewDataChange("focused_type", this.focus.get_focused_type());
        });
    }
}