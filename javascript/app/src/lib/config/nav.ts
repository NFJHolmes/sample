import { File, MessagesSquare, Settings } from 'lucide-svelte';
import AudioWaveform from 'lucide-svelte/icons/audio-waveform';
import Command from 'lucide-svelte/icons/command';
import GalleryVerticalEnd from 'lucide-svelte/icons/gallery-vertical-end';
import Lightbulb from 'lucide-svelte/icons/lightbulb';

import type { Nav } from '@repo/types';

export const nav: Nav = {
    user: {
        name: 'Sample',
        email: 'test@sample.com',
        avatar: '',
    },
    teams: [
        {
            name: 'Sample Inc',
            logo: GalleryVerticalEnd,
            plan: 'Enterprise',
        },
        {
            name: 'Acme Corp.',
            logo: AudioWaveform,
            plan: 'Startup',
        },
        {
            name: 'Evil Corp.',
            logo: Command,
            plan: 'Free',
        },
    ],
    items: [
        {
            title: 'Chat',
            url: '/',
            icon: MessagesSquare,
        },
        {
            title: 'Insights',
            url: '/insights',
            icon: Lightbulb,
        },
        {
            title: 'Files',
            url: '/files',
            icon: File,
        },
        {
            title: 'Settings',
            url: '/settings',
            icon: Settings,
        },
    ],
};
