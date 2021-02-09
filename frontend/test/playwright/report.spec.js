const { it, expect } = require('@playwright/test');

it('is able to navigate to report', async ({ page }) => {
  await page.goto('http://0.0.0.0:8080');
  await Promise.all([
    page.click('data-test-id=menu-report-link'),
    page.waitForNavigation()
  ])
  const url = await page.url()
  expect(url).toBe('http://0.0.0.0:8080/#/rapports/')
});

