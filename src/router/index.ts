import { createRouter, createWebHistory } from 'vue-router';
import HistoryView from '../views/HistoryView.vue';

const router = createRouter(
    {
        history: createWebHistory(import.meta.env.BASE_URL),
        routes: [
            {
                path: '/',
                name: 'home',
                component: HistoryView,
            },
            {
                path: '/history',
                name: 'history',
                component: HistoryView
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