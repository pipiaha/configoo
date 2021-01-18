import Router from 'vue-router'
import Home from "../Home";
import FileSelector from "../FileSelector";

export default new Router({
    mode: 'history',
    routes: [

        {
            // 主页
            path: '/',
            component: Home,
            name: 'home',
        },
        {
            // 上传文件
            path: '/file_select',
            component: FileSelector,
            name: 'file_select',
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