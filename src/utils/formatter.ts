export class formatter {
    static formatName(name: string): string {
        name = name.split("-").join(" ");
        name = name.split("_").join(" ");
        return name;
    }

    static formatScriptName(name: string): string {
        let lastPointIndex = name.lastIndexOf(".");
        name = name.slice(0, lastPointIndex);
        return this.formatName(name);
    }

    static breadcrumpsFrom(path: string, fallbackPath: string = ""): string[] {
        if (path) {
            return path.split("/");
        }
        return fallbackPath.split("/");
    }
}