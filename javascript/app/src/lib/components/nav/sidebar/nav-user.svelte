<script lang="ts">
    import { goto } from '$app/navigation';
    import { getSupabaseState } from '$lib/state/supabase-state.svelte';
    import { IconSize } from '@repo/core';
    import * as Avatar from '@repo/ui/avatar';
    import { buttonVariants } from '@repo/ui/button';
    import * as DropdownMenu from '@repo/ui/dropdown-menu';
    import * as Sidebar from '@repo/ui/sidebar';
    import { useSidebar } from '@repo/ui/sidebar';
    import { MoonStar, Sun } from 'lucide-svelte';
    import BadgeCheck from 'lucide-svelte/icons/badge-check';
    import Bell from 'lucide-svelte/icons/bell';
    import ChevronsUpDown from 'lucide-svelte/icons/chevrons-up-down';
    import CreditCard from 'lucide-svelte/icons/credit-card';
    import LogIn from 'lucide-svelte/icons/log-in';
    import LogOut from 'lucide-svelte/icons/log-out';
    import Sparkles from 'lucide-svelte/icons/sparkles';
    import { toggleMode } from 'mode-watcher';

    const sidebar = useSidebar();

    let _supabase = getSupabaseState();
    const user = $derived.by(() => {
        return _supabase?.session?.user;
    });

    async function signOut() {
        await _supabase.signOut();
    }
</script>

{#if user}
    <Sidebar.Menu>
        <Sidebar.MenuItem>
            <DropdownMenu.Root>
                <DropdownMenu.Trigger>
                    {#snippet child({ props })}
                        <Sidebar.MenuButton
                            size="lg"
                            class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
                            {...props}>
                            <Avatar.Root class="h-8 w-8 rounded-lg">
                                <Avatar.Fallback class="rounded-lg"
                                    >{user.email?.charAt(0)}</Avatar.Fallback>
                            </Avatar.Root>
                            <div class="grid flex-1 text-left text-sm leading-tight">
                                <span class="truncate text-xs">{user.email}</span>
                            </div>
                            <ChevronsUpDown class="ml-auto size-4" />
                        </Sidebar.MenuButton>
                    {/snippet}
                </DropdownMenu.Trigger>
                <DropdownMenu.Content
                    class="w-[--bits-dropdown-menu-anchor-width] min-w-56 rounded-lg"
                    side={sidebar.isMobile ? 'bottom' : 'right'}
                    align="end"
                    sideOffset={4}>
                    <DropdownMenu.Label class="p-0 font-normal">
                        <div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
                            <Avatar.Root class="h-8 w-8 rounded-lg">
                                <Avatar.Fallback class="rounded-lg"
                                    >{user.email?.charAt(0)}</Avatar.Fallback>
                            </Avatar.Root>
                            <div class="grid flex-1 text-left text-sm leading-tight">
                                <span class="truncate text-xs">{user.email}</span>
                            </div>
                        </div>
                    </DropdownMenu.Label>
                    <DropdownMenu.Separator />
                    <DropdownMenu.Group>
                        <DropdownMenu.Item>
                            <Sparkles />
                            Upgrade to Pro
                        </DropdownMenu.Item>
                    </DropdownMenu.Group>
                    <DropdownMenu.Separator />
                    <DropdownMenu.Group>
                        <DropdownMenu.Item>
                            <BadgeCheck />
                            Account
                        </DropdownMenu.Item>
                        <DropdownMenu.Item>
                            <CreditCard />
                            Billing
                        </DropdownMenu.Item>
                        <DropdownMenu.Item>
                            <Bell />
                            Notifications
                        </DropdownMenu.Item>
                    </DropdownMenu.Group>
                    <DropdownMenu.Separator />
                    <DropdownMenu.Item onclick={toggleMode}>
                        <Sun class="scale-100 transition-all dark:scale-0" />
                        <MoonStar class="absolute scale-0 transition-all dark:scale-100" />
                        Theme
                    </DropdownMenu.Item>
                    <DropdownMenu.Separator />
                    <DropdownMenu.Item onclick={signOut}>
                        <LogOut />
                        Log out
                    </DropdownMenu.Item>
                </DropdownMenu.Content>
            </DropdownMenu.Root>
        </Sidebar.MenuItem>
    </Sidebar.Menu>
{:else}
    <Sidebar.Menu>
        <Sidebar.MenuItem onclick={() => goto('/auth')} class={buttonVariants({ variant: 'ghost' })}
            ><span class="flex items-center gap-x-2"
                >{#if sidebar.open}
                    <span>Sign in</span>
                {/if}
                <LogIn size={IconSize.SMALL} />
            </span>
        </Sidebar.MenuItem>
    </Sidebar.Menu>
{/if}
