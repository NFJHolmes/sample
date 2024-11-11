<script lang="ts">
    import NavGroupSimple from '$lib/components/nav/sidebar/nav-group-simple.svelte';
    import type { Nav } from '@repo/types';
    import * as Sidebar from '@repo/ui/sidebar';
    import type { ComponentProps } from 'svelte';
    import NavUser from './nav-user.svelte';
    import TeamSwitcher from './team-switcher.svelte';

    let {
        ref = $bindable(null),
        collapsible = 'icon',
        nav,
        ...restProps
    }: ComponentProps<typeof Sidebar.Root> & {
        nav: Nav;
    } = $props();
</script>

<Sidebar.Root bind:ref {collapsible} {...restProps}>
    <Sidebar.Header>
        <TeamSwitcher teams={nav.teams} />
    </Sidebar.Header>
    <Sidebar.Content>
        <NavGroupSimple items={nav.items} />
    </Sidebar.Content>
    <Sidebar.Footer>
        <NavUser />
    </Sidebar.Footer>
    <Sidebar.Rail />
</Sidebar.Root>
