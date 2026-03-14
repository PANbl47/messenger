import { expect, test } from '@playwright/test'

async function createAccount(page: import('@playwright/test').Page) {
  await page.goto('/')
  await page.getByLabel('Display name').fill('Ser Test')
  await page.getByLabel('Username').fill('ser-test')
  await page.getByLabel('Phone').fill('+79990001122')
  await page.getByRole('button', { name: 'Create account' }).click()
}

test('offline send retries automatically after reconnect', async ({ page }) => {
  await createAccount(page)

  await page.getByLabel('Network toggle').uncheck()
  await page.getByLabel('Message draft').fill('Offline alpha message')
  await page.getByRole('button', { name: 'Send', exact: true }).click()

  await expect(page.getByText('queued', { exact: true })).toBeVisible()

  await page.getByLabel('Network toggle').check()

  await expect(page.getByText('delivered')).toBeVisible()
  await expect(
    page.getByText('Queued messages retried automatically.'),
  ).toBeVisible()
})

test('three minute timeout becomes actionable failure state', async ({ page }) => {
  await createAccount(page)

  await page.getByLabel('Network toggle').uncheck()
  await page.getByLabel('Message draft').fill('Needs attention')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  await page.getByRole('button', { name: 'Simulate 3 min timeout' }).click()

  await expect(page.getByText('failed')).toBeVisible()
  await expect(page.getByRole('button', { name: /Retry msg-/ })).toBeEnabled()
})
