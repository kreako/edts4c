describe('Menu', () => {
  beforeEach(() => {
    cy.visit('/')
  })

  it('.should() - assert that evaluations link is correct', () => {
    cy.dataCy('menu-evaluations-link').click()
    cy.testRoute('/evaluations')
  })

  it('.should() - assert that eleves link is correct', () => {
    cy.dataCy('menu-eleves-link').click()
    cy.testRoute('/eleves')
  })

  it('.should() - assert that report link is correct', () => {
    cy.dataCy('menu-report-link').click()
    cy.testRoute('/rapports')
  })

  it('.should() - assert that socle link is correct', () => {
    cy.dataCy('menu-socle-link').click()
    cy.testRoute('/socle/start')
  })

  it('.should() - assert that support link is correct', () => {
    cy.dataCy('menu-support-link').click()
    cy.testRoute('/support')
  })
})
