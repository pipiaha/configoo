const state = {

    templates: [],
}

const mutations = {

    fetchFormTemplates: (state, pageNumber, pageSize) => {
        console.log(pageSize);
    }
}

const actions = {

}

export default {
    namespaced: true,
    state,
    mutations,
    actions
}
