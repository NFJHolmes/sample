<script lang="ts">
    import { page } from '$app/stores';
    import { getOpenAIState } from '$lib/state/openai-state.svelte';
    import type { NavGroup } from '@repo/types';
    import * as Collapsible from '@repo/ui/collapsible';
    import * as Sidebar from '@repo/ui/sidebar';
    import ChevronRight from 'lucide-svelte/icons/chevron-right';

    let {
        group,
    }: {
        group: NavGroup;
    } = $props();

    const _openAI = getOpenAIState();
</script>

<Sidebar.Group>
    <Sidebar.GroupLabel>{group.title}</Sidebar.GroupLabel>
    <Sidebar.Menu>
        {#each group.items as item (item.title)}
            <Collapsible.Root open={item.isActive} class="group/collapsible">
                {#snippet child({ props })}
                    <Sidebar.MenuItem {...props}>
                        <Collapsible.Trigger>
                            {#snippet child({ props })}
                                <Sidebar.MenuButton {...props}>
                                    {#snippet tooltipContent()}
                                        {item.title}
                                    {/snippet}
                                    {#if item.icon}
                                        <item.icon />
                                    {/if}
                                    <span>{item.title}</span>
                                    <ChevronRight
                                        class="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90" />
                                </Sidebar.MenuButton>
                            {/snippet}
                        </Collapsible.Trigger>
                        <Collapsible.Content>
                            {#if item.items}
                                <Sidebar.MenuSub>
                                    {#each item.items as subItem (subItem.title)}
                                        <Sidebar.MenuSubItem>
                                            <Sidebar.MenuSubButton
                                                isActive={$page.route.id === subItem.url}>
                                                {#snippet child({ props })}
                                                    <a href={subItem.url} {...props}>
                                                        <span class="flex w-full justify-between">
                                                            <span>{subItem.title}</span>
                                                            {#if subItem.title === 'Files'}
                                                                <span
                                                                    class="text-muted-foreground/50"
                                                                    >{_openAI.files.length}</span>
                                                            {/if}
                                                        </span>
                                                    </a>
                                                {/snippet}
                                            </Sidebar.MenuSubButton>
                                        </Sidebar.MenuSubItem>
                                    {/each}
                                </Sidebar.MenuSub>
                            {/if}
                        </Collapsible.Content>
                    </Sidebar.MenuItem>
                {/snippet}
            </Collapsible.Root>
        {/each}
    </Sidebar.Menu>
</Sidebar.Group>
