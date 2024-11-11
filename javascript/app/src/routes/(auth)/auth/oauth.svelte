<script lang="ts">
    import { enhance } from '$app/forms';
    import { faGoogle } from '@fortawesome/free-brands-svg-icons';
    import { Button } from '@repo/ui/button';
    import Fa from 'svelte-fa';
</script>

<form method="POST" action="/auth?/oauth" class="space-y-2" use:enhance>
    <Button class="w-full space-x-1" variant="secondary" type="submit" name="method" value="google">
        <Fa icon={faGoogle} />
        <span> Google </span>
    </Button>
    <!-- <Button class="w-full space-x-1" variant="secondary" type="submit" name="method" value="github">
        <Fa icon={faGithub} />
        <span> GitHub </span>
    </Button> -->
</form>
