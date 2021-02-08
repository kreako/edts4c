import axios from 'axios'

export async function loadToc (context, cycle) {
  context.commit('setLoading', true)
  try {
    const rc = await axios.get(`/api/evaluations/toc/${cycle}`)
    context.commit('setToc', rc.data.toc)
  } catch (error) {
    // TODO
    console.error(error)
  }
  context.commit('setLoading', false)
}

export async function loadDetail (context, { cycle, componentId }) {
  context.commit('setLoading', true)
  try {
    const rc = await axios.get(`/api/evaluations/detail/${cycle}/${componentId}`)
    context.commit('setDetail', rc.data.detail)
  } catch (error) {
    // TODO
    console.error(error)
  }
  context.commit('setLoading', false)
}

export async function loadComment (context, cycle) {
  context.commit('setLoading', true)
  try {
    const rc = await axios.get(`/api/general_comments/comment/${cycle}`)
    context.commit('setGeneralComment', rc.data.comment)
  } catch (error) {
    // TODO
    console.error(error)
  }
  context.commit('setLoading', false)
}

export async function setStatus (context, { id, status }) {
  try {
    await axios.put(`/api/evaluations/set_status/${id}`, { status: status })
    context.commit('setStatus', { id: id, status: status })
  } catch (error) {
    // TODO
    console.error(error)
  }
}

export async function setComment (context, { id, comment }) {
  try {
    await axios.put(`/api/evaluations/set_comment/${id}`, { comment: comment })
    context.commit('setComment', { id: id, comment: comment })
  } catch (error) {
    // TODO
    console.error(error)
  }
}

export async function setGeneralComment (context, { id, comment }) {
  try {
    await axios.put(`/api/general_comments/comment/${id}`, { comment: comment })
    context.commit('setSingleGeneralComment', { id: id, comment: comment })
  } catch (error) {
    // TODO
    console.error(error)
  }
}
