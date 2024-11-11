import type { Config } from 'tailwindcss';

import sharedConfig from '@repo/config-tailwind/tailwind.config';

const config: Pick<Config, 'content' | 'presets'> = {
    content: [
        './src/**/*.{html,js,svelte,ts,svx}',
        '../../packages/ui/lib/**/*.{html,js,svelte,ts}',
        '../../packages/components/lib/**/*.{html,js,svelte,ts}',
        '../../packages/forms/lib/**/*.{html,js,svelte,ts}',
    ],
    presets: [sharedConfig],
};

export default config;
