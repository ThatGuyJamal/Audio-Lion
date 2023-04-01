// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
    interface Window {
        __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
    }
}

const invoke = window.__TAURI_INVOKE__;

export function loadConfig() {
    return invoke<ConfigResult>("load_config")
}

export function saveConfig(config: AppConfig) {
    return invoke<ConfigResult>("save_config", { config })
}

export function resetConfig() {
    return invoke<ConfigResult>("reset_config")
}

export function getAudioFiles(audioFileType: AudioFileTypes) {
    return invoke<string[]>("get_audio_files", { audioFileType })
}

export function getAppInfo() {
    return invoke<AppInfo>("get_app_info")
}

export type AudioFileTypes = "MP3" | "WAV" | "WEBM"
export type ConfigResult = { data: AppConfig; error: IError | null }
export type AppInfo = { os: string; name: string; version: string; description: string }
/**
 *  The configuration file for the application
 */
export type AppConfig = { audio_directories: string[]; audio_device_name: string | null; audio_file_types_allowed: AudioFileTypes[] }
/**
 *  basic Errors returned by the application to the front end.
 */
export type IError = { status: boolean; message: string }
