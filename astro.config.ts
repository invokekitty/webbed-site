// @ts-check
import { defineConfig, fontProviders } from 'astro/config';

import cloudflare from '@astrojs/cloudflare';
import mdx from '@astrojs/mdx';

// https://astro.build/config
export default defineConfig({
  adapter: cloudflare(),
  integrations: [mdx()],
  experimental: {
  },
  build: {
    format: "preserve"
  },
  prefetch: true,
  fonts: [
    {
      provider: fontProviders.fontsource(),
      name: "Roboto Slab",
      cssVariable: "--font-roboto-slab",
      formats: ["woff2", "woff", "ttf"],
      fallbacks: ["monospace"],
      display: "swap"
    },
    {
      provider: fontProviders.fontsource(),
      name: "Tiny5",
      cssVariable: "--font-tiny5",
      formats: ["ttf"],
      fallbacks: ["sans-serif"],
      display: "swap"
    },
    {
      provider: fontProviders.fontsource(),
      name: "Lilex",
      cssVariable: "--font-lilex",
      formats: ["woff2", "woff", "ttf"],
      fallbacks: ["monospace"],
      display: "swap"
    },
    {
      provider: fontProviders.local(),
      name: "waffle",
      cssVariable: "--font-waffle",
      formats: ["ttf"],
      options: {
        variants: [{
          src: ['./src/assets/fonts/waffle-traced.ttf'],
          weight: 'normal',
          style: 'normal'
        }]
      }
    }
  ]
});
