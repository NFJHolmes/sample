import type { Session, SupabaseClient } from '@supabase/supabase-js';
import { getContext, setContext } from 'svelte';

import type { Database } from '@repo/types';

export class SupabaseState {
    client = $state<SupabaseClient<Database>>();
    session = $state<Session | null>();

    constructor(client: SupabaseClient, session?: Session | null) {
        this.client = client;
        this.session = session;
    }

    async signOut() {
        await this.client?.auth.signOut();
    }

    async getOrgs() {
        const userId = this.session?.user?.id;

        if (!this.client || !userId) {
            return [];
        }

        const { data, error } = await this.client.from('organizations').select();

        if (error) {
            console.error('Error fetching orgs:', error);
            return [];
        }

        return data;
    }

    async getTeams() {
        const userId = this.session?.user?.id;

        if (!this.client || !userId) {
            return [];
        }

        const { data, error } = await this.client.from('teams').select();

        if (error) {
            console.error('Error fetching teams:', error);
            return [];
        }

        return data;
    }
}

// Context key for accessing the state
const SUPABASE_STATE_KEY = Symbol('SUPABASE_STATE');

// Set up the context function
export function setSupabaseState(client: SupabaseClient, session?: Session | null): SupabaseState {
    return setContext(SUPABASE_STATE_KEY, new SupabaseState(client, session));
}

// Get the context function
export function getSupabaseState() {
    return getContext<ReturnType<typeof setSupabaseState>>(SUPABASE_STATE_KEY);
}
