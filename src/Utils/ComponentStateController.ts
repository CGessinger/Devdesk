import type { ProjectModel } from "$src/Project/utils/ProjectModel";
import PortfolioView from "../Components/PortfolioDisplayView.svelte";
import ProjectView from "../Project/ProjectDisplayView.svelte";
import NewProjectView from "../Project/ProjectCreateView.svelte";
import SettingsView from "../Settings/SettingsDisplayView.svelte";
import type { ProjectModelBuilder } from "$src/Project/utils/ProjectModelBuilder";
import type { Portfolio } from "$utils/Data";
import { writable } from "svelte/store";

export enum WindowStates {
    Dasboard,
    Settings,
    ProjectCreation,
    Project,
    Portfolio
}

export class StateHolder {
    windowState: WindowStates;
    value: any;
    _prevWindowState?: WindowStates;
    _prevValue?: any;

    constructor(state: WindowStates, value: any, prevState?: WindowStates, prevValue?: any) {
        this.windowState = state;
        this.value = value;
        this._prevWindowState = prevState;
        this._prevValue = prevValue;
    }

    getComponent() {
        switch (this.windowState) {
            case WindowStates.Dasboard:
                return null;
            case WindowStates.Settings:
                return SettingsView;
            case WindowStates.ProjectCreation:
                return NewProjectView;
            case WindowStates.Project:
                return ProjectView;
            case WindowStates.Portfolio:
                return PortfolioView;
        }
    }
}

export const ComponentStateController = () => {
	const { subscribe, set, update } = writable(new StateHolder(WindowStates.Dasboard, null));

    const updateTo = (newState: StateHolder) => {
        update(state => {
            newState._prevValue = state.value;
            newState._prevWindowState = state.windowState;
            return newState;
        })
    }

    return {
		subscribe,
        switchToSettings: () => updateTo(new StateHolder(WindowStates.Settings, null)),
        switchToProjectCreation: (p: ProjectModelBuilder) => updateTo(new StateHolder(WindowStates.ProjectCreation, p)),
        switchToProject: (p: ProjectModel) => updateTo(new StateHolder(WindowStates.Project, p)),
        switchToPortfolio: (p: Portfolio.Model) => updateTo(new StateHolder(WindowStates.Portfolio, p)),
        update,
        switchToPrev: () => update(state => state = new StateHolder(state._prevWindowState, state._prevValue)),
		reset: () => set(new StateHolder(WindowStates.Dasboard, null)),
	};
}