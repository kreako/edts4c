export function domainFromId (state) {
  function _domainFromId (domainId) {
    for (const domain of state.domains) {
      if (domain.id === domainId) {
        return domain
      }
    }
    return null
  }
  return _domainFromId
}

export function componentFromId (state) {
  function _componentFromId (componentId) {
    for (const component of state.components) {
      if (component.id === componentId) {
        return component
      }
    }
    return null
  }
  return _componentFromId
}

export function competencyFromId (state) {
  function _competencyFromId (competencyId) {
    for (const competency of state.competencies) {
      if (competency.id === competencyId) {
        return competency
      }
    }
    return null
  }
  return _competencyFromId
}
