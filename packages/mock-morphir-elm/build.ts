// NOTE: This isn't working... will have to come back to this later
import ElmPlugin from "esbuild-plugin-elm";

const watch = process.argv.includes('--watch')
const isProd = process.env.NODE_ENV === 'production'

const elmOptions = {
    debug: true,
    optimize: isProd,
    clearOnWatch: watch,
    verbose: true,
};

const elmPlugin = ElmPlugin(elmOptions);

Bun.build({
    entrypoints: ["./index.ts"],
    outdir: "./out",
    plugins: [elmPlugin],
});