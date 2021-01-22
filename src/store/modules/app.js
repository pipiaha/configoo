const state = {

    mods: [
        {
            path: "/home",
        },
        {
            path: "/type_select",
        },
        {
            path: "/file_select",
        },
    ],
    currentModule: {
        index: 0,
        title: 'configoo',
        onLoad: null,
        beforeSubmit: null,
        afterSubmit: null,
    },
    setting: {
        fileType: null,
        fileMultiUpload: false,

    },
}

const mutations = {}

const actions = {}

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
