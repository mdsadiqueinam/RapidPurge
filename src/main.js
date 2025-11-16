import "./assets/main.css";

import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import i18n from "./locales";
import { createHead } from "@unhead/vue/client";

const app = createApp(App);
const head = createHead();

app.use(router);
app.use(i18n);
app.use(head);

app.mount(document.body);
