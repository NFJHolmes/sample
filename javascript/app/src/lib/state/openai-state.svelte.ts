import { PUBLIC_OPENAI_API_KEY } from '$env/static/public';
import OpenAI from 'openai';
import { getContext, onMount, setContext } from 'svelte';
import { toast } from 'svelte-sonner';
import { SvelteMap } from 'svelte/reactivity';

import { InsightsResponseFormat, type InsightsType } from '$lib/schemas';

export class OpenAIState {
    files = $state<OpenAI.FileObject[]>([]);
    assistant = $state<OpenAI.Beta.Assistants.Assistant>();
    vector = $state<OpenAI.Beta.VectorStores.VectorStore>();
    ready = $state(false);
    insights = new SvelteMap<string, InsightsType>();

    // TODO: Need to migrate to server APIs and remove this here.
    client = new OpenAI({
        apiKey: PUBLIC_OPENAI_API_KEY,
        dangerouslyAllowBrowser: true,
    });

    constructor(ip: string) {
        onMount(async () => {
            await this.setup(ip);
        });
    }

    async setup(ip: string): Promise<void> {
        this.assistant = await this.getAssistant(ip);
        this.vector = await this.getVectorStore(ip);

        await this.attachVectorStoreToAssistant();

        await this.refreshFiles();

        // this.generateStructuredInsights();

        this.ready = true;
    }

    private async getAssistant(ip: string): Promise<OpenAI.Beta.Assistants.Assistant> {
        const assistants = await this.client.beta.assistants.list();

        let assistantIndex = -1;

        if (assistants.data.length > 0) {
            assistants.data.forEach(async (assistant, i) => {
                const metadata = assistant.metadata as { ip: string };
                const assistantIp = metadata.ip;

                if (assistantIp === ip) {
                    assistantIndex = i;
                }
            });

            if (assistantIndex !== -1) {
                const assistant = assistants.data[assistantIndex];
                console.log('Found assistant with id: ', assistant.id);
                return assistant;
            }
        }

        console.log('Creating new assistant');
        const assistant = await this.client.beta.assistants.create({
            name: 'Brand Insights Assistant',
            metadata: { ip },
            instructions: assistantPrompt,
            model: 'gpt-4o-mini',
            tools: [{ type: 'file_search' }],
        });

        return assistant;
    }

    private async getVectorStore(ip: string): Promise<OpenAI.Beta.VectorStores.VectorStore> {
        const vectorStores = await this.client.beta.vectorStores.list();

        let vectorIndex = -1;

        if (vectorStores.data.length > 0) {
            vectorStores.data.forEach(async (vector, i) => {
                const metadata = vector.metadata as { ip: string };
                const vectorIp = metadata.ip;

                if (vectorIp === ip) {
                    vectorIndex = i;
                }
            });

            if (vectorIndex !== -1) {
                const vectorStore = vectorStores.data[vectorIndex];
                console.log('Found vector store with id: ', vectorStore.id);
                return vectorStore;
            }
        }

        console.log('Creating new vector store');
        const vectorStore = await this.client.beta.vectorStores.create({
            name: 'Uploaded Files',
            metadata: { ip },
        });

        return vectorStore;
    }

    private async attachVectorStoreToAssistant(): Promise<void> {
        if (!this.assistant) {
            console.error('Assistant not found');
            return;
        }

        if (!this.vector) {
            console.error('Vector store not found');
            return;
        }

        if ((this.assistant?.tool_resources?.file_search?.vector_store_ids?.length ?? 0) > 0) {
            return;
        }

        await this.client.beta.assistants.update(this.assistant.id, {
            tool_resources: { file_search: { vector_store_ids: [this.vector.id] } },
        });
    }

    async getFile(fileId: string): Promise<OpenAI.Files.FileObject> {
        return await this.client.files.retrieve(fileId);
    }

    async getFiles(): Promise<OpenAI.Files.FileObject[]> {
        if (!this.vector) {
            console.error('Vector store not found');
            return [];
        }

        const vectorFiles = await this.client.beta.vectorStores.files.list(this.vector.id);

        const files: OpenAI.FileObject[] = [];

        for (const file of vectorFiles.data) {
            const f = await this.client.files.retrieve(file.id);
            files.push(f);
        }

        return files;
    }

    async deleteFile(fileId: string): Promise<OpenAI.FileObject | undefined> {
        const file = this.files.find((f) => f.id === fileId);

        if (!file) return;

        await this.client.files.del(file.id);

        this.files = this.files.filter((f) => f.id !== file.id);

        return file;
    }

    async refreshFiles(): Promise<void> {
        if (!this.vector) {
            console.error('Vector store not found');
            return;
        }

        this.files = await this.getFiles();
    }

    async getCompletion(
        messages: OpenAI.Chat.Completions.ChatCompletionMessageParam[],
    ): Promise<string> {
        const res = await this.client.chat.completions.create({
            // model: 'gpt-4o-2024-08-06',
            model: 'gpt-4o-mini',
            messages,
        });

        return res.choices?.[0].message.content || '';
    }

    async vectorizeFile(file: File): Promise<void> {
        if (!this.vector) {
            console.error('Vector store not found');
            return;
        }

        try {
            const response = await this.client.beta.vectorStores.files.uploadAndPoll(
                this.vector.id,
                file,
            );

            toast.success(`${file.name} uploaded successfully`);

            const uploadedFile = await this.getFile(response.id);

            this.files.push(uploadedFile);

            this.generateStructuredInsight(uploadedFile);
        } catch (error) {
            console.error(error);
            toast.error(error?.error?.message || `Failed to upload ${file.name}`, {
                duration: 5000,
            });
        }
    }

