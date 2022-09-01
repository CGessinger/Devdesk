import type { Portfolio } from '$src/utils/Portfolio';
import type { Project } from '$src/utils/Project';
import { writable } from 'svelte/store';

export const focused_portfolio = writable<Portfolio>(undefined);
export const focused_project = writable<Project>(undefined);