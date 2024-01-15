import { invoke } from "@tauri-apps/api/tauri";
import { Settings, Song } from "./types";

export async function build_type(): Promise<string> {
    return await invoke("build_type");
}

export async function get_settings(): Promise<Settings> {
    return await invoke("get_settings");
}

export async function save_settings(): Promise<boolean> {
    return await invoke("save_settings");
}

export async function update_settings(new_settings: Settings) {
    await invoke("update_settings", { new: new_settings });
}

export async function get_data_dir(): Promise<string> {
    return await invoke("get_app_dir");
}

export async function get_library_path(): Promise<string> {
    return await invoke("get_library_path");
}

export async function get_config_path(): Promise<string> {
    return await invoke("get_config_path");
}

export async function get_library(): Promise<Array<Song>> {
    return await invoke("get_library");
}

export async function reload_library(): Promise<Array<Song>> {
    return await invoke("reload_library");
}

export async function play_track(id: number) {
    await invoke("play_track", { id: id });
}

export async function pause_playback() {
    await invoke("pause_playback");
}

export async function resume_playback() {
    await invoke("resume_playback");
}

export async function stop_playback() {
    await invoke("stop_playback");
}

export async function set_volume(value: number) {
    await invoke("set_volume", { value: value });
}