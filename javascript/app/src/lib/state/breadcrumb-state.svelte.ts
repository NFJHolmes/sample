import { getContext, setContext } from 'svelte';

import type { Breadcrumb } from '@repo/types';

export class BreadcrumbState {
    items = $state<Breadcrumb[]>([]);

    constructor(breadcrumbs: Breadcrumb[]) {
        this.items = breadcrumbs;
    }

    setBreadcrumbs(breadcrumbs: Breadcrumb[]): void {
        this.items = breadcrumbs;
    }
}

// Context key for accessing the state
const BREADCRUMB_STATE_KEY = Symbol('BREADCRUMB_STATE');

// Set up the context function
export function setBreadcrumbState(breadcrumbs: Breadcrumb[] = []): BreadcrumbState {
    return setContext(BREADCRUMB_STATE_KEY, new BreadcrumbState(breadcrumbs));
}

// Get the context function
export function getBreadcrumbState(): BreadcrumbState {
    return getContext<ReturnType<typeof setBreadcrumbState>>(BREADCRUMB_STATE_KEY);
}
