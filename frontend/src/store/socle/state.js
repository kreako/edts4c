export default function () {
  return {
    loading: true,
    // List of domains
    domains: [],
    // List of components and attached domain
    components: [],
    domain: {},
    nextDomainId: null,
    previousDomainId: null,
    // List of competencies and attached domain and attached component
    competencies: [],
    component: {},
    nextComponentId: null,
    previousComponentId: null,
    // To move competencies around
    domainsWithComponents: [],
    // Single competency
    competency: {},
    // TODO
    socle: []
  }
}
