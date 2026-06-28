import { type APIContext, type APIRoute } from "astro";
import { env } from "cloudflare:workers";

export const prerender = false;

export async function GET({ url }: APIContext): Promise<Response> {
    const route = url.searchParams.get("route")?.replaceAll('/', '_') ?? "/";
    const current = await env.PERSISTENT_DATA.get(`visits_${route}`).then(s => Number.parseInt(s ?? "0"));
    const n = current + 1;
    const message = `.visit-counter::after {content: "${n}" !important;}}`;
    await env.PERSISTENT_DATA.put(`visits_${route}`, n.toString()).catch(console.error);

    return new Response(message, {
        headers: {
            'Content-Type': 'text/css',
        },
    });
}
