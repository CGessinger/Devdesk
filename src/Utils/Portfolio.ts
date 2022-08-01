export class Portfolio {
    uid: string;
    path: string;
    types = ["Concept", "Sandbox", "Release"];
    constructor(path: string) {
        this.uid = path;
        this.path = path;
    }

    async load_projects(): Promise<void> {
        
    }
}