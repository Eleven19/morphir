import { defineConfig } from 'vite'
import elmPlugin from 'vite-plugin-elm'

let rollupOptions = {};
if (process.argv.includes('--examples')) {
    rollupOptions = {
        input: ['./examples/hello.ts'],
        output: {
            format: 'esm',
            entryFileNames: '[name].js',
        }
    }
} else {
    rollupOptions = {
        input: ['./index.ts'],
        output: {
            format: 'esm',
            entryFileNames: '[name].js',
        }
    }
}

export default defineConfig({
    plugins: [elmPlugin()],
    build: {
        rollupOptions: rollupOptions
    }
})