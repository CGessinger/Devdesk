import { ProjectModel } from "$src/Project/utils/ProjectModel";
import { Ok, Err, type Result } from "$utils/Result";

export class ModelBuilder {
    p: ProjectModel;
    target_path = (tree: string): string => {
        return this.buildFormattedPath(tree).value;
    }
    
    constructor(_p?: ProjectModel) {
        this.p = _p ?? new ProjectModel();
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

    withGitUrl(git: string) {
        this.p.git.url = git;
        return this;
    }

    withGitBranch(branch: string) {
        this.p.git.branch = branch;
        return this;
    }

    formattedType() {
        return this.formattedString(this.p.type);
    }

    formattedString(str: string) {
        return str.replace(/\s+/g, "-").toLowerCase();
    }

    buildFormattedPath(tree: string): Result<string, string> {
        const form_type = this.formattedType();
        const form_name = this.formattedName();

        if (form_type == "") {
            return Err("No type provided");
        } else if (form_name == "") {
            return Err("No name provided");
        }

        return Ok(`${tree}/${form_type}/${form_name}`);
    }

    tryBuildPath(tree: string): Result<typeof this, string> {
        const path = this.buildFormattedPath(tree);
        if (path.is_err()) {
            return Err(path.value);
        }
        return Ok(this.withPath(path.unwrap()));
    }

    build(): Result<ProjectModel, string> {
        if (this.p.name == "") {
            return Err("No name provided");
        } else if (this.p.path == "") {
            return Err("No path provided");
        }

        return Ok(this.p);
    }
}