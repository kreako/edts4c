const { it, expect } = require('@playwright/test')

it('is able to navigate to eleves', async ({ page }) => {
  await page.goto('http://0.0.0.0:8080/')
  await Promise.all([
    page.click('data-test-id=menu-eleves-link'),
    page.waitForNavigation()
  ])
  const url = await page.url()
  expect(url).toBe('http://0.0.0.0:8080/#/eleves/')
})

it('is showing headers on eleve table', async ({ page }) => {
  // Navigate
  await page.goto('http://0.0.0.0:8080/#/eleves')
  // Check headers
  expect(await page.$eval(
    '[data-test-id=eleves-table] thead > tr > :nth-child(1)',
    x => x.innerText)
  ).toBe('Prénom')
  expect(await page.$eval(
    '[data-test-id=eleves-table] thead > tr > :nth-child(2)',
    x => x.innerText)
  ).toBe('Nom')
  expect(await page.$eval(
    '[data-test-id=eleves-table] thead > tr > :nth-child(3)',
    x => x.innerText)
  ).toBe('Date de naissance')
  expect(await page.$eval(
    '[data-test-id=eleves-table] thead > tr > :nth-child(4)',
    x => x.innerText)
  ).toBe('Entrée à l\'école')
  expect(await page.$eval(
    '[data-test-id=eleves-table] thead > tr > :nth-child(5)',
    x => x.innerText)
  ).toBe('Cycle')
  expect(await page.$eval(
    '[data-test-id=eleves-table] thead > tr > :nth-child(6)',
    x => x.innerText)
  ).toBe('')
})

it('is filtering on C1', async ({ page }) => {
  // Navigate
  await page.goto('http://0.0.0.0:8080/#/eleves')

  // Click on everything but C1
  await page.click('[aria-label="C2"] > .q-toggle__inner')
  await page.click('[aria-label="C3"] > .q-toggle__inner')
  await page.click('[aria-label="C4"] > .q-toggle__inner')

  // Now check cycle cell content starts with C1
  const cycles = await page.$$eval(
    '[data-test-id=eleves-table] tbody > tr > :nth-child(5)',
    a => a.map(x => x.innerText))
  expect(cycles.map(c => c.startsWith('C1')).every(x => x)).toBeTruthy()
})

it('is filtering on C2', async ({ page }) => {
  // Navigate
  await page.goto('http://0.0.0.0:8080/#/eleves')

  // Click on everything but C2
  await page.click('[aria-label="C1"] > .q-toggle__inner')
  await page.click('[aria-label="C3"] > .q-toggle__inner')
  await page.click('[aria-label="C4"] > .q-toggle__inner')

  // Now check cycle cell content starts with C2
  const cycles = await page.$$eval(
    '[data-test-id=eleves-table] tbody > tr > :nth-child(5)',
    a => a.map(x => x.innerText))
  expect(cycles.map(c => c.startsWith('C2')).every(x => x)).toBeTruthy()
})

it('is filtering on C3', async ({ page }) => {
  // Navigate
  await page.goto('http://0.0.0.0:8080/#/eleves')

  // Click on everything but C3
  await page.click('[aria-label="C1"] > .q-toggle__inner')
  await page.click('[aria-label="C2"] > .q-toggle__inner')
  await page.click('[aria-label="C4"] > .q-toggle__inner')

  // Now check cycle cell content starts with C3
  const cycles = await page.$$eval(
    '[data-test-id=eleves-table] tbody > tr > :nth-child(5)',
    a => a.map(x => x.innerText))
  expect(cycles.map(c => c.startsWith('C3')).every(x => x)).toBeTruthy()
})

it('is filtering on C4', async ({ page }) => {
  // Navigate
  await page.goto('http://0.0.0.0:8080/#/eleves')

  // Click on everything but C4
  await page.click('[aria-label="C1"] > .q-toggle__inner')
  await page.click('[aria-label="C2"] > .q-toggle__inner')
  await page.click('[aria-label="C3"] > .q-toggle__inner')

  // Now check cycle cell content starts with C4
  const cycles = await page.$$eval(
    '[data-test-id=eleves-table] tbody > tr > :nth-child(5)',
    a => a.map(x => x.innerText))
  expect(cycles.map(c => c.startsWith('C4')).every(x => x)).toBeTruthy()
})

it('is able to create/delete an eleve', async ({ page }) => {
  // Navigate
  await page.goto('http://0.0.0.0:8080/#/eleves')

  // Check that Babibou is not in the list
  // :nth-child(1) because first name is the first column
  expect(await page.$$eval(
    '[data-test-id=eleves-table] tbody > tr > :nth-child(1)',
    trs => trs.findIndex(x => x.innerText === 'Babibou'))
  ).toBe(-1)

  // Create a new eleve with the modal by filling all the fields
  await page.click('text="Nouvel élève"')
  await page.fill('input[data-test-id="eleves-new-firstname"]', 'Babibou')
  await page.fill('input[data-test-id="eleves-new-lastname"]', 'Bibibou')
  await page.fill('input[data-test-id="eleves-new-birthdate"]', '1988-03-12')
  await page.fill('input[data-test-id="eleves-new-schoolentry"]', '2020-05-13')
  await page.click('button[role="button"] >> text="C1"')
  await page.click('data-test-id=eleves-new-save')

  // Now check that Babibou is here and filled with correct informations
  // Wait for the row to appear
  await page.waitForSelector('text=Babibou')
  // find the index of the corresponding row
  const index = await page.$$eval(
    '[data-test-id=eleves-table] tbody > tr > :nth-child(1)',
    trs => trs.findIndex(x => x.innerText === 'Babibou'))
  expect(index).not.toBe(-1)

  // Check infos
  // firstname index + 1 because nth-child is 1 based
  expect(await page.$eval(
    `[data-test-id=eleves-table] tbody > :nth-child(${index + 1}) > :nth-child(1)`,
    x => x.innerText)
  ).toBe('Babibou')
  // lastname
  expect(await page.$eval(
    `[data-test-id=eleves-table] tbody > :nth-child(${index + 1}) > :nth-child(2)`,
    x => x.innerText)
  ).toBe('Bibibou')
  // birthdate
  expect(await page.$eval(
    `[data-test-id=eleves-table] tbody > :nth-child(${index + 1}) > :nth-child(3)`,
    x => x.innerText)
  ).toBe('1988-03-12')
  // school entry
  expect(await page.$eval(
    `[data-test-id=eleves-table] tbody > :nth-child(${index + 1}) > :nth-child(4)`,
    x => x.innerText)
  ).toBe('2020-05-13')
  // cycle
  expect(await page.$eval(
    `[data-test-id=eleves-table] tbody > :nth-child(${index + 1}) > :nth-child(5)`,
    x => x.innerText)
  ).toBe('C1(C4)')

  // Now delete
  // first click more
  await page.click(
    `[data-test-id=eleves-table] tbody > :nth-child(${index + 1}) [data-test-id=eleves-more]`)
  // then click delete
  await page.click('text=/.*Supprimer.*/')

  // Check that Babibou is not displayed anymore
  await page.waitForSelector('text=Babibou', { state : 'hidden' })
})
