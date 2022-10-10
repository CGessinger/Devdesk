import type { PortfolioModel } from "$src/Portfolio/utils/PortfolioModel";
import { ProjectModel } from "$src/Project/utils/ProjectModel";
import { fs } from "$src/utils/Path";
import { Ok, Err, type Result } from "$utils/Result";

interface ProjectModelBuilderParameters {
    name?: string,
    description?: string | undefined,
    git_url?: string | undefined,
    git_branch?: string | undefined,
    type?: string | undefined,
    targetPortfolioPath?: string | undefined
}

export class ProjectModelBuilder {
    parameters: ProjectModelBuilderParameters;
    constructor(param?: ProjectModelBuilderParameters) {
        this.parameters = param ?? {};
    }

    formatString(str: string) {
        return str.replace(/\s+/g, "-").toLowerCase();
    }

    async projectAlreadyExists(): Promise<boolean> {
        const path = this.getPathFormattedPortfolio();
        if(path.is_err())
            return false;
        
        const configPath = fs.joinPath(path.unwrap(), ".ppa", "config.json");
        const exists = await fs.file_exists(configPath);
        return exists;
    }

    getPathFormattedPortfolio(): Result<string, string> {
        const valid = this.isValidForCreation();
        if (valid.is_err()) 
            return Err(valid.value as string);

        const formattedType = this.formatString(this.parameters.type);
        const formattedName = this.formatString(this.parameters.name);
        const portfolioPath = this.parameters.targetPortfolioPath;

        const formattedPath = fs.joinPath(portfolioPath, formattedType, formattedName);

        return Ok(formattedPath);
    }

    // ToDo false use of Result
    isValidForCreation(): Result<{}, string> {
        if (!this.parameters.type || this.parameters.type == "") {
            return Err("No type provided");
        } else if (!this.parameters.name || this.parameters.name == "") {
            return Err("No name provided");
        } else if (!this.parameters.targetPortfolioPath) {
            return Err("No portfolio provided");
        }
        return Ok({})
    }

    build(): Result<ProjectModel, string> {
        const valid = this.isValidForCreation();
        if (valid.is_err())
            return Err(valid.value as string);
        
        const project = new ProjectModel();
        project.name = this.parameters.name;
        project.description = this.parameters.description;
        project.path = this.getPathFormattedPortfolio().unwrap();
        project.type = this.parameters.type;
        return Ok(project);
    }
}