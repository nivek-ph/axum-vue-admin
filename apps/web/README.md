# React Admin Console

`apps/web` is the default Admin Console.

```bash
pnpm install
pnpm dev
```

The API base defaults to `http://127.0.0.1:3000/api` and can be overridden with
`VITE_API_BASE_URL`.

## Verification

```bash
pnpm test
pnpm typecheck
pnpm build
```

The application uses browser-history routes. A production static host must return `index.html`
for unknown non-API paths so direct navigation and reloads work. The Vite development and preview
servers already provide this SPA fallback.
