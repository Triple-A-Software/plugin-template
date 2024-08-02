# {{plugin_name}}

# Config plugin.json

| **Key** | **Default** | **Type**        | **Description**                                                                                                                                               | **Example**                        |
|---------|-------------|-----------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------|
| build   | undefined   | string          | defines a command, which should be run **instead** of the CLI Build command. Output _dir_ of the build **needs to be `build/`**!                                | `"build": "bun run ./build.js`     |
| files   | [ ]          | Array\<string\> | defines additional assets, which are also needed to be bundled.                                                                                              | `"files": ["icon.svg", "assets/"]` |
| type   | undefined          | string | Type of the Plugin: `"service"` or `"module"`                                                                                      				   | `"type": "module"` |
| routes   | undfeined          | Array\<string\> | defines the Routes (endpoints or pages) for the Plugin, works only if the type is set to `"service"`                                                   | `"routes": {"/my/route": {type:"page" }` |
| bin   | undefined          | string | defines the start command to serve the ServicePlugin			                                                                                         | `"bin": "bun run index.ts"` |


## Example config for a Service plugin
```json
{
	"name": "my-plugin",
	"version": "0.0.1",
	"bin": "bun run my-plugin.js",
	"build": "bun build src/server/index.ts --outfile my-plugin.js --minifiy",
	"type": "service",
	"files": ["glw-htmx.js"],
	"routes": {
		"/products/": {
			"type": "page",
			"layout": {
				"name": "Header & Footer",
				"slot": "<slot/>"
			}
		},
		"/api/products": {
			"type": "api"
		}
	}
}

```



