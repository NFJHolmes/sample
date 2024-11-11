<script lang="ts">
    import { type Snippet } from 'svelte';

    interface Props {
        children?: Snippet;
    }

    let { children }: Props = $props();
</script>

<div class="bg-secondary flex min-h-dvh w-full items-center justify-center px-3">
    {@render children?.()}
</div>
