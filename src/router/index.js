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
    path: '/about',
    name: 'about',
    component: () => import('../views/AboutView.vue')
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
  }
]

const router = new VueRouter({
  routes
})

export default router