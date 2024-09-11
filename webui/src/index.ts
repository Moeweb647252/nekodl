import { createApp } from "vue";
import App from "./App.vue";
import { router } from "./router";
import "./index.css";
import { createPinia } from "pinia";
import piniaPluginPresistedState from "pinia-plugin-persistedstate";

const pinia = createPinia();
pinia.use(piniaPluginPresistedState);

createApp(App).use(pinia).use(router).mount("#root");
