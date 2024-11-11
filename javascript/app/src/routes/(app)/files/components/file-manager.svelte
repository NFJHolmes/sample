<script lang="ts">
    import type { FileUploadState } from '$lib/state/file-upload-state.svelte';
    import type { OpenAIState } from '$lib/state/openai-state.svelte';
    import { cn, IconSize } from '@repo/core';
    import { Button, buttonVariants } from '@repo/ui/button';
    import { Card, CardContent, CardHeader, CardTitle } from '@repo/ui/card';
    import { Input } from '@repo/ui/input';
    import { Progress } from '@repo/ui/progress';
    import * as Select from '@repo/ui/select';
    import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@repo/ui/table';
    import { Tabs, TabsContent, TabsList, TabsTrigger } from '@repo/ui/tabs';
    import * as Tooltip from '@repo/ui/tooltip';
    import {
        CircleHelp,
        // File as FileIcon,
        // FileSpreadsheet,
        // FileText,
        // Image,
        Trash2,
        Upload,
    } from 'lucide-svelte';
    import type { FileObject } from 'openai/resources/files.mjs';
    import { onMount } from 'svelte';
    import { toast } from 'svelte-sonner';

    interface Props {
        _openAI: OpenAIState;
        _fileUpload: FileUploadState;
    }

    let { _openAI, _fileUpload }: Props = $props();

    let dragActive = $state(false);
    let searchTerm = $state('');
    let sortBy = $state('name');
    let sortOrder = $state<'asc' | 'desc'>('asc');
    let deletingFiles = $state<string[]>([]);

    // Used storage in MB
    const usedStorage = $derived(
        _openAI.files.reduce((acc, file) => acc + file.bytes / (1024 * 1024), 0),
    );
    let totalStorage = 10; // Total storage in MB

    function handleDrag(e: DragEvent) {
        e.preventDefault();
        e.stopPropagation();
        if (e.type === 'dragenter' || e.type === 'dragover') {
            dragActive = true;
            if (e.dataTransfer && e.dataTransfer.types.includes('Files')) {
                e.dataTransfer.dropEffect = 'copy';
            }
        } else if (e.type === 'dragleave') {
            dragActive = false;
        }
    }

    function handleDrop(e: DragEvent) {
        e.preventDefault();
        e.stopPropagation();
        dragActive = false;
        if (e.dataTransfer?.files && e.dataTransfer.files[0]) {
            handleFiles(e.dataTransfer.files);
        }
    }

    function handleChange(e: Event) {
        const target = e.target as HTMLInputElement;
        if (target.files) {
            handleFiles(target.files);
        }
    }

    function handleFiles(fileList: FileList) {
        uploadFiles(fileList);
    }

    async function removeFile(fileId: string) {
        deletingFiles.push(fileId);

        const removedFile = await _openAI.deleteFile(fileId);

        deletingFiles = deletingFiles.filter((id) => id !== fileId);

        toast.success(`${removedFile?.filename || 'File'} removed successfully!`);
    }

    function sortFiles(a: FileObject, b: FileObject) {
        let comparison = 0;
        if (sortBy === 'name') {
            comparison = a.filename.localeCompare(b.filename);
        } else if (sortBy === 'size') {
            comparison = a.bytes - b.bytes;
        } else if (sortBy === 'type') {
            let aExt = a.filename.split('.').pop() || '';
            let bExt = b.filename.split('.').pop() || '';
            comparison = aExt.localeCompare(bExt);
        }
        return sortOrder === 'asc' ? comparison : -comparison;
    }

    const filteredFiles = $derived.by(() => {
        return _openAI.files
            .filter((file) => file.filename.toLowerCase().includes(searchTerm.toLowerCase()))
            .sort(sortFiles);
    });

    onMount(() => {
        window.addEventListener('dragover', (e) => e.preventDefault());
        window.addEventListener('drop', (e) => e.preventDefault());
    });

    let fileInput: HTMLInputElement;

    function triggerFileUpload() {
        fileInput.click(); // This triggers the file input click
    }

    function uploadFiles(files: FileList) {
        [...files].forEach((file) => {
            _fileUpload.addFileToUpload(file);
        });
    }

    const completedFiles = $derived(_fileUpload.recentlyCompletedFiles.length);
    const totalFiles = $derived(_fileUpload.files.length + completedFiles);
</script>

