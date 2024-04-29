import { defineConfig } from 'vite'
import elmPlugin from 'vite-plugin-elm'

export default defineConfig({
    plugins: [elmPlugin()],
    build: {
        rollupOptions: {
            input: ['./index.ts', './sandbox-api.ts'],
            output: {
                format: 'esm',
                entryFileNames: '[name].js',
            }
        }
    }
})