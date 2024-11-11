<script lang="ts">
    import { SampleSVG } from '@repo/components/graphics';
    import { SplitScreen } from '.';

    interface Props {
        children?: import('svelte').Snippet;
    }

    let { children }: Props = $props();
</script>

<SplitScreen>
    {#snippet left()}
        <div class="w-full max-w-96 space-y-3">
            <div class="flex w-full justify-center lg:hidden">
                <SampleSVG class="h-16 w-16" />
            </div>
            {@render children?.()}
        </div>
    {/snippet}
    {#snippet right()}
        <div class="flex items-center justify-center">
            <SampleSVG> class="h-32 w-32" />
        </div>
    {/snippet}
</SplitScreen>
