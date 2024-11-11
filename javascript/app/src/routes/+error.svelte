<script lang="ts">
    import { page } from '$app/stores';
    import { Feed } from '@repo/components/feed';
    import * as PageHeader from '@repo/components/page-header';
</script>

<Feed>
    <PageHeader.Root class="w-fit">
        <PageHeader.Heading>{$page.status}</PageHeader.Heading>
        <PageHeader.Description>{$page.error?.message}</PageHeader.Description>
    </PageHeader.Root>
</Feed>
