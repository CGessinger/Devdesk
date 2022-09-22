import { SettingsModel } from '$src/Settings/utils/SettingsModel';
import { writable } from 'svelte/store';
import type { PortfolioModel } from './Portfolio/utils/PortfolioModel';
import type { ProjectModel } from './Project/utils/ProjectModel';

// Listed By Priority
export const focus_settings = writable<boolean>(false);
export const new_project = writable<ProjectModel>(undefined);
export const focused_project = writable<ProjectModel>(undefined);
export const focused_portfolio = writable<PortfolioModel>(undefined);

export const cached_settings = writable<SettingsModel>(new SettingsModel());
