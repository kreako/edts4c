import axios from 'axios'

export async function load (context) {
  context.commit('setLoading', true)
  try {
    await context.dispatch('loadEvaluationDate')
    await context.dispatch('loadEvaluationStats')
  } catch (error) {
    // TODO
    console.error(error)
  }
  context.commit('setLoading', false)
}

export async function loadEvaluationDate (context) {
  try {
    const response = await axios.get('/api/key_store/evaluation_date')
    const evaluationDate = response.data.evaluation_date
    context.commit('setEvaluationDate', evaluationDate)
  } catch (error) {
    // TODO
    console.error(error)
  }
}

export async function loadEvaluationStats (context) {
  try {
    const rc1 = await axios.get('/api/evaluations/stats')
    context.commit('setStats', rc1.data)
  } catch (error) {
    // TODO
    console.error(error)
  }
}

export async function setEvaluationDate (context, evaluationDate) {
  // TODO error ?
  await axios.put('/api/key_store/evaluation_date', { evaluation_date: evaluationDate })
  await context.dispatch('loadEvaluationDate')
}
