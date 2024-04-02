# {{plugin_name}}

# Config plugin.json

| **Key** | **Default** | **Type**        | **Description**                                                                                                                                               | **Example**                        |
|---------|-------------|-----------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------|
| build   | undefined   | string          | defines a command, which should be run instead of the CLI Build command. If specified it will run this command  **instead** of the default CLI build command! | `"build": "bun run ./build.js`     |
| files   | [ ]          | Array\<string\> | defines additional assets, which are also needed to be bundled.                                                                                               | `"files": ["icon.svg", "assets/"]` |
