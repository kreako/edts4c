export function setLoading (state, loading) {
  state.loading = loading
}

export function setEleves (state, eleves) {
  state.eleves = eleves
}

export function setEleve (state, eleve) {
  // workaround because vue reactivity do not work with array
  const eleves = []
  for (let i = 0; i < state.eleves.length; i++) {
    if (eleve.id === state.eleves[i].id) {
      eleves.push(eleve)
    } else {
      eleves.push(state.eleves[i])
    }
  }
  state.eleves = eleves
}
