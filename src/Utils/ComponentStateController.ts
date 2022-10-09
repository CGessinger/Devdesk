import type { PortfolioModel } from "$src/Portfolio/utils/PortfolioModel";
import type { ProjectModel } from "$src/Project/utils/ProjectModel";
import type { SettingsModel } from "$src/Settings/utils/SettingsModel";
import { writable } from "svelte/store";

import PortfolioView from "../Portfolio/PortfolioDisplayView.svelte";
import ProjectView from "../Project/ProjectDisplayView.svelte";
import NewProjectView from "../Project/ProjectCreateView.svelte";
import SettingsView from "../Settings/SettingsDisplayView.svelte";
import type { ProjectModelBuilder } from "$src/Project/utils/ProjectModelBuilder";

export enum WindowStates {
    Dasboard,
    Settings,
    ProjectCreation,
    Project,
    Portfolio
}

class StateHolder {
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

    const switchToPortfolio = async (portfolio: PortfolioModel) => {
        await portfolio.loadProjects();
        updateTo(new StateHolder(WindowStates.Portfolio, portfolio));
    }

    const updateTo = (newState: StateHolder) => {
        update(state => {
            newState._prevValue = state.value;
            newState._prevWindowState = state.windowState;
            return newState;
        })
    }

    return {
		subscribe,
        switchToSettings: (s: SettingsModel) => updateTo(new StateHolder(WindowStates.Settings, s)),
        switchToProjectCreation: (p: ProjectModelBuilder) => updateTo(new StateHolder(WindowStates.ProjectCreation, p)),
        switchToProject: (p: ProjectModel) => updateTo(new StateHolder(WindowStates.Project, p)),
        switchToPortfolio,
        update,
		reset: () => set(new StateHolder(WindowStates.Dasboard, null)),
	};
}