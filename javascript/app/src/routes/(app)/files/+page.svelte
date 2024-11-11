<script lang="ts">
    import { getBreadcrumbState } from '$lib/state/breadcrumb-state.svelte';
    import { getFileUploadState } from '$lib/state/file-upload-state.svelte';
    import { getOpenAIState } from '$lib/state/openai-state.svelte';
    import { FileManager } from './components';

    const _openAI = getOpenAIState();
    const _fileUpload = getFileUploadState();
    const _breadcrumbs = getBreadcrumbState();
    _breadcrumbs.setBreadcrumbs([{ label: 'Files' }]);
</script>

<div class="flex flex-1 flex-col items-center">
    <FileManager {_openAI} {_fileUpload} />
</div>
