import Router from 'vue-router'
import Home from "../Home";
import FileSelector from "../FileSelector";
import TypeSelector from "../TypeSelector";

export default new Router({
    mode: 'history',
    routes: [

        {
            // 主页
            path: '/',
            component: Home,
            name: 'main',
        },
        {
            // home
            path: '/home',
            component: Home,
            name: 'home',
        },
        {
            // 选择文件类型
            path: '/type_select',
            component: TypeSelector,
            name: 'type_select',
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