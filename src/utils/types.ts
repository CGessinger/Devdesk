import { z } from "zod";

export const VaultConfigObj = z.object({

});
export type VaultConfig = z.infer<typeof VaultConfigObj>;

export const VaultObj = z.object({
    vault_id: z.number(),
    path: z.string(),
    parent_vault_id: z.number(),
    config: VaultConfigObj.optional(),
});
export type Vault = z.infer<typeof VaultObj>;

export const ProjectObj = z.object({
    project_id: z.number(),
    name: z.string(),
    path: z.string(),
    modified: z.string(),
    vault_id: z.number(),
});
export type Project = z.infer<typeof ProjectObj>;

export const DashboardResponseObj = z.object({
    vault: VaultObj,
    sub_directories: z.array(VaultObj),
    projects: z.array(ProjectObj),
    recent: z.array(ProjectObj),
    selected: ProjectObj.optional(),
});
export type Dashboard = z.infer<typeof DashboardResponseObj>;