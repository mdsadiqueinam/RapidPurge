import { fileURLToPath, URL } from "node:url";
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import AutoImport from "unplugin-auto-import/vite";
import VueRouter from "unplugin-vue-router/vite";
import Components from "unplugin-vue-components/vite";
import { HeadlessUiResolver } from "unplugin-vue-components/resolvers";
import { VueRouterAutoImports } from "unplugin-vue-router";
import svgLoader from "vite-svg-loader";
import { v4 as uuidv4 } from "uuid";
import tailwindcss from "@tailwindcss/vite";

const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [
    VueRouter({ importMode: "sync", dts: "./typed-router.d.ts" }),
    vue(),
    tailwindcss(),
    Components({
      resolvers: [HeadlessUiResolver()],
      dirs: ["resources/js/shared/components/**", "src/components/**"],
    }),
    svgLoader({
      svgoConfig: {
        plugins: [
          {
            name: "prefixIds",
            params: {
              prefix: uuidv4().split("-")[0],
            },
          },
        ],
      },
    }),
    AutoImport({
      imports: ["vue", "vue-i18n", "@vueuse/core", VueRouterAutoImports],
      dirs: [
        "src/utils/*i18n.js",
        "src/utils/*Shortcut.js",
        "resources/js/shared/use/*.js",
        "src/composables/*.js",
      ],
      dts: true,
      eslintrc: {
        enabled: true,
        filepath: "./eslintrc-auto-import.js",
      },
    }),
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  resolve: {
    alias: {
      "@root": fileURLToPath(new URL("./src", import.meta.url)),
      "@shared": fileURLToPath(
        new URL("./resources/js/shared", import.meta.url)
      ),
      "@pages": fileURLToPath(new URL("./src/pages", import.meta.url)),
      "@svgs": fileURLToPath(new URL("./src/svgs", import.meta.url)),
      "@utils": fileURLToPath(new URL("./src/utils", import.meta.url)),
      "@components": fileURLToPath(
        new URL("./src/components", import.meta.url)
      ),
      "@composables": fileURLToPath(
        new URL("./src/composables", import.meta.url)
      ),
    },
  },
}));
