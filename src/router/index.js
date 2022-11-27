import Vue from 'vue'
import VueRouter from 'vue-router'
import HomeView from '../views/HomeView.vue'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'home',
    component: HomeView
  },
  {
    path: '/generate-password',
    name: 'generate-password',
    component: () => import('../views/GeneratePasswordView.vue')
  },
  {
    path: '/math',
    name: 'math',
    component: () => import('../views/MathView.vue')
  },
  {
    path: '/levenshtein',
    name: 'levenshtein',
    component: () => import('../views/LevenshteinView.vue')
  },
  {
    path: '/about',
    name: 'about',
    component: () => import('../views/AboutView.vue')
  },
]

const router = new VueRouter({
  routes
})

export default router