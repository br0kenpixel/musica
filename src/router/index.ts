import { createRouter, createWebHistory } from 'vue-router';
import HomeViewVue from '../views/HomeView.vue';

const router = createRouter(
    {
        history: createWebHistory(import.meta.env.BASE_URL),
        routes: [
            {
                path: '/',
                name: 'home',
                component: HomeViewVue
            },
            {
                path: '/history',
                name: 'history',
                component: () => import('../views/HistoryView.vue')
            },
            {
                path: '/library',
                name: 'library',
                component: () => import('../views/LibraryView.vue')
            },
            {
                path: '/about',
                name: 'about',
                component: () => import('../views/AboutView.vue')
            },
            {
                path: '/settings',
                name: 'settings',
                component: () => import('../views/SettingsView.vue')
            }
        ]
    }
);

export default router