import { createRouter, createWebHashHistory } from "vue-router";
import LoginView from "./views/LoginView.vue";
import MainView from "./views/MainView.vue";
import { useStore } from "./store.ts";

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      name: "login",
      path: "/login",
      component: LoginView,
    },
    {
      name: "main",
      path: "/",
      component: MainView,
    },
  ],
});

router.beforeEach((to, from) => {
  const store = useStore();
  if (store.token || to.path === "/login") {
    return true;
  } else {
    return { path: "/login" };
  }
});
