import { PUBLIC_SUPABASE_ANON_KEY, PUBLIC_SUPABASE_URL } from '$env/static/public';
import { createBrowserClient, createServerClient, isBrowser } from '@supabase/ssr';
import { type MetaTagsProps } from 'svelte-meta-tags';

import { SampleImg } from '@repo/components/graphics';
import { type Database } from '@repo/types';

import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ data, url, depends, fetch }) => {
    /**
     * Declare a dependency so the layout can be invalidated, for example, on
     * session refresh.
     */
    depends('supabase:auth');

    const supabase = isBrowser()
        ? createBrowserClient<Database>(PUBLIC_SUPABASE_URL, PUBLIC_SUPABASE_ANON_KEY, {
              global: {
                  fetch,
              },
          })
        : createServerClient<Database>(PUBLIC_SUPABASE_URL, PUBLIC_SUPABASE_ANON_KEY, {
              global: {
                  fetch,
              },
              cookies: {
                  getAll() {
                      return data.cookies;
                  },
              },
          });

    /**
     * It's fine to use `getSession` here, because on the client, `getSession` is
     * safe, and on the server, it reads `session` from the `LayoutData`, which
     * safely checked the session using `safeGetSession`.
     */
    const {
        data: { session },
    } = await supabase.auth.getSession();

    const {
        data: { user },
    } = await supabase.auth.getUser();

    const title = `Sample`;
    const description = `Sample Description`;
    const canonicalUrl = new URL(url.pathname, url.origin).href;

    const baseMetaTags: MetaTagsProps = {
        title,
        titleTemplate: `%s | Sample`,
        description,
        canonical: canonicalUrl,
        openGraph: {
            type: 'website',
            url: canonicalUrl,
            locale: 'en_US',
            title,
            description,
            siteName: title,
            images: [
                {
                    url: SampleImg,
                    alt: title,
                    width: 1200,
                    height: 630,
                    type: 'image/png',
                },
            ],
        },
        twitter: {
            creator: `@sample`,
            site: `https://sample.com`,
            cardType: 'summary_large_image',
            description,
            image: SampleImg,
            imageAlt: description,
        },
    };

    return { session, supabase, user, baseMetaTags, ip: data.ip };
};
