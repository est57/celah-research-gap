import { defineConfig } from "@playwright/test";

// Layout tests run against the browser preview, which renders every screen from sample data.
export default defineConfig({
  testDir: "e2e",
  testMatch: "*.e2e.ts",
  timeout: 300_000,
  reporter: process.env.CI ? "list" : "line",
  use: { baseURL: "http://127.0.0.1:1420", browserName: "chromium" },
  webServer: {
    command: "npm run dev",
    url: "http://127.0.0.1:1420",
    reuseExistingServer: !process.env.CI,
    timeout: 60_000,
  },
});
