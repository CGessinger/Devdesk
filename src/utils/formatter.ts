export class formatter {
    static formatName(name: string): string {
        name = name.split("-").join(" ");
        name = name.split("_").join(" ");
        return name;
    }
}