<script lang="ts">
    import { dev } from '$app/environment';
    import { invalidate } from '$app/navigation';
    import { page } from '$app/stores';
    import { setSupabaseState } from '$lib/state/supabase-state.svelte';
    import { TailwindIndicator } from '@repo/components/misc';
    import { isDesktop } from '@repo/core';
    import '@repo/core/style';
    import { ModeWatcher } from 'mode-watcher';
    import { onMount } from 'svelte';
    import { MetaTags, deepMerge } from 'svelte-meta-tags';
    import { Toaster } from 'svelte-sonner';
    import '../app.postcss';
    import type { LayoutData } from './$types';

    interface Props {
        data: LayoutData;
        children: import('svelte').Snippet;
    }

    let { data, children }: Props = $props();
    let { session, supabase } = $derived(data);

    // svelte-ignore state_referenced_locally
    const _supabase = setSupabaseState(supabase, session);

    onMount(() => {
        const { data } = supabase.auth.onAuthStateChange((_, newSession) => {
            if (newSession?.expires_at !== session?.expires_at) {
                invalidate('supabase:auth');
                _supabase.session = newSession;
            }
        });

        return () => data.subscription.unsubscribe();
    });

    let metaTags = $derived(deepMerge(data.baseMetaTags, $page.data.pageMetaTags || {}));
</script>

<ModeWatcher defaultMode="system" />
<Toaster richColors closeButton position={isDesktop.matches ? 'bottom-right' : 'top-center'} />
{#if dev}
    <TailwindIndicator />
{/if}
<MetaTags {...metaTags} />

{@render children()}
