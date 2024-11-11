<script lang="ts">
    import { getBreadcrumbState } from '$lib/state/breadcrumb-state.svelte';

    const _breadcrumbs = getBreadcrumbState();
    _breadcrumbs.setBreadcrumbs([{ label: 'Settings' }]);
</script>

<h1>WIP</h1>
