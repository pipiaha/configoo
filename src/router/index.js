import Router from 'vue-router'
import Home from "../components/wizard/Home";
import FileSelector from "../components/wizard/FileSelector";
import TypeSelector from "../components/wizard/TypeSelector";
import SettingView from "../components/wizard/SettingView";
import HeadPatternSelector from "../components/wizard/HeadPatternSelector";

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
            // 选择文件类型
            path: '/meta_pattern',
            component: HeadPatternSelector,
            name: 'meta_pattern',
        },
        {
            // 上传文件
            path: '/file_select',
            component: FileSelector,
            name: 'file_select',
        },
        {
            // 预览
            path: '/setting_view',
            component: SettingView,
            name: 'setting_view',
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