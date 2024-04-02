# {{plugin_name}}

# Config plugin.json

1. `build:string` defines a command, which should be run instead of the CLI Build command. If specified it will run this command **instead** of the default CLI build command!    
    Example: `"build": "bun run ./build.js"`
2. `files:Array<string>` defines additional assets, which are also needed to be bundled.  
     Example `"files": ["icon.svg", "assets/"]`

