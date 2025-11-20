import { defineConfig, globalIgnores } from "eslint/config";
import globals from "globals";
import js from "@eslint/js";
import pluginVue from "eslint-plugin-vue";
import pluginOxlint from "eslint-plugin-oxlint";
import skipFormatting from "@vue/eslint-config-prettier/skip-formatting";
import eslintrcAutoImport from "./eslintrc-auto-import.js";
import vueI18n from "@intlify/eslint-plugin-vue-i18n";
import babelParser from "@babel/eslint-parser";

export default defineConfig([
  {
    name: "app/files-to-lint",
    files: ["**/*.{js,mjs,jsx,vue}"],
  },

  globalIgnores([
    "**/dist/**",
    "**/dist-ssr/**",
    "**/coverage/**",
    "**/resources/**",
  ]),

  {
    languageOptions: {
      globals: {
        ...globals.browser,
        ...eslintrcAutoImport.globals,
        process: "readonly",
      },
      parser: babelParser,
      parserOptions: {
        requireConfigFile: false,
        ecmaVersion: "latest", // enables modern syntax
        sourceType: "module",
        babelOptions: {
          babelrc: false,
          configFile: false,
          presets: ["@babel/preset-env"],
        },
      },
    },
    settings: {
      "vue-i18n": {
        localeDir: "./src/locales/*.{json,json5,yaml,yml}",
        messageSyntaxVersion: "11.1.6",
      },
    },
  },

  js.configs.recommended,
  ...pluginVue.configs["flat/strongly-recommended"],
  ...vueI18n.configs.recommended,
  ...pluginOxlint.configs["flat/recommended"],
  skipFormatting,

  {
    rules: {
      "no-unsafe-optional-chaining": "warn",
      "no-console": ["error", { allow: ["warn", "error", "info"] }],
      "no-unused-vars": [
        "error",
        { argsIgnorePattern: "^_", varsIgnorePattern: "^_" },
      ],
      "vue/multi-word-component-names": "off",
      "vue/attribute-hyphenation": ["error", "never"],
      "vue/v-on-event-hyphenation": ["error", "never"],
      "vue/attributes-order": [
        "error",
        {
          order: [
            "DEFINITION",
            "LIST_RENDERING",
            "CONDITIONALS",
            "RENDER_MODIFIERS",
            "GLOBAL",
            "UNIQUE",
            "TWO_WAY_BINDING",
            "OTHER_DIRECTIVES",
            "OTHER_ATTR",
            "EVENTS",
            "CONTENT",
          ],
          alphabetical: false,
        },
      ],
      "vue/no-v-html": "off", // Disabled as we use it in some places (e.g. for rendering HTML emails)
      "vue/block-order": [
        "error",
        { order: [["script", "template"], "style"] },
      ],
      "@intlify/vue-i18n/no-deprecated-i18n-component": "error",
      "@intlify/vue-i18n/no-deprecated-i18n-place-attr": "error",
      "@intlify/vue-i18n/no-deprecated-i18n-places-prop": "error",
      "@intlify/vue-i18n/no-deprecated-modulo-syntax": "error",
      "@intlify/vue-i18n/no-deprecated-tc": "error",
      "@intlify/vue-i18n/no-deprecated-v-t": "error",
      "@intlify/vue-i18n/no-i18n-t-path-prop": "error",
      "@intlify/vue-i18n/no-raw-text": "error",
      "@intlify/vue-i18n/no-v-html": "error",
      "@intlify/vue-i18n/valid-message-syntax": "error",
      "@intlify/vue-i18n/no-duplicate-keys-in-locale": "error",
      "@intlify/vue-i18n/prefer-linked-key-with-paren": "error",
      "@intlify/vue-i18n/no-html-messages": "off", // Temporary turn off to allow HTML in translation strings (e.g., mailto links)
    },
  },
]);
