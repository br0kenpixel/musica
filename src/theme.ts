import { ThemeInstance } from 'vuetify';

export function setTheme(vuetifyTheme: ThemeInstance, name: string) {
    vuetifyTheme.global.name.value = name;
    document.querySelector("body")!.setAttribute("data-bs-theme", name);
}