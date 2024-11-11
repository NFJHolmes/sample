<script lang="ts">
    import { type Snippet } from 'svelte';

    interface Props {
        left?: Snippet;
        right?: Snippet;
    }

    let { left, right }: Props = $props();
</script>

<div class="relative flex min-h-dvh">
    <div class="bg-secondary flex w-full items-center justify-center p-3">
        {@render left?.()}
    </div>
    <div class="hidden w-full items-center justify-center p-3 lg:flex">
        {@render right?.()}
    </div>
</div>
