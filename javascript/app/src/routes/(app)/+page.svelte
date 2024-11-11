<script lang="ts">
    import { getBreadcrumbState } from '$lib/state/breadcrumb-state.svelte';
    import { getOpenAIState } from '$lib/state/openai-state.svelte';
    import { SampleSVG } from '@repo/components/graphics';
    import * as Avatar from '@repo/ui/avatar';
    import { Button } from '@repo/ui/button';
    import { Loader } from '@repo/ui/loader';
    import { ScrollArea } from '@repo/ui/scroll-area';
    import { Textarea } from '@repo/ui/textarea';
    import OpenAI from 'openai';
    import type { Annotation } from 'openai/resources/beta/threads/messages.mjs';
    import { onMount, tick } from 'svelte';
    import { toast } from 'svelte-sonner';

    const _openAI = getOpenAIState();
    const _breadcrumbs = getBreadcrumbState();
    _breadcrumbs.setBreadcrumbs([{ label: 'Chat' }]);

    type Message = {
        role: 'assistant' | 'user';
        content: string;
        annotations: Annotation[];
    };

    const user = { name: 'User' };
    const handWaveEmoji = String.fromCodePoint(0x1f44b);
    const introMessage: Message = {
        role: 'assistant',
        content: `
Hello! ${handWaveEmoji}

Intro message sample text.
        `,
        annotations: [],
    };

    let disableInput = $state(false);
    let messages: Message[] = $state([introMessage]);
    let input = $state('');

    let textarea: HTMLTextAreaElement | null = null;
    let thread: OpenAI.Beta.Threads.Thread | undefined = undefined;

    let loading = $state(true);
    onMount(async () => {
        textarea = document.querySelector('textarea');
        if (textarea) adjustHeight(textarea);

        loading = false;
    });

    async function sendMessage() {
        try {
            if (!_openAI.assistant) {
                toast.error('Assistant not set up, please refresh');
                return;
            }

            disableInput = true;

            let content = input;
            input = '';
            await tick();

            if (textarea) adjustHeight(textarea);

            messages = [...messages, { role: 'user', content, annotations: [] }];

            if (!thread) {
                thread = await _openAI.createThread([
                    { role: introMessage.role, content: introMessage.content },
                    { role: 'user', content },
                ]);
            } else {
                await _openAI.addMessageToThread(thread.id, content);
            }

            const response = await _openAI.runThread(thread.id);

            if (!response) {
                toast.error('Error sending message, please refresh');
                disableInput = false;
                return;
            }

            messages = [
                ...messages,
                { role: 'assistant', content: response.value, annotations: response.annotations },
            ];

            disableInput = false;
        } catch (e) {
            console.error(e);
            toast.error(
                'Error sending message, please refresh or reduce the number of files uploaded',
            );
            disableInput = false;
        }
    }

    function adjustHeight(element: HTMLTextAreaElement) {
        // Set height to 'auto' first to shrink it on delete
        element.style.height = 'auto';
        // Set the height based on the scroll height
        element.style.height = `${element.scrollHeight}px`;
    }
</script>

<div class="flex flex-1 flex-col items-center justify-end">
    <div class="s0:w-24 absolute left-1/2 top-1/2 w-16 -translate-x-1/2 -translate-y-1/2">
        <SampleSVG class="fill-muted" />
    </div>

    {#if loading}
        <div class="flex flex-col items-center justify-center pb-10">
            <Loader />
        </div>
    {:else}
        <div class="bg-muted z-[1] flex w-full flex-col justify-end rounded-md p-4">
            <div class="flex w-full flex-col gap-y-4">
                {#each messages as message, i (i)}
                    {#if message.role === 'user'}
                        <div class="flex items-end gap-x-2 self-end">
                            <div
                                class="bg-background ml-8 flex w-fit self-end rounded-xl rounded-br-none px-4 py-2 text-end">
                                <p>{message.content}</p>
                            </div>
                            <Avatar.Root class="h-8 w-8 rounded-lg">
                                <!-- <Avatar.Image src={user.avatar} alt={user.name} /> -->
                                <Avatar.Fallback class="bg-primary/10 rounded-full"
                                    >{user.name.charAt(0)}</Avatar.Fallback>
                            </Avatar.Root>
                        </div>
                    {:else}
                        <div class="flex items-end gap-x-2">
                            <Avatar.Root class="h-8 w-8 rounded-lg">
                                <Avatar.Fallback class="bg-primary/10 rounded-full p-1.5"
                                    ><SampleSVG /></Avatar.Fallback>
                            </Avatar.Root>
                            <div
                                class="bg-background flex w-fit flex-col gap-y-2 text-wrap rounded-xl rounded-bl-none px-4 py-2 text-start">
                                <pre class="w-fit text-wrap font-sans">{message.content}</pre>
                                {#if message.annotations.length > 0}
                                    <span>{JSON.stringify(message.annotations)}</span>
                                {/if}
                            </div>
                        </div>
                    {/if}
                {/each}
                {#if disableInput}
                    <div class="flex items-end gap-x-2">
                        <Avatar.Root class="h-8 w-8 rounded-lg">
                            <Avatar.Fallback class="bg-primary/10 rounded-full p-1.5"
                                ><SampleSVG /></Avatar.Fallback>
                        </Avatar.Root>
                        <div
                            class="bg-background flex w-fit flex-col gap-y-2 text-wrap rounded-xl rounded-bl-none px-4 py-2 text-start">
                            <Loader />
                        </div>
                    </div>
                {/if}
            </div>
            <form onsubmit={sendMessage} class="mt-4 flex w-full gap-x-4">
                <ScrollArea
                    class="bg-background focus-within:ring-ring w-full rounded-md shadow-inner focus-within:ring-1"
                    viewportClass="max-h-[20dvh]"
                    orientation="vertical"
                    type="scroll">
                    <Textarea
                        bind:value={input}
                        class="min-h-0 resize-none overflow-hidden border-transparent focus:border-transparent focus:ring-0 focus-visible:ring-0"
                        rows={1} />
                </ScrollArea>
                <Button type="submit" class="self-end" disabled={disableInput}>Send</Button>
            </form>
        </div>
    {/if}
</div>
