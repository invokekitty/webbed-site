// @ts-check
import { defineConfig, fontProviders } from 'astro/config';

import cloudflare from '@astrojs/cloudflare';
import mdx from '@astrojs/mdx';

// https://astro.build/config
export default defineConfig({
  adapter: cloudflare(),
  integrations: [mdx()],
  experimental: {
    rustCompiler: true
  },
  build: {
    format: "preserve"
  },
  fonts: [
    {
      provider: fontProviders.fontsource(),
      name: "Roboto Slab",
      cssVariable: "--font-roboto-slab",
      formats: ["woff2", "woff", "ttf"],
      fallbacks: ["serif"]
    },
    {
      provider: fontProviders.fontsource(),
      name: "Tiny5",
      cssVariable: "--font-tiny5",
      formats: ["woff2", "woff", "ttf"],
      fallbacks: ["monospace"]
    },
    {
      provider: fontProviders.fontsource(),
      name: "Lilex",
      cssVariable: "--font-lilex",
      formats: ["woff2", "woff", "ttf"],
      fallbacks: ["monspace"]
    },
    {
      provider: fontProviders.local(),
      name: "waffle",
      cssVariable: "--font-waffle",
      formats: ["ttf"],
      options: {
        variants: [{
          src: ['./src/assets/iconfont/waffle-traced.ttf'],
          weight: 'normal',
          style: 'normal',
          display: 'swap'
        }]
      }
    }
  ]
});
