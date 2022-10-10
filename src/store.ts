import { SettingsModel } from '$src/Settings/utils/SettingsModel';
import { writable } from 'svelte/store';
import { ComponentStateController } from './utils/ComponentStateController';

export const cached_settings = writable<SettingsModel>(new SettingsModel());
export const StateController = ComponentStateController();
