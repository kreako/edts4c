// import Vue from 'vue'

export function setLoading (state, value) {
  state.loading = value
}

// Domains related

export function fixDomainsRank (state) {
  let idx = 1
  for (const domain of state.domains) {
    domain.rank = idx
    idx += 1
  }
}

export function setDomains (state, domains) {
  state.domains = domains
  fixDomainsRank(state)
}

export function setDomainTitle (state, { domainId, title }) {
  for (const [idx, domain] of state.domains.entries()) {
    if (domain.id === domainId) {
      state.domains[idx].title = title
    }
  }
}

export function removeDomain (state, domainId) {
  let index = -1
  for (const [idx, domain] of state.domains.entries()) {
    if (domain.id === domainId) {
      index = idx
    }
  }
  if (index !== -1) {
    state.domains.splice(index, 1)
  }
  fixDomainsRank(state)
}

export function addDomain (state, { domainId, rank, title }) {
  state.domains.push({
    id: domainId,
    rank: rank,
    title: title
  })
}

export function setDomain (state, domain) {
  state.domain = domain
}

export function setNextDomainId (state, nextDomainId) {
  state.nextDomainId = nextDomainId
}

export function setPreviousDomainId (state, previousDomainId) {
  state.previousDomainId = previousDomainId
}

// Components related

export function fixComponentsRank (state) {
  let idx = 1
  for (const component of state.components) {
    component.rank = idx
    idx += 1
  }
}

export function setComponents (state, components) {
  state.components = components
  fixComponentsRank(state)
}

export function setComponentTitle (state, { componentId, title }) {
  for (const [idx, component] of state.components.entries()) {
    if (component.id === componentId) {
      state.components[idx].title = title
    }
  }
}

export function removeComponent (state, componentId) {
  let index = -1
  for (const [idx, component] of state.components.entries()) {
    if (component.id === componentId) {
      index = idx
    }
  }
  if (index !== -1) {
    state.components.splice(index, 1)
  }
  fixComponentsRank(state)
}

export function addComponent (state, { componentId, rank, title, domainId }) {
  state.components.push({
    id: componentId,
    rank: rank,
    title: title,
    domain_id: domainId
  })
}

export function setComponent (state, component) {
  state.component = component
}

export function setNextComponentId (state, nextComponentId) {
  state.nextComponentId = nextComponentId
}

export function setPreviousComponentId (state, previousComponentId) {
  state.previousComponentId = previousComponentId
}

// Competencies

export function fixCompetenciesRank (state) {
  let idx = 1
  for (const competency of state.competencies) {
    competency.rank = idx
    idx += 1
  }
}

export function setCompetencies (state, competencies) {
  state.competencies = competencies
  fixCompetenciesRank(state)
}

export function setCompetencyTitle (state, { competencyId, title }) {
  for (const [idx, competency] of state.competencies.entries()) {
    if (competency.id === competencyId) {
      state.competencies[idx].title = title
    }
  }
  if ('title' in state.competency) {
    state.competency.title = title
  }
}

export function addCompetency (state, { competencyId, rank, title, componentId }) {
  state.competencies.push({
    id: competencyId,
    rank: rank,
    title: title,
    c1: null,
    c2: null,
    c3: null,
    c4: null,
    component_id: componentId
  })
}

export function removeCompetency (state, competencyId) {
  let index = -1
  for (const [idx, competency] of state.competencies.entries()) {
    if (competency.id === competencyId) {
      index = idx
    }
  }
  if (index !== -1) {
    state.competencies.splice(index, 1)
  }
  fixCompetenciesRank(state)
}

export function setDomainsWithComponents (state, domainsWithComponents) {
  state.domainsWithComponents = domainsWithComponents
}

// Competency

export function setCompetency (state, competency) {
  state.competency = competency
}

export function setCompetencyCycle (state, { cycle, text }) {
  if (cycle === 'C1') {
    state.competency.c1 = text
  } else if (cycle === 'C2') {
    state.competency.c2 = text
  } else if (cycle === 'C3') {
    state.competency.c3 = text
  } else {
    state.competency.c4 = text
  }
}

/// Below is not used anymore ?

export function setSocle (state, socle) {
  state.socle = socle
}

export function updateDomainTitle (state, { id, title }) {
  for (const domain of state.socle) {
    if (domain.id === id) {
      domain.title = title
    }
  }
}

export function updateComponentTitle (state, { id, title }) {
  for (const domain of state.socle) {
    for (const component of domain.components) {
      if (component.id === id) {
        component.title = title
      }
    }
  }
  if (state.component.id === id) {
    state.component.title = title
  }
}

export function updateCompetencyTitle (state, { id, title }) {
  console.log('begin update competency', id, title)
  for (const competency of state.component.competencies) {
    if (competency.id === id) {
      console.log('update competency', id, title)
      competency.title = title
    }
  }
}
