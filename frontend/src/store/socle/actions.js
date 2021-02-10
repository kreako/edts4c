import axios from 'axios'

// Domains

export async function loadDomains (context) {
  context.commit('setLoading', true)
  const response = await axios.get('/api/domains/all')
  const domains = response.data.domains
  context.commit('setDomains', domains)
  context.commit('setLoading', false)
}

export async function setDomainTitle (context, { domainId, title }) {
  await axios.put(`/api/domains/set_title/${domainId}`, { title: title })
  context.commit('setDomainTitle', { domainId, title })
}

export async function setDomains (context, domains) {
  await axios.put('/api/domains/set_domains_rank', { domains: domains })
  context.commit('setDomains', domains)
}

export async function removeDomain (context, domainId) {
  await axios.delete(`/api/domains/delete/${domainId}`)
  context.commit('removeDomain', domainId)
}

export async function addDomain (context, title) {
  const response = await axios.post('/api/domains/append', { title: title })
  const domain = response.data.domain
  context.commit('addDomain', { domainId: domain.id, rank: domain.rank, title: domain.title })
}

// Components

export async function loadComponents (context, domainId) {
  context.commit('setLoading', true)
  const response = await axios.get(`/api/components/by_domain_id/${domainId}`)
  context.commit('setDomain', response.data.domain)
  context.commit('setNextDomainId', response.data.next_domain_id)
  context.commit('setPreviousDomainId', response.data.previous_domain_id)
  context.commit('setComponents', response.data.components)
  context.commit('setLoading', false)
}

export async function setComponents (context, components) {
  await axios.put('/api/components/set_components_rank', { components: components })
  context.commit('setComponents', components)
}

export async function setComponentTitle (context, { componentId, title }) {
  await axios.put(`/api/components/set_title/${componentId}`, { title: title })
  context.commit('setComponentTitle', { componentId, title })
}

export async function removeComponent (context, componentId) {
  await axios.delete(`/api/components/delete/${componentId}`)
  context.commit('removeComponent', componentId)
}

export async function addComponent (context, { domainId, title }) {
  const response = await axios.post('/api/components/append', { domain_id: domainId, title: title })
  const component = response.data.component
  context.commit('addComponent', { componentId: component.id, rank: component.rank, title: component.title, domainId: domainId })
}

export async function moveComponent (context, { componentId, domainId }) {
  await axios.put('/api/components/move_to_domain', { domain_id: domainId, component_id: componentId })
  // No commit, the view will use router to change url so load will happen
}

// Competencies

export async function loadCompetencies (context, componentId) {
  context.commit('setLoading', true)
  const response = await axios.get(`/api/competencies/by_component_id/${componentId}`)
  context.commit('setDomain', response.data.domain)
  context.commit('setComponent', response.data.component)
  context.commit('setNextComponentId', response.data.next_component_id)
  context.commit('setPreviousComponentId', response.data.previous_component_id)
  context.commit('setCompetencies', response.data.competencies)
  context.commit('setLoading', false)
}

export async function setCompetencies (context, competencies) {
  await axios.put('/api/competencies/set_competencies_rank', { competencies: competencies })
  context.commit('setCompetencies', competencies)
}

export async function setCompetencyTitle (context, { competencyId, title }) {
  await axios.put(`/api/competencies/set_title/${competencyId}`, { title: title })
  context.commit('setCompetencyTitle', { competencyId, title })
}

export async function addCompetency (context, { componentId, title }) {
  const response = await axios.post('/api/competencies/append', { component_id: componentId, title: title })
  const competency = response.data.competency
  context.commit('addCompetency', { competencyId: competency.id, rank: competency.rank, title: competency.title, componentId: componentId })
}

export async function removeCompetency (context, competencyId) {
  await axios.delete(`/api/competencies/delete/${competencyId}`)
  context.commit('removeCompetency', competencyId)
}

export async function loadDomainsWithComponents (context) {
  const response = await axios.get('/api/socle/toc')
  context.commit('setDomainsWithComponents', response.data.toc)
}

export async function moveCompetency (context, { competencyId, componentId }) {
  await axios.put('/api/competencies/move_to_component', { competency_id: competencyId, component_id: componentId })
  // No commit, the view will use router to change url so load will happen
}

export async function loadCompetency (context, competencyId) {
  context.commit('setLoading', true)
  const response = await axios.get(`/api/competencies/by_id/${competencyId}`)
  context.commit('setDomain', response.data.domain)
  context.commit('setComponent', response.data.component)
  context.commit('setCompetency', response.data.competency)
  context.commit('setNextCompetencyId', response.data.next_competency_id)
  context.commit('setPreviousCompetencyId', response.data.previous_competency_id)
  context.commit('setLoading', false)
}

export async function setCompetencyCycle (context, { competencyId, cycle, text }) {
  await axios.put(`/api/competencies/update_cycle/${cycle.toLowerCase()}/${competencyId}`, { text: text })
  context.commit('setCompetencyCycle', { cycle: cycle, text: text })
}

/// Below is not used anymore ?

export async function loadComponent (context, componentId) {
  context.commit('setLoading', true)
  const response = await axios.get(`/api/socle/component/${componentId}`)
  const component = response.data.component
  context.commit('setComponent', component)
  context.commit('setLoading', false)
}

export async function setTitle (context, { id, title, type }) {
  if (type === 'domains') {
    await axios.put(`/api/domains/set_title/${id}`, { title: title })
    context.commit('updateDomainTitle', { id: id, title: title })
  } else if (type === 'components') {
    await axios.put(`/api/components/set_title/${id}`, { title: title })
    context.commit('updateComponentTitle', { id: id, title: title })
  } else if (type === 'competencies') {
    await axios.put(`/api/competencies/set_title/${id}`, { title: title })
    context.commit('updateCompetencyTitle', { id: id, title: title })
  } else {
    throw new Error(`Invalid type : ${type}`)
  }
}
export async function load (context) {
  context.commit('setLoading', true)
  const response = await axios.get('/api/socle/toc')
  const socle = response.data.toc
  context.commit('setSocle', socle)
  context.commit('setLoading', false)
}
