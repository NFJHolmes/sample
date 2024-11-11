<script>
    import { Center } from '$lib/components/layouts';
    import { SampleSVG } from '@repo/components/graphics';
</script>

<Center>
    <div class="min-h-dvh w-full max-w-md space-y-3 py-16 md:min-h-0 md:text-center">
        <div class="flex w-full md:justify-center">
            <SampleSVG class="s1:w-16 s1:h-16 h-12 w-12" />
        </div>
        <h1 class="text-3xl font-medium tracking-tighter md:text-4xl">
            We have a message for you.
        </h1>
        <p class="text-muted-foreground text-lg">
            Check your email inbox for a message from us. Click the link that is attached.
        </p>
    </div>
</Center>