    async vectorizeFiles(files: File[]): Promise<void> {
        if (!this.vector) {
            console.error('Vector store not found');
            return;
        }

        try {
            await this.client.beta.vectorStores.fileBatches.uploadAndPoll(this.vector.id, {
                files,
            });

            toast.success(`Files uploaded successfully`);

            this.refreshFiles();
        } catch (error) {
            console.error(error);
            toast.error(error?.error?.message || 'Failed to upload files', {
                duration: 5000,
            });
        }
    }

    async createThread(
        messages: OpenAI.Beta.Threads.ThreadCreateParams.Message[],
    ): Promise<OpenAI.Beta.Threads.Thread> {
        const thread = await this.client.beta.threads.create({
            messages,
        });

        return thread;
    }

    async addMessageToThread(threadId: string, message: string): Promise<void> {
        await this.client.beta.threads.messages.create(threadId, {
            role: 'user',
            content: message,
        });
    }

    async runThread(threadId: string): Promise<OpenAI.Beta.Threads.Messages.Text | undefined> {
        if (!this.assistant) {
            console.error('Assistant not found');
            return;
        }

        await this.client.beta.threads.runs.createAndPoll(threadId, {
            assistant_id: this.assistant.id,
        });

        return this.getThreadAnswer(threadId);
    }

    async getThreadAnswer(threadId: string): Promise<OpenAI.Beta.Threads.Messages.Text> {
        const messages = await this.client.beta.threads.messages.list(threadId);
        const message = messages.data[0];
        if (message.content[0].type === 'text') {
            return message.content[0].text;
        }

        return { value: '', annotations: [] };
    }

    async runStructuredThread(
        threadId: string,
        structuredCompletionPrompt: string,
        responseFormat: OpenAI.ResponseFormatJSONObject,
    ): Promise<unknown> {
        const response = await this.runThread(threadId);

        if (!response) {
            console.error('No response found');
            return;
        }

        const structuredResponse = await this.getStructuredCompletion(
            response.value,
            structuredCompletionPrompt,
            responseFormat,
        );

        return structuredResponse;
    }

    async getStructuredCompletion(
        message: string,
        structuredCompletionPrompt: string,
        responseFormat: OpenAI.ResponseFormatJSONObject,
    ): Promise<unknown> {
        try {
            const completion = await this.client.beta.chat.completions.parse({
                // model: 'gpt-4o-2024-08-06',
                model: 'gpt-4o-mini',
                messages: [
                    {
                        role: 'system',
                        content: structuredCompletionPrompt,
                    },
                    {
                        role: 'assistant',
                        content: message,
                    },
                ],
                response_format: responseFormat,
            });

            const response = completion.choices?.[0].message;

            if (response.parsed) {
                console.log(response.parsed);
                return response.parsed;
            } else if (response.refusal) {
                // TODO: handle refusal
                console.log(response.refusal);
            }
        } catch (error) {
            // TODO: Handle edge cases
            if (error.constructor.name == 'LengthFinishReasonError') {
                // Retry with a higher max tokens
                console.error('Too many tokens: ', error.message);
            } else {
                // Handle other exceptions
                console.error('An error occurred: ', error.message);
            }
        }
    }

    async generateStructuredInsight(file: OpenAI.FileObject): Promise<InsightsType | undefined> {
        const thread = await this.createThread([
            {
                role: 'user',
                content: `Please provide insights for this file: [id: ${file.id} / name:${file.filename}]. Do not ask any clarifying questions.`,
            },
        ]);

        const response = (await this.getStructuredInsights(thread.id)) as InsightsType;

        if (!response) {
            console.error('No response found');
            return;
        }

        this.insights.set(file.id, response);

        return response;
    }

    async generateStructuredInsights(): Promise<void> {
        this.insights.clear();

        for (const file of this.files) {
            const thread = await this.createThread([
                {
                    role: 'user',
                    content: `Please provide insights for this file: [id: ${file.id} / name:${file.filename}]. Do not ask any clarifying questions.`,
                },
            ]);

            const response = (await this.getStructuredInsights(thread.id)) as InsightsType;

            if (!response) {
                console.error('No response found');
                return;
            }

            this.insights.set(file.id, response);
        }
    }

    // Structured Insights
    async getStructuredInsights(threadId: string): Promise<InsightsResponseFormat | undefined> {
        const response = await this.runThread(threadId);

        if (!response) {
            console.error('No response found');
            return;
        }

        const structuredResponse = (await this.getStructuredCompletion(
            response.value,
            structuredInsightsPrompt,
            InsightsResponseFormat,
        )) as InsightsResponseFormat | undefined;

        return structuredResponse;
    }

    // TODO: Admin - secure this before prod
    async deleteAllResources(): Promise<void> {
        const vectors = await this.client.beta.vectorStores.list();
        const assistants = await this.client.beta.assistants.list();
        const files = await this.client.files.list();

        for (const vector of vectors.data) {
            await this.client.beta.vectorStores.del(vector.id);
        }

        for (const assistant of assistants.data) {
            await this.client.beta.assistants.del(assistant.id);
        }

        for (const file of files.data) {
            await this.client.files.del(file.id);
        }
    }
}

const OPENAI_KEY = Symbol('OPENAI');

export function setOpenAIState(ip: string) {
    return setContext(OPENAI_KEY, new OpenAIState(ip));
}

export function getOpenAIState() {
    return getContext<ReturnType<typeof setOpenAIState>>(OPENAI_KEY);
}

const assistantPrompt = `
Sample Assistant Prompt
`;

const structuredInsightsPrompt = `
Sample Structured Insights Prompt
`;
