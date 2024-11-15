import adapter from '@sveltejs/adapter-netlify';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    // Consult https://kit.svelte.dev/docs/integrations#preprocessors
    // for more information about preprocessors
    preprocess: vitePreprocess(),
    kit: {
        // adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
        // If your environment is not supported or you settled on a specific environment, switch out the adapter.
        // See https://kit.svelte.dev/docs/adapters for more information about adapters.
        adapter: adapter(),
        alias: {
            $lib: './src/lib',
            $components: './src/lib/components',
            $utils: './src/lib/utils',
        },
    },
    csp: {
        mode: 'auto',
        directives: {
            'default-src': ['self'],
            'script-src': ['self', 'blob:'],
            'connect-src': [
                'self',
            ],
            'style-src': ['self', 'unsafe-inline'],
            'img-src': [
                'self',
                'data:',
                'blob:',
            ],
            'font-src': ['self'],
            'upgrade-insecure-requests': true,
        },
    },
    csrf: {
        checkOrigin: true,
    },
};

export default config;
