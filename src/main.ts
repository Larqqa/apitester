import { createApp } from "vue";
// import "./style.css";
import App from "./App.vue";
import devtools from '@vue/devtools'
import "prismjs/themes/prism.css";

if (process.env.NODE_ENV === 'development') {
  devtools.connect('http://localhost', 8098)
}

createApp(App).mount("#app");
