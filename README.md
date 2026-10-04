# Neleto plugin template

Scaffold for a Neleto plugin. Pick a language directory (`rust`, `typescript`,
`python`, `php`, `go`); `plugin-cli create` renders the `.liquid` files and the
`plugin.json` manifest for you.

## Before you wire up an admin UI — read this

The #1 recurring bug when building plugins is the **two-level `/api` prefix**.
Neleto proxies your `api` endpoints at:

```
/api/rest/plugins/<plugin-name>/api/<your-route>
```

`/api/rest/plugins/<plugin-name>/api/` is a **fixed gateway prefix**, and
`<your-route>` is matched **verbatim** against the routes you declared under
`api` in `plugin.json`. So `/api` can appear **twice** in the client URL:

| Manifest `api` route | Browser must call                           |
| -------------------- | ------------------------------------------- |
| `/settings`          | `/api/rest/plugins/<name>/api/settings`     |
| `/api/settings`      | `/api/rest/plugins/<name>/api/api/settings` |

Canonical admin-UI fetch helper (the UI is served at `.../api/rest/plugins/<name>/ui`):

```js
const base = location.pathname.replace(/\/ui\/?(index\.html)?$/, "");
const api = (p) => base + "/api" + p; // adds the gateway's fixed /api prefix
fetch(api("/settings")); // pass your plugin's OWN route, incl. its /api namespace if any
```

A **404** on a plugin API call is **always a path mismatch, never the HTTP
method** — the proxy forwards every method and body unchanged (a method mismatch
would be a 405). Count the `/api` segments first.

## Access control

Restrict an `api` route with **`allowed_permissions`** (array of CMS permission
ids like `page.update`, OR semantics). Omit it to allow every logged-in user.
**`allowed_roles` is deprecated and ignored** — a route using it has no
authorization check. This template's `plugin.json` uses `allowed_permissions`.

See `CLAUDE.md` in this template and `plugin-cli/docs/plugin-api-routing.md` for
the full write-up.
