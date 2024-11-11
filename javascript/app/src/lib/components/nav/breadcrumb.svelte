<script lang="ts">
    import { BreadcrumbState } from '$lib/state/breadcrumb-state.svelte';
    import { isDesktop } from '@repo/core';
    import * as Breadcrumb from '@repo/ui/breadcrumb';
    import { buttonVariants } from '@repo/ui/button';
    import * as Drawer from '@repo/ui/drawer';
    import * as DropdownMenu from '@repo/ui/dropdown-menu';

    const { _breadcrumbs }: { _breadcrumbs: BreadcrumbState } = $props();
    let open = $state(false);
</script>

{#if _breadcrumbs.items.length > 0}
    <Breadcrumb.Root>
        <Breadcrumb.List>
            <!-- Case for exactly one breadcrumb -->
            {#if _breadcrumbs.items.length === 1}
                <Breadcrumb.Item>
                    <Breadcrumb.Link href={_breadcrumbs.items[0].href}>
                        {_breadcrumbs.items[0].label}
                    </Breadcrumb.Link>
                </Breadcrumb.Item>

                <!-- Case for exactly two breadcrumbs -->
            {:else if _breadcrumbs.items.length === 2}
                {#each _breadcrumbs.items as item, index}
                    <Breadcrumb.Item>
                        {#if item.href}
                            <Breadcrumb.Link href={item.href}>{item.label}</Breadcrumb.Link>
                        {:else}
                            <Breadcrumb.Page>{item.label}</Breadcrumb.Page>
                        {/if}
                    </Breadcrumb.Item>
                    {#if index === 0}
                        <Breadcrumb.Separator />
                    {/if}
                {/each}

                <!-- Case for three items -->
            {:else if _breadcrumbs.items.length === 3}
                {#each _breadcrumbs.items as item, index}
                    <Breadcrumb.Item>
                        {#if item.href}
                            <Breadcrumb.Link href={item.href}>{item.label}</Breadcrumb.Link>
                        {:else}
                            <Breadcrumb.Page>{item.label}</Breadcrumb.Page>
                        {/if}
                    </Breadcrumb.Item>
                    {#if index < 2}
                        <Breadcrumb.Separator />
                    {/if}
                {/each}

                <!-- Case for more than three items -->
            {:else}
                <!-- First item -->
                <Breadcrumb.Item>
                    <Breadcrumb.Link href={_breadcrumbs.items[0].href}>
                        {_breadcrumbs.items[0].label}
                    </Breadcrumb.Link>
                </Breadcrumb.Item>
                <Breadcrumb.Separator />

                <!-- Ellipsis menu for middle items -->
                <Breadcrumb.Item>
                    {#if isDesktop.matches}
                        <DropdownMenu.Root bind:open>
                            <DropdownMenu.Trigger aria-label="Toggle menu">
                                <Breadcrumb.Ellipsis class="size-4" />
                            </DropdownMenu.Trigger>
                            <DropdownMenu.Content>
                                {#each _breadcrumbs.items.slice(1, -1) as item}
                                    <DropdownMenu.Item>
                                        <a href={item.href || '#'}>{item.label}</a>
                                    </DropdownMenu.Item>
                                {/each}
                            </DropdownMenu.Content>
                        </DropdownMenu.Root>
                    {:else}
                        <Drawer.Root bind:open>
                            <Drawer.Trigger aria-label="Toggle Menu">
                                <Breadcrumb.Ellipsis class="size-4" />
                            </Drawer.Trigger>
                            <Drawer.Content>
                                <Drawer.Header>
                                    <Drawer.Title>Navigate to</Drawer.Title>
                                </Drawer.Header>
                                <div class="grid gap-1 px-4">
                                    {#each _breadcrumbs.items.slice(1, -1) as item}
                                        <a href={item.href || '#'} class="py-1 text-sm">
                                            {item.label}
                                        </a>
                                    {/each}
                                </div>
                                <Drawer.Footer>
                                    <Drawer.Close class={buttonVariants({ variant: 'outline' })}>
                                        Close
                                    </Drawer.Close>
                                </Drawer.Footer>
                            </Drawer.Content>
                        </Drawer.Root>
                    {/if}
                </Breadcrumb.Item>

                <Breadcrumb.Separator />

                <!-- Last item -->
                <Breadcrumb.Item>
                    <Breadcrumb.Page>
                        {_breadcrumbs.items[_breadcrumbs.items.length - 1].label}
                    </Breadcrumb.Page>
                </Breadcrumb.Item>
            {/if}
        </Breadcrumb.List>
    </Breadcrumb.Root>
{/if}
