import axios from 'axios'

export async function loadSocle (context) {
  context.commit('setLoading', true)
  let response = await axios.get('/api/domains/all')
  const domains = response.data.domains
  response = await axios.get('/api/components/all')
  const components = response.data.components
  response = await axios.get('/api/competencies/all')
  const competencies = response.data.competencies
  for (const domain of domains) {
    domain.components = []
  }
  for (const component of components) {
    component.competencies = []
    for (const domain of domains) {
      if (domain.id === component.domain_id) {
        domain.components.push(component)
        break
      }
    }
  }
  for (const competency of competencies) {
    for (const component of components) {
      if (component.id === competency.component_id) {
        component.competencies.push(competency)
        break
      }
    }
  }
  context.commit('setSocle', domains)
  context.commit('setLoading', false)
}

export async function loadReportSummary (context, cycle) {
  context.commit('setLoading', true)
  const rc = await axios.get(`/api/report/summary/${cycle}`)
  context.commit('setReportSummary', rc.data.report)
  context.commit('setLoading', false)
}

export async function loadReportFull (context, cycle) {
  context.commit('setLoading', true)
  const rc = await axios.get(`/api/report/full/${cycle}`)
  context.commit('setReportFull', rc.data.report)
  context.commit('setLoading', false)
}
