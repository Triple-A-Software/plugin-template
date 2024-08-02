# {{plugin_name}}

# Config plugin.json

| **Key** | **Default** | **Type**        | **Description**                                                                                                                                               | **Example**                        |
|---------|-------------|-----------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------|
| build   | undefined   | string          | defines a command, which should be run **instead** of the CLI Build command. Output _dir_ of the build **needs to be `build/`**!                                | `"build": "bun run ./build.js`     |
| files   | [ ]          | Array\<string\> | defines additional assets, which are also needed to be bundled.                                                                                              | `"files": ["icon.svg", "assets/"]` |
| type   | undefined          | string | Type of the Plugin: `"service"` or `"module"`                                                                                         | `"type": "module"` |
| routes   | undfeined          | Array\<string\> | defines the Routes (endpoints or pages) for the Plugin, works only if the type is set to `"service"`                                                                                            | `"routes": {"/my/route": {type:"page" }` |


## Example config for a Service plugin
```json
{
	"name": "glw-htmx-plugin",
	"version": "0.0.1",
	"bin": "bun run glw-htmx.js",
	"build": "bun build src/server/index.ts --outfile glw-htmx.js --minifiy",
	"type": "service",
	"files": ["glw-htmx.js"],
	"routes": {
		"/produkte/aderendhuelsen": {
			"type": "page",
			"layout": {
				"name": "Header & Footer",
				"slot": "<slot/>"
			}
		},
		"/api/wire_end_slevees": {
			"type": "api"
		}
	}
}

```



