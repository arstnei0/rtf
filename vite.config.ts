import { defineConfig } from "vite"

export default defineConfig({
	build: {
		lib: {
			entry: { node: "./src/node.ts", web: "./src/web.ts" },
			formats: ["es"],
			fileName: (format, name) =>
				`${name}.${name === "node" ? "cjs" : "mjs"}`,
		},
		target: "esnext",
		minify: true,
	},
})
