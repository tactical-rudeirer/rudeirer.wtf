import Vue from 'vue';
import Router from 'vue-router';
import Home from './views/Home.vue';

Vue.use(Router);

export default new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home,
    },
    {
      path: '/about',
      name: 'about',
      // route level code-splitting
      // this generates a separate chunk (about.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import(/* webpackChunkName: "about" */ './views/About.vue'),
    },
    {
      path: '/news',
      name: 'news',
      component: () => import(/* webpackChunkName: "news" */ './views/News.vue'),
    },
    {
      path: '/evaluations',
      name: 'evaluations',
      component: () => import(/* webpackChunkName: "evaluations" */ './views/Evaluations.vue'),
    },
    {
      path: '/compilerbau2',
      name: 'compilerbau2',
      component: () => import(/* webpackChunkName: "compilerbau2" */ './views/Compilerbau2.vue'),
    },
    {
      path: '/login',
      name: 'login',
      component: () => import(/* webpackChunkName: "login" */ './views/Login.vue'),
    },
    {
      path: '/games',
      name: 'games',
      component: () => import(/* webpackChunkName: "games" */ './views/Games.vue'),
    },
    {
      path: '/quiz',
      name: 'quiz',
      component: () => import(/* webpackChunkName: "quiz" */ './views/Quiz.vue'),
    },
    {
      path: '/boerse',
      name: 'stocks',
      component: () => import(/* webpackChunkName: "stocks" */ './views/Stocks.vue'),
    },
    {
      path: '/404',
      name: '404',
      component: () => import(/* webpackChunkName: "notfound" */ './views/NotFound.vue'),
    },
    {
      path: '*',
      component: () => import(/* webpackChunkName: "notfound" */ './views/NotFound.vue'),
    },
  ],
});