<div class="container p-0">
    <Tabs value="upload" class="w-full">
        <TabsList class="grid w-full grid-cols-2">
            <TabsTrigger value="upload">Upload Files</TabsTrigger>
            <TabsTrigger value="manage">Manage Files</TabsTrigger>
        </TabsList>
        <TabsContent value="upload">
            <Card>
                <CardHeader>
                    <CardTitle class="flex items-center gap-x-2"
                        ><span>Upload New Files</span>
                        <Tooltip.Root>
                            <Tooltip.Trigger
                                class={buttonVariants({ variant: 'ghost', size: 'xsicon' })}>
                                <CircleHelp size={IconSize.XSMALL} />
                            </Tooltip.Trigger>
                            <Tooltip.Content class="bg-card text-card-foreground shadow-md">
                                <a
                                    class="hover:text-primary underline"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    href="https://platform.openai.com/docs/assistants/tools/file-search/supported-files"
                                    >See allowed file types</a>
                            </Tooltip.Content>
                        </Tooltip.Root>
                    </CardTitle>
                </CardHeader>
                <CardContent>
                    <div
                        class={cn(
                            'bg-background w-full rounded-lg border-2 border-dashed p-8 transition-all duration-300 ease-in-out',
                            dragActive ? 'border-primary bg-muted' : 'border-muted-foreground',
                        )}
                        ondragenter={handleDrag}
                        ondragleave={handleDrag}
                        ondragover={handleDrag}
                        ondrop={handleDrop}
                        role="searchbox"
                        tabindex="0">
                        <div class="flex h-64 flex-col items-center justify-center space-y-4">
                            <Upload
                                size={IconSize.XLARGE}
                                class="text-primary animate-bounce drop-shadow-md" />
                            <h3
                                class={cn(
                                    'text-lg font-semibold',
                                    dragActive ? 'text-primary drop-shadow-md' : '',
                                )}>
                                Drag and drop files here
                            </h3>
                            <p class="text-muted-foreground text-sm">or</p>
                            <label for="file-upload" class="cursor-pointer">
                                <Button variant="default" onclick={triggerFileUpload}
                                    >Select Files</Button>
                            </label>
                            <input
                                id="file-upload"
                                type="file"
                                multiple
                                class="hidden"
                                accept="
                                    text/x-c,
                                    text/x-c++,
                                    text/x-csharp,
                                    text/css,
                                    application/msword,
                                    application/vnd.openxmlformats-officedocument.wordprocessingml.document,
                                    text/x-golang,
                                    text/html,
                                    text/x-java,
                                    text/javascript,
                                    application/json,
                                    text/markdown,
                                    application/pdf,
                                    text/x-php,
                                    application/vnd.openxmlformats-officedocument.presentationml.presentation,
                                    text/x-python,
                                    text/x-script.python,
                                    text/x-ruby,
                                    application/x-sh,
                                    text/x-tex,
                                    application/typescript,
                                    text/plain
                                "
                                bind:this={fileInput}
                                onchange={handleChange} />
                        </div>
                    </div>

                    {#if totalFiles > 0}
                        <div class="mt-8 flex flex-col items-center justify-center">
                            <p class="text-muted-foreground self-start text-sm">
                                Uploaded ({completedFiles} / {totalFiles}) files...
                            </p>
                            <Progress indeterminate value={(completedFiles / totalFiles) * 100} />
                        </div>
                    {/if}
                </CardContent>
            </Card>
        </TabsContent>
        <TabsContent value="manage">
            <Card>
                <CardHeader>
                    <CardTitle class="py-1">Manage Your Files</CardTitle>
                </CardHeader>
                <CardContent>
                    <div class="mb-4">
                        <h4 class="mb-2 text-lg font-semibold">Storage Usage</h4>
                        <Progress value={(usedStorage / totalStorage) * 100} class="h-2 w-full" />
                        <p class="text-muted-foreground mt-1 text-sm">
                            {usedStorage.toFixed(2)} MB used of {totalStorage} MB
                        </p>
                    </div>
                    <div class="mb-4 flex flex-col items-center justify-between sm:flex-row">
                        <Input
                            type="text"
                            placeholder="Search files..."
                            class="mb-2 sm:mb-0 sm:mr-2"
                            bind:value={searchTerm} />
                        <div class="flex space-x-2">
                            <Select.Root type="single" name="sortBy" bind:value={sortBy}>
                                <Select.Trigger class="w-[180px]">
                                    {sortBy === 'name'
                                        ? 'Name'
                                        : sortBy === 'size'
                                          ? 'Size'
                                          : 'Type'}
                                </Select.Trigger>
                                <Select.Content>
                                    <Select.Group>
                                        <Select.GroupHeading>Sort By</Select.GroupHeading>
                                        <Select.Item value="name">Name</Select.Item>
                                        <Select.Item value="size">Size</Select.Item>
                                        <Select.Item value="type">Type</Select.Item>
                                    </Select.Group>
                                </Select.Content>
                            </Select.Root>
                            <Button
                                variant="outline"
                                onclick={() => (sortOrder = sortOrder === 'asc' ? 'desc' : 'asc')}>
                                {sortOrder === 'asc' ? '▲' : '▼'}
                            </Button>
                        </div>
                    </div>
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead>File Name</TableHead>
                                <TableHead>Type</TableHead>
                                <TableHead>Size</TableHead>
                                <TableHead>Actions</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {#each filteredFiles as file}
                                <TableRow>
                                    <TableCell class="font-medium">{file.filename}</TableCell>
                                    <TableCell>{file.filename.split('.').pop() || ''}</TableCell>
                                    <TableCell
                                        >{(file.bytes / (1024 * 1024)).toFixed(2)} MB</TableCell>
                                    <TableCell>
                                        <div class="flex space-x-2">
                                            <Button
                                                variant="destructive"
                                                size="sm"
                                                disabled={deletingFiles.includes(file.id)}
                                                loading={deletingFiles.includes(file.id)}
                                                onclick={() => removeFile(file.id)}
                                                Icon={Trash2}>
                                                Delete
                                            </Button>
                                        </div>
                                    </TableCell>
                                </TableRow>
                            {/each}
                        </TableBody>
                    </Table>
                </CardContent>
            </Card>
        </TabsContent>
    </Tabs>
</div>

<style>
    @keyframes pulse {
        0%,
        100% {
            opacity: 1;
            transform: scale(1);
        }
        50% {
            opacity: 0.5;
            transform: scale(1.05);
        }
    }
</style>
