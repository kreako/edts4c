import axios from 'axios'

export async function load (context) {
  context.commit('setLoading', true)
  await context.dispatch('loadEvaluationDate')
  await context.dispatch('loadEvaluationStats')
  context.commit('setLoading', false)
}

export async function loadEvaluationDate (context) {
  const response = await axios.get('/api/key_store/evaluation_date')
  const evaluationDate = response.data.evaluation_date
  context.commit('setEvaluationDate', evaluationDate)
}

export async function loadEvaluationStats (context) {
  const rc1 = await axios.get('/api/evaluations/stats')
  context.commit('setStats', rc1.data)
}

export async function setEvaluationDate (context, evaluationDate) {
  await axios.put('/api/key_store/evaluation_date', { evaluation_date: evaluationDate })
  await context.dispatch('loadEvaluationDate')
}
