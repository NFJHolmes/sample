<script lang="ts">
    import { AppSidebar } from '$lib/components/nav';
    import Breadcrumb from '$lib/components/nav/breadcrumb.svelte';
    import { nav } from '$lib/config/nav';
    import { setBreadcrumbState } from '$lib/state/breadcrumb-state.svelte';
    import { setFileUploadState } from '$lib/state/file-upload-state.svelte';
    import { setOpenAIState } from '$lib/state/openai-state.svelte';
    import { SampleSVG } from '@repo/components/graphics';
    import { Loader } from '@repo/ui/loader';
    import { ScrollArea } from '@repo/ui/scroll-area';
    import { Separator } from '@repo/ui/separator';
    import * as Sidebar from '@repo/ui/sidebar';

    let { data, children } = $props();

    const _openAI = setOpenAIState(data.ip);
    setFileUploadState(_openAI);
    const _breadcrumbs = setBreadcrumbState();

    let header = $state<HTMLElement | null>(null);
</script>

<div class="relative flex min-h-screen flex-col" id="page">
    {#if !_openAI.ready}
        <div class="flex h-screen w-full flex-col items-center justify-center">
            <SampleSVG class="fill-foreground s0:w-24 mb-10 w-16 drop-shadow-md" />
            <Loader />
            <p>Setting up assistant...</p>
        </div>
    {:else}
        <Sidebar.Provider>
            <AppSidebar {nav} />
            <Sidebar.Inset>
                <header
                    bind:this={header}
                    class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12">
                    <div class="flex items-center gap-2 px-4">
                        <Sidebar.Trigger class="-ml-1" />
                        <Separator orientation="vertical" class="mr-2 h-4" />
                        <Breadcrumb {_breadcrumbs} />
                    </div>
                </header>
                <main class="flex flex-1 flex-col p-0">
                    <ScrollArea class="flex flex-1 flex-col" orientation="vertical">
                        <div
                            style={`min-height: calc(100vh - ${header?.offsetHeight}px);`}
                            class="flex flex-1 flex-col p-4">
                            {@render children?.()}
                        </div>
                    </ScrollArea>
                </main>
            </Sidebar.Inset>
        </Sidebar.Provider>
    {/if}
</div>
