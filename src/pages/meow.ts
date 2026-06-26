import type { APIRoute } from "astro";

export const prerender = false;

let meow = 0;

export const GET: APIRoute = async ({ request, session }) => {
    let message = `.visit-counter::after {content: "${meow++}" !important;}}`

    return new Response(message, {
        headers: {
            'Content-Type': 'text/css',
        },
    });
}