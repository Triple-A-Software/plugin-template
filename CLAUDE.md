# CLAUDE.md — Neleto plugin

Guidance for building this Neleto plugin. See the developer docs
(`/docs/developer/plugins`) and `plugin-cli/docs/plugin-api-routing.md` for the
full reference.

## Plugin API routing (read before wiring an admin UI to `api` endpoints)

- Neleto proxies your `api` endpoints at `/api/rest/plugins/<name>/api/<route>`.
  The `/api/rest/plugins/<name>/api/` part is a **fixed gateway prefix**;
  `<route>` is matched **verbatim** against the routes you declared under `api`
  in `plugin.json`, then forwarded to your binary unchanged.
- So `/api` can appear **twice** in the client URL, depending on how you named
  your own routes:
  - manifest route `/settings` → call `/api/rest/plugins/<name>/api/settings`
  - manifest route `/api/settings` → call `/api/rest/plugins/<name>/api/api/settings`

  Keep the manifest route, your server route, and the client fetch path in sync.
- Canonical admin-UI helper (the UI is served at `/api/rest/plugins/<name>/ui`):

  ```js
  const base = location.pathname.replace(/\/ui\/?(index\.html)?$/, "");
  const api = (p) => base + "/api" + p; // adds the gateway's fixed /api prefix
  fetch(api("/settings")); // pass your plugin's OWN route (incl. its /api, if any)
  ```
- A **404** on a plugin API call is **always a path mismatch, never the HTTP
  method.** The proxy forwards every method and body; a method your server
  doesn't handle returns **405**, not 404. Don't switch `PUT`→`POST` to "fix" a 404.

## Access control

- Restrict an `api` route with **`allowed_permissions`** — an array of CMS
  permission ids (e.g. `page.update`) with **OR** semantics (user needs any one).
  Omit it (or use `[]`) to allow every logged-in user.
- **`allowed_roles` is deprecated and ignored** by the backend — it is not in the
  manifest schema. A route declaring only `allowed_roles` has **no authorization
  check** and is open to all logged-in users. Always use `allowed_permissions`.

## Calling external APIs

- Surface **nested / per-action** error status, not just the top-level envelope —
  otherwise you get blank messages with no detail.
- Field/column allowlists are **all-or-nothing**: one invalid field can make the
  upstream reject the whole request. Only request documented fields.

## Release

- Bump the version in `plugin.json` (and your language's manifest, e.g.
  `Cargo.toml`) for every release so Neleto treats it as a new install.
- Package with `plugin-cli package`.
