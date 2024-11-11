import { getContext, setContext } from 'svelte';

import type { OpenAIState } from '$lib/state/openai-state.svelte';

export interface FileUpload {
    id: string;
    name: string;
    progress: number;
}

interface UploadQueueItem {
    file: File;
    startUpload: () => Promise<void>;
}

export class FileUploadState {
    files = $state<FileUpload[]>([]);
    recentlyCompletedFiles = $state<FileUpload[]>([]);
    private activeUploads: number = 0;
    private maxConcurrentUploads: number = 5;
    private uploadQueue: UploadQueueItem[] = [];

    constructor(private OpenAIState: OpenAIState) {}

    // Adds a file to the upload queue and triggers the toast if needed
    async addFileToUpload(file: File): Promise<void> {
        const newFile: FileUpload = { id: file.name, name: file.name, progress: 0 };
        this.files.push(newFile);

        // Add the upload task to the queue
        this.uploadQueue.push({
            file,
            startUpload: async () => {
                // Can batch w/ vectorizeFiles()
                await this.OpenAIState.vectorizeFile(file);
                this.updateProgress(file.name, 100);
            },
        });

        // Try to start processing the queue
        this.processQueue();
    }

    async addFilesToUpload(files: File[]): Promise<void> {
        files.forEach((file) => {
            this.addFileToUpload(file);
        });
    }

    // Updates progress of a file and handles completion
    updateProgress(fileId: string, progress: number): void {
        this.files.forEach((file) => {
            if (file.id === fileId) {
                file.progress = progress;
            }
        });

        if (progress >= 100) {
            this.handleRecentlyCompletedFile(fileId);
        }
    }

    // Svelte 5 proxy weirdness made this fn hacky
    handleRecentlyCompletedFile(fileId: string): void {
        const index = this.files.findIndex((f) => f.id === fileId);
        if (index < 0) return;

        const file = this.files[index];
        this.files.splice(index, 1);

        this.recentlyCompletedFiles.push(file);
    }

    // Processes the upload queue, ensuring only a set number run concurrently
    private processQueue(): void {
        while (this.activeUploads < this.maxConcurrentUploads && this.uploadQueue.length > 0) {
            const nextItem = this.uploadQueue.shift();
            if (nextItem) {
                this.activeUploads++;
                nextItem.startUpload().finally(() => {
                    this.activeUploads--;
                    this.processQueue(); // Continue processing queue after upload completes
                });
            }
        }
    }

    resetState(): void {
        this.files = [];
    }
}

// Context key for accessing the state
const FILE_UPLOAD_STATE_KEY = Symbol('FILE_UPLOAD_STATE');

// Set up the context function
export function setFileUploadState(OpenAIState: OpenAIState): FileUploadState {
    return setContext(FILE_UPLOAD_STATE_KEY, new FileUploadState(OpenAIState));
}

// Get the context function
export function getFileUploadState(): FileUploadState {
    return getContext<FileUploadState>(FILE_UPLOAD_STATE_KEY);
}
