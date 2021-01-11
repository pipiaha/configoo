import Router from 'vue-router'
import Home from "../Home";

export default new Router({
    mode: 'history',
    routes: [

        {
            // 主页
            path: '/',
            component: Home,
            name: 'home',
        },
        // {
        //     // FAQ
        //     path: '/faq',
        //     component: WikiSearchPanel,
        //     name: 'faq',
        // },
        // {
        //     // Wiki
        //     path: '/wiki',
        //     component: WikiPanel,
        //     name: 'wiki',
        // },
        // {
        //     // 客服
        //     path: '/support/:pid/:token/:appId',
        //     component: SupportPanel,
        //     name: 'support',
        // },
        // {
        //     // Editor
        //     path: '/editor',
        //     component: Editor,
        //     name: 'editor',
        // },
    ]
})