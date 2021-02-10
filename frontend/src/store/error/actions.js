export async function setError (context, message) {
  // TODO record it to the backend
  context.commit('setInError', true)
  context.commit('setMessage', message)
}

export async function clearError (context) {
  context.commit('setInError', false)
  context.commit('setMessage', '')
}
