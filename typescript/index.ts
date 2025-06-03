console.log(`Bun server listening on localhost:${process.env.PORT ?? "3000"}`);
Bun.serve({
	port: parseInt(process.env.PORT ?? "3000"),
	fetch(req) {
		return new Response("Hello from typescript plugin template");
	},
	routes: {
		"/": () => {
			return new Response(Bun.file("./index.html"));
		},
	},
});
