import type { Provider } from '@supabase/supabase-js';
import { error, fail, redirect } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';

import { emailAuthSchema } from '$lib/schemas';

import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
    throw redirect(303, '/login');
};

export const actions: Actions = {
    async email(event) {
        const {
            locals: { supabase },
            url,
        } = event;

        const form = await superValidate(event, zod(emailAuthSchema));

        if (!form.valid) {
            return fail(400, { form });
        }

        const { email } = form.data;

        console.log('REDIRECT TO', url.origin);

        const { error } = await supabase.auth.signInWithOtp({
            email,
            options: {
                shouldCreateUser: true,
                emailRedirectTo: `${url.origin}`,
            },
        });

        if (error) {
            console.error('Error signing in with OTP:', error.name, error.code);
            return redirect(303, '/auth/error');
        }

        return redirect(303, '/auth/verify');
    },

    async oauth(event) {
        const {
            request,
            url,
            locals: { supabase },
        } = event;

        const formData = await request.formData();
        const method = formData.get('method');

        if (typeof method !== 'string') {
            throw error(400, 'Invalid OAuth method provided. Please try again.');
        }

        const { data, error: authError } = await supabase.auth.signInWithOAuth({
            provider: method as Provider,
            options: {
                redirectTo: `${url.origin}`,
            },
        });

        if (authError) {
            console.error('Error signing in with OAuth:', authError);
            throw error(500, authError.message);
        }

        if (data?.url) {
            throw redirect(303, data.url);
        }
    },

    async logout(event) {
        const {
            locals: { supabase },
        } = event;

        try {
            await supabase.auth.signOut();
        } catch {
            throw error(500, `Failed to log out. Please try again.`);
        }
    },
};
