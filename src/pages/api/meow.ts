import type { APIContext, APIRoute } from "astro";

export const prerender = false;

let meow: Record<string, number> = {};

export function GET({ url }: APIContext): Response {
    const route = url.searchParams.get("route") ?? "/";
    meow[route] ??= 0;
    const message = `.visit-counter::after {content: "${++meow[route]}" !important;}}`;

    return new Response(message, {
        headers: {
            'Content-Type': 'text/css',
        },
    });
}
