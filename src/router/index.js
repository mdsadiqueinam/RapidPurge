import { createWebHistory, createRouter } from "vue-router";
import { routes } from "vue-router/auto-routes";

const router = createRouter({
  history: createWebHistory(),
  routes,
  // You don't need to pass the routes anymore,
  // the plugin writes it for you 🤖
});

export default router;
