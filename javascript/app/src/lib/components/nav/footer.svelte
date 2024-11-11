<script lang="ts">
    import { SampleSVG } from '@repo/components/graphics';
    import { Separator } from '@repo/ui/separator';
</script>

<footer class="bg-muted flex w-full min-w-full flex-col gap-y-8 p-8">
    <div class="max-s0:justify-center flex">
        <a href="/" class="flex w-fit items-center gap-2 text-lg font-semibold md:text-base">
            <div class="w-10">
                <SampleSVG />
            </div>
            <span class="text-3xl">Sample</span>
        </a>
    </div>
    <div
        class="text-muted-foreground max-s0:text-center s0:justify-between flex flex-wrap justify-center gap-4 text-nowrap text-sm">
        <span>Copyright © 2024, Sample</span>
        <span class="flex space-x-4">
            <a href="/" class="hover:text-primary transition-colors">Privacy Policy</a>
            <Separator orientation="vertical" />
            <a href="/" class="hover:text-primary transition-colors">Terms of Use</a>
        </span>
    </div>
</footer>
