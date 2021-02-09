
const routes = [
  {
    path: '/print',
    component: () => import('layouts/EmptyLayout.vue'),
    children: [
      { path: 'rapport/:detail/:type', component: () => import('pages/Report.vue') }
    ]
  },
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      { path: '', component: () => import('pages/Index.vue') },
      { path: 'socle/', component: () => import('pages/SocleDomains.vue') },
      { path: 'socle/domain/:domainId', component: () => import('pages/SocleComponents.vue') },
      { path: 'socle/domain/:domainId/component/:componentId', component: () => import('pages/SocleCompetencies.vue') },
      { path: 'socle/domain/:domainId/component/:componentId/competency/:competencyId', component: () => import('pages/SocleCompetency.vue') },
      { path: 'eleves/', component: () => import('pages/Eleves.vue') },
      { path: 'support/', component: () => import('pages/Index.vue') },
      { path: 'rapports/', component: () => import('pages/Reports.vue') },
      { path: 'evaluations/', component: () => import('pages/Evaluations.vue') },
      { path: 'evaluation/:cycle/:componentId', component: () => import('pages/Evaluation.vue') }
    ]
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '*',
    component: () => import('pages/Error404.vue')
  }
]

export default routes
