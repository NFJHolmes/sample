<script lang="ts">
    import type { InsightsType, InsightType } from '$lib/llm/openai/structure';
    import { getBreadcrumbState } from '$lib/state/breadcrumb-state.svelte';
    import { getOpenAIState } from '$lib/state/openai-state.svelte';
    import { Feed } from '@repo/components/feed';
    import { SampleSVG } from '@repo/components/graphics';
    import * as Accordion from '@repo/ui/accordion';
    import * as Avatar from '@repo/ui/avatar';
    import { Button } from '@repo/ui/button';
    import * as Card from '@repo/ui/card';
    import { Loader } from '@repo/ui/loader';
    import { ArrowRight } from 'lucide-svelte';
    import type { FileObject } from 'openai/resources/files.mjs';

    const _openAI = getOpenAIState();
    const _breadcrumbs = getBreadcrumbState();
    _breadcrumbs.setBreadcrumbs([{ label: 'Insights' }]);

    const filesAndInsights: { file: FileObject; insights?: InsightsType }[] = $derived.by(() => {
        return _openAI.files.map((file) => {
            const insights = _openAI.insights.get(file.id);
            return { file, insights };
        });
    });
</script>

<Feed class="s1:pt-0 justify-start pt-0">
    {#if filesAndInsights.length === 0}
        <Button href="/files" variant="link">Please upload files to gather insights</Button>
    {:else}
        <h1 class="sr-only">File Insights</h1>
        {#each filesAndInsights as { file, insights }}
            <section class="flex w-full items-center justify-center">
                <Card.Root class="w-full space-y-4 p-4 shadow-md">
                    <h2 class="text-lg font-semibold">
                        {file.filename}
                    </h2>
                    <div class="flex flex-col gap-y-4">
                        {#if insights}
                            <div class="flex items-end gap-x-2">
                                <Avatar.Root class="h-8 w-8 rounded-lg">
                                    <Avatar.Fallback class="bg-primary/10 rounded-full p-1.5"
                                        ><SampleSVG /></Avatar.Fallback>
                                </Avatar.Root>
                                <p class="bg-muted rounded-md rounded-bl-none p-4">
                                    {insights.summary}
                                </p>
                            </div>
                            <Accordion.Root type="single" class="w-full">
                                {@render insight(insights.insights.first, 1)}
                                {@render insight(insights.insights.second, 2)}
                                {@render insight(insights.insights.third, 3)}
                            </Accordion.Root>
                        {:else}
                            <div class="text-muted-foreground flex items-center gap-x-2 text-sm">
                                <Loader /> Generating insights, please wait...
                            </div>
                        {/if}
                    </div>

                    <Button
                        href={`/insights/${file.id}`}
                        variant="ghost"
                        Icon={ArrowRight}
                        iconRight
                        class="">Learn More</Button>
                </Card.Root>

                {#snippet insight(insight: InsightType, i: number)}
                    <Accordion.Item value={`${i}`}>
                        <Accordion.Trigger>{i}. {insight.title}</Accordion.Trigger>
                        <Accordion.Content>{insight.explanation}</Accordion.Content>
                    </Accordion.Item>
                {/snippet}
            </section>
        {/each}
    {/if}
</Feed>
