import { sveltekit } from '@sveltejs/kit/vite';
import tsconfigPaths from 'vite-tsconfig-paths';
import { defineConfig } from 'vitest/config';

export default defineConfig({
    plugins: [sveltekit(), tsconfigPaths()],
    server: {
        port: 5174,
    },
});
