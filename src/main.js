import Vue from 'vue'
import App from './App.vue'
import VueRouter from 'vue-router';
import router from './router';
import store from './store'
import 'element-ui/lib/theme-chalk/index.css'
import uploader from 'vue-simple-uploader'

import ElementUI from 'element-ui'
import locale from 'element-ui/lib/locale/lang/en' // lang i18n

Vue.use(VueRouter);
Vue.config.productionTip = false
Vue.use(ElementUI, { locale })
Vue.use(uploader)

new Vue({
  router,
  store,
  render: h => h(App),
}).$mount('#app')
