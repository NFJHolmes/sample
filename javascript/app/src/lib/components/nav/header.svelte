<script lang="ts">
    import { SampleSVG } from '@repo/components/graphics';
    import { LightSwitch } from '@repo/components/light-switch';
    import { cn, IconSize } from '@repo/core';
    import { Button } from '@repo/ui/button';
    import { Upload } from 'lucide-svelte';

    export let fadeHeader = false;
</script>

<div class="min-h-16 w-[100vw] md:min-h-24"></div>
<header
    class={cn(
        'absolute top-0 z-10 flex w-full items-center gap-4 border-b px-4 transition-all md:px-6',
        fadeHeader
            ? 'bg-card h-16 min-h-16'
            : 'bg-card md:bg-background h-16 min-h-16 md:h-24 md:min-h-24 md:border-transparent',
    )}>
    <nav
        class="flex w-full items-center justify-between gap-6 text-lg font-medium md:gap-5 md:text-sm lg:gap-6">
        <a href="/" class="flex items-center gap-2 text-lg font-semibold md:text-base">
            <div class="w-10">
                <SampleSVG />
            </div>
            <span class="text-3xl">Sample<span class="text-primary">.ai</span></span>
        </a>
        <div class="juftify-center flex items-center gap-x-2">
            <LightSwitch />
            <Button class="gap-x-2"><Upload size={IconSize.MEDIUM} />Upload</Button>
        </div>
    </nav>
</header>
