<script lang="ts">
    import { page } from '$app/stores';
    import { getOpenAIState } from '$lib/state/openai-state.svelte';
    import type { NavItem } from '@repo/types';
    import * as Sidebar from '@repo/ui/sidebar';

    let {
        items,
    }: {
        items: NavItem[];
    } = $props();

    const _openAI = getOpenAIState();
</script>

<Sidebar.Group>
    <Sidebar.Menu>
        {#each items as item (item.title)}
            <Sidebar.MenuItem>
                <Sidebar.MenuButton isActive={$page.route.id === item.url}>
                    {#snippet tooltipContent()}
                        {item.title}
                    {/snippet}
                    {#snippet child({ props })}
                        <a href={item.url} {...props}>
                            {#if item.icon}
                                <item.icon />
                            {/if}
                            <span>{item.title}</span>
                            {#if item.title === 'Files'}
                                <span class="text-muted-foreground/50 w-full text-end"
                                    >{_openAI.files.length}</span>
                            {/if}
                        </a>
                    {/snippet}
                </Sidebar.MenuButton>
            </Sidebar.MenuItem>
        {/each}
    </Sidebar.Menu>
</Sidebar.Group>
