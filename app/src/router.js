import Vue from 'vue';
import Router from 'vue-router';

import Vote from './views/Vote.vue';
import Confirmation from './views/Confirmation.vue';
import Results from './views/Results.vue';
import Home2 from "./views/Home2";
import Welcome from "./views/Welcome";

Vue.use(Router);

const router = new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
      // {
      //     path: '/',
      //     name: 'home',
      //     component: Home2,
      // },
    {
      path: '/',
      name: 'welcome',
      component: Welcome,
    },
    {
      path: '/vote',
      name: 'vote',
      component: Vote,
    },
    {
      path: '/confirm',
      name: 'confirm',
      component: Confirmation,
    },
    {
      path: '/results',
      name: 'results',
      component: Results,
    }
  ],
});

router.replace({
  path: '*',
  redirect: '/home',
});

export default router;
