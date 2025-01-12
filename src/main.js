import { createApp } from "vue";
// import './style.css'
import App from "./App.vue";
import tray_init from "./tray.js";


// 初始化系统托盘
tray_init()
createApp(App).mount("#app");
