<script lang="ts">
    import type { PageServerData } from './$types';

    import ArrowLeftIcon from 'lucide-svelte/icons/arrow-left';

    import { BrandedSplitScreen } from '$lib/components/layouts';
    import { cn, historyBack } from '@repo/core';
    import { buttonVariants } from '@repo/ui/button';

    import AuthForm from '../auth/auth-form.svelte';

    interface Props {
        data: PageServerData;
    }

    let { data }: Props = $props();
    let { method, emailAuthForm } = $derived(data);
</script>

<div class="absolute left-3 top-3 z-50">
    <button
        class={cn(
            buttonVariants({ variant: 'ghost', size: 'icon' }),
            'text-muted-foreground rounded-full',
        )}
        use:historyBack>
        <ArrowLeftIcon />
    </button>
</div>

<BrandedSplitScreen>
    <AuthForm {method} form={emailAuthForm} />
</BrandedSplitScreen>
