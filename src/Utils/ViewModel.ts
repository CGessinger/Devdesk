export abstract class ViewModel {    
    private readonly ViewData: any;
    onViewDataChange: (key: string, value: any) => void;

    abstract Name: string;

    constructor() {
        this.ViewData = {};
        this.onViewDataChange = (key: string, value: any) => {};
    }

    ViewDataChange(key: string, value: any): void {
        this.ViewData[key] = value;
        this.onViewDataChange(key, value);
    }

    GetViewData(): any {
        return this.ViewData;
    }
}