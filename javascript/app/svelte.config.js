import adapter from '@sveltejs/adapter-netlify';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

// import { sveltePreprocess } from 'svelte-preprocess';
// import sequence from 'svelte-sequential-preprocessor';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    // Consult https://kit.svelte.dev/docs/integrations#preprocessors
    // for more information about preprocessors
    preprocess: vitePreprocess(),
    // preprocess: sequence([
    //     vitePreprocess(),
    //     // sveltePreprocess({
    //     //     postcss: true,
    //     // }),
    // ]),
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
                // process.env.VITE_BACKEND_BASE_URL,
                // process.env.VITE_BUNNYCDN_IMAGE_URL,
                // process.env.VITE_BUNNYCDN_DOCUMENT_URL,
            ],
            'style-src': ['self', 'unsafe-inline'],
            'img-src': [
                'self',
                'data:',
                'blob:',
                // process.env.VITE_BUNNYCDN_IMAGE_URL
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
