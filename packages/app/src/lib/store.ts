import type { AppConfig } from '$lib/bindings';
import { writable } from 'svelte/store';
import type { AudioAPI } from './Audio';

/** The current state and data view for all config settings around the application. */
export const ApplicationConfigurationState = writable<AppConfig | null>();

export interface AudioState {
	name: string;
	file: string;
	api: AudioAPI;
}