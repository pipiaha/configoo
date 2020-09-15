import request from '@/utils/request'

export function fetchList(query) {
    return request({
        url: '/api/form/page',
        method: 'get',
        params: query
    })
}

export function saveTemplate(form) {
    return request({
        url: '/api/form',
        method: 'post',
        data: form
    })
}

export function updateTemplate(form) {
    return request({
        url: '/api/form',
        method: 'put',
        data: form
    })
}

export function removeTemplate(id) {
    return request({
        url: '/api/form/' + id,
        method: 'delete',
    })
}