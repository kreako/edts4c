export function setLoading (state, value) {
  state.loading = value
}

export function setToc (state, toc) {
  state.toc = toc
}

export function setDetail (state, detail) {
  state.detail = detail
}

export function setGeneralComment (state, comment) {
  state.comment = comment
}

export function setStatus (state, { id, status }) {
  for (const competency of state.detail.competencies) {
    for (const evaluation of competency.evaluations) {
      if (evaluation.id === id) {
        evaluation.status = status
        break
      }
    }
  }
}

export function setComment (state, { id, comment }) {
  for (const competency of state.detail.competencies) {
    for (const evaluation of competency.evaluations) {
      if (evaluation.id === id) {
        evaluation.comment = comment
        break
      }
    }
  }
}

export function setSingleGeneralComment (state, { id, comment }) {
  for (const c of state.comment) {
    if (c.id === id) {
      c.comment = comment
      break
    }
  }
}
