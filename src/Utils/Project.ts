import type { Error, Result} from "../Utils/Result";
import { Ok, Err } from "../Utils/Result";

export class Project{
    name: string;
    description: string;
    path: string;
    image: string;
    tags: string[];
    technologies: string[];
    date: string;
    type: string;
    constructor() {
        this.name = "";
        this.description = "";
        this.path = "";
        this.image = "";
        this.tags = [];
        this.technologies = [];
        this.date = "";
    }

    static Builder = class {
        p: Project;
        constructor() {
            this.p = new Project();
        }

        static get() {
            return new this();
        }

        withName(name: string) {
            this.p.name = name;
            return this;
        }

        formattedName() {
            return this.formattedString(this.p.name);
        }

        withDescription(description: string) {
            this.p.description = description;
            return this;
        }

        withPath(path: string) {
            this.p.path = path;
            return this;
        }

        withImageB64(image: string) {
            this.p.image = image;
            return this;
        }

        withTags(tags: string[]) {
            this.p.tags = tags;
            return this;
        }

        withTechnologies(technologies: string[]) {
            this.p.technologies = technologies;
            return this;
        }

        withDate(date: string) {
            this.p.date = date;
            return this;
        }

        withType(type: string) {
            this.p.type = type;
            return this;
        }

        formattedType() {
            return this.formattedString(this.p.type);
        }

        formattedString(str: string) {
            return str.replace(/\s+/g, "-").toLowerCase();
        }

        tryBuildPath(tree: string): Result<typeof this, string> {
            const form_type = this.formattedType();
            const form_name = this.formattedName();

            if (form_type == "") {
                return Err("No type provided");
            } else if (form_name == "") {
                return Err("No name provided");
            }

            return Ok(this.withPath(tree + "/" + form_type + "/" + form_name));
        }

        build(): Result<Project, string> {
            if (this.p.name == "") {
                return Err("No name provided");
            } else if (this.p.path == "") {
                return Err("No path provided");
            }

            return Ok(this.p);
        }
    }
}
