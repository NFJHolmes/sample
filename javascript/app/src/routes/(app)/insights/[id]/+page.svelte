<script lang="ts">
    import { goto } from '$app/navigation';
    import { type InsightsType, type InsightType } from '$lib/llm/openai/structure';
    import { getBreadcrumbState } from '$lib/state/breadcrumb-state.svelte';
    import { getOpenAIState } from '$lib/state/openai-state.svelte';
    import { Feed } from '@repo/components/feed';
    import * as Card from '@repo/ui/card';
    import { Loader } from '@repo/ui/loader';
    import { onMount } from 'svelte';

    let { data } = $props();

    const _openAI = getOpenAIState();
    const _breadcrumbs = getBreadcrumbState();

    const file = _openAI.files.find((file) => file.id === data.id);
    const insights: InsightsType | undefined = $state(_openAI.insights.get(data.id));

    _breadcrumbs.setBreadcrumbs([
        { label: 'Insights', href: '/insights' },
        { label: file?.filename || 'File Insights' },
    ]);

    let loading = $state(true);
    onMount(async () => {
        if (!file) {
            console.error('File not found');
            goto('/insights');
            return;
        }
    });

    $effect(() => {
        if (insights) loading = false;
    });
</script>

<Feed>
    {#if !file}
        <Loader />
    {:else if loading}
        <div class="flex flex-col items-center justify-center gap-y-2">
            <Loader />
            <p>Generating insights for {file.filename}...</p>
        </div>
    {:else if !insights}
        <p>Failed to generate insights, please refresh and try again</p>
    {:else}
        <section class="space-y-6 p-4">
            <h1 class="mb-2 text-3xl font-semibold">Insights: {file?.filename}</h1>

            <div class="bg-primary/5 rounded-md p-4 shadow-sm">
                <h2 class="text-xl font-bold">Summary</h2>
                <p class="mt-1">{insights?.summary}</p>
            </div>

            <div class="mt-6 space-y-4">
                {#snippet insight(insight: InsightType, i: number)}
                    <Card.Root class="space-y-4 p-4 shadow-md">
                        <h3 class="text-lg font-semibold">{i}. {insight.title}</h3>
                        <p class="">{insight.explanation}</p>
                        <div class="mt-2 space-y-2">
                            <h4 class="text-md font-semibold">Implementation Steps:</h4>
                            <ul class="list-inside pl-4">
                                {#each insight.implementation as implementation}
                                    <li class="space-y-1">
                                        <p>
                                            <span class="font-semibold"
                                                >Step {implementation.number}:</span>
                                            {implementation.description}
                                        </p>
                                    </li>
                                {/each}
                            </ul>
                        </div>
                    </Card.Root>
                {/snippet}

                {@render insight(insights.insights.first, 1)}
                {@render insight(insights.insights.second, 2)}
                {@render insight(insights.insights.third, 3)}
            </div>
        </section>
    {/if}
</Feed>
