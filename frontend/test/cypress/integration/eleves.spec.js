describe('Eleves', () => {
  beforeEach(() => {
    cy.visit('/#/eleves')
  })

  it('.should() - assert that eleves creation works as expected', () => {
    const firstname = randomString()
    const lastname = randomString()
    const birthdate = '2006-03-25'
    const schoolentry = '2006-03-25'

    // Create the eleve
    cy.dataCy('eleves-new').click()
    cy.dataCy('eleves-new-firstname').should('have.value', '')
    cy.dataCy('eleves-new-firstname').type(firstname)
    cy.dataCy('eleves-new-lastname').should('have.value', '')
    cy.dataCy('eleves-new-lastname').type(lastname)
    cy.dataCy('eleves-new-birthdate').should('have.value', '____-__-__')
    cy.dataCy('eleves-new-birthdate').type(birthdate)
    cy.dataCy('eleves-new-schoolentry').should('have.value', '____-__-__')
    cy.dataCy('eleves-new-schoolentry').type(schoolentry)
    cy.get('[data-cy=eleves-new-cycle] button:nth-child(3)').click()
    cy.dataCy('eleves-new-save').click()

    // Now check cycle cell content
    cy.get('[data-cy=eleves-table] tbody > tr > td').then((td) => {
      const rowCount = td.length / 6
      let found = false
      for (let i = 0; i < rowCount; i++) {
        if (td[i * 6 + 0].innerText === firstname &&
          td[i * 6 + 1].innerText === lastname &&
          td[i * 6 + 2].innerText === birthdate &&
          td[i * 6 + 3].innerText === schoolentry &&
          td[i * 6 + 4].innerText.includes('C3')) {
          expect(td[i * 6 + 0].innerText).to.equal(firstname)
          expect(td[i * 6 + 1].innerText).to.equal(lastname)
          expect(td[i * 6 + 2].innerText).to.equal(birthdate)
          expect(td[i * 6 + 3].innerText).to.equal(schoolentry)
          expect(td[i * 6 + 4].innerText).to.contain('C3')
          found = true
        }
      }
      expect(found).to.be.true
    })

    // Check that new eleve is clear
    cy.dataCy('eleves-new').click()
    cy.dataCy('eleves-new-firstname').should('have.value', '')
    cy.dataCy('eleves-new-lastname').should('have.value', '')
    cy.dataCy('eleves-new-birthdate').should('have.value', '____-__-__')
    cy.dataCy('eleves-new-schoolentry').should('have.value', '____-__-__')
  })

  it('.should() - assert that eleves table has an header', () => {
    cy.get('[data-cy=eleves-table] thead > tr > td').then((td) => {
      expect(td.length).to.equal(6)
      expect(td[0].innerText).to.equal('Prénom')
      expect(td[1].innerText).to.equal('Nom')
      expect(td[2].innerText).to.equal('Date de naissance')
      expect(td[3].innerText).to.equal('Entrée à l\'école')
      expect(td[4].innerText).to.equal('Cycle')
      expect(td[5].innerText).to.equal('')
    })
  })

  it('.should() - assert that C1 filter is working', () => {
    // Click on everything but C1
    cy.get('[aria-label="C2"] > .q-toggle__inner').click()
    cy.get('[aria-label="C3"] > .q-toggle__inner').click()
    cy.get('[aria-label="C4"] > .q-toggle__inner').click()
    // Now check cycle cell content
    cy.get('[data-cy=eleves-table] tbody > tr > td').then((td) => {
      const rowCount = td.length / 6
      for (let i = 0; i < rowCount; i++) {
        expect(td[i * 6 + 4].innerText).to.contain('C1')
      }
    })
  })

  it('.should() - assert that C2 filter is working', () => {
    // Click on everything but C2
    cy.get('[aria-label="C1"] > .q-toggle__inner').click()
    cy.get('[aria-label="C3"] > .q-toggle__inner').click()
    cy.get('[aria-label="C4"] > .q-toggle__inner').click()
    // Now check cycle cell content
    cy.get('[data-cy=eleves-table] tbody > tr > td').then((td) => {
      const rowCount = td.length / 6
      for (let i = 0; i < rowCount; i++) {
        expect(td[i * 6 + 4].innerText).to.contain('C2')
      }
    })
  })

  it('.should() - assert that C3 filter is working', () => {
    // Click on everything but C3
    cy.get('[aria-label="C1"] > .q-toggle__inner').click()
    cy.get('[aria-label="C2"] > .q-toggle__inner').click()
    cy.get('[aria-label="C4"] > .q-toggle__inner').click()
    // Now check cycle cell content
    cy.get('[data-cy=eleves-table] tbody > tr > td').then((td) => {
      const rowCount = td.length / 6
      for (let i = 0; i < rowCount; i++) {
        expect(td[i * 6 + 4].innerText).to.contain('C3')
      }
    })
  })

  it('.should() - assert that C4 filter is working', () => {
    // Click on everything but C4
    cy.get('[aria-label="C1"] > .q-toggle__inner').click()
    cy.get('[aria-label="C2"] > .q-toggle__inner').click()
    cy.get('[aria-label="C3"] > .q-toggle__inner').click()
    // Now check cycle cell content
    cy.get('[data-cy=eleves-table] tbody > tr > td').then((td) => {
      const rowCount = td.length / 6
      for (let i = 0; i < rowCount; i++) {
        expect(td[i * 6 + 4].innerText).to.contain('C4')
      }
    })
  })
})

function randomString () {
  return Math.random().toString(36).substring(2, 15)
}
