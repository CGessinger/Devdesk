import type { Portfolio } from '$src/utils/Portfolio';
import type { Project } from '$src/utils/Project';
import { Settings } from '$src/utils/Settings';
import { writable } from 'svelte/store';

export const focused_portfolio = writable<Portfolio>(undefined);
export const focused_project = writable<Project>(undefined);
export const focus_settings = writable<boolean>(false);
export const cached_settings = writable<Settings>(new Settings());