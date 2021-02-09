const { it, expect } = require('@playwright/test');

it('is able to navigate to socle', async ({ page }) => {
  await page.goto('http://0.0.0.0:8080/');
  await Promise.all([
    page.click('data-test-id=menu-socle-link'),
    page.waitForNavigation()
  ])
  const url = await page.url()
  expect(url).toBe('http://0.0.0.0:8080/#/socle/')
});
