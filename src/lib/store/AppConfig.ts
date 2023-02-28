import type { AppConfig } from "$lib/types/AppConfig";
import { writable } from "svelte/store";

/**
 * The current state and data view for all config settings around the application.
 */
export const ApplicationConfigurationState = writable<AppConfig | null>();
