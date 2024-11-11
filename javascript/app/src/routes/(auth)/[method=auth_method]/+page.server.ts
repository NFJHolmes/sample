import type { MetaTagsProps } from 'svelte-meta-tags';
import { superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';

import { emailAuthSchema } from '$lib/schemas';

import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
    const { method } = params;
    const methodString = method === 'login' ? 'Log in' : 'Sign up';

    const pageMetaTags: MetaTagsProps = {
        title: methodString,
    };

    return {
        pageMetaTags,
        method,
        emailAuthForm: await superValidate(zod(emailAuthSchema)),
    };
};
