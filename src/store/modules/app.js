const state = {

    mods: [
        {
            path: "/home",// 主页
        },
        {
            path: "/type_select",// 输入文件类型选择
        },
        {
            path: "/file_select",// 文件路径
        },
        {
            path: "/setting_view",// 结果预览
        },
    ],
    currentModule: {
        index: 0,
        title: 'configoo',
        // onLoad: null,
        canProceed: null,
        beforeSubmit: null,
        onSubmit: null,
        afterSubmit: null,
    },
    setting: {
        fileType: null,
        fileMultiUpload: false,

    },
}

const mutations = {
    updateSettingValue: function (state, obj) {
        state.setting = Object.assign(state.setting, obj);
    },
    updateModuleValue: function (state, obj) {
        state.currentModule = Object.assign(state.currentModule, obj);
    }
}

const actions = {
    updateModuleIndex: function ({commit}, index) {
        commit('updateModuleValue', {index: index});
    },
    updateModuleValue: function ({commit}, obj) {
        obj.title = obj.title ? obj.title : '';
        obj.beforeSubmit = obj.beforeSubmit ? obj.beforeSubmit : null;
        obj.onSubmit = obj.onSubmit ? obj.onSubmit : null;
        obj.afterSubmit = obj.afterSubmit ? obj.afterSubmit : null;
        commit('updateModuleValue', obj);
    },
    updateSettingValue: function ({commit}, obj) {
        commit('updateSettingValue', obj);
    }
}

// const mutations = {
//     setUserMenus: function (state, data) {
//         state.userMenus = data;
//     },
// };
//
// const actions = {
//     fetchUserMenu: function ({commit}) {
//         userTreeView().then(response => {
//             commit('setUserMenus', response);
//         })
//     },
// };

export default {
    namespaced: true,
    state,
    mutations,
    actions
}
