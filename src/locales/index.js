import { createI18n } from "vue-i18n";
import en from "./en.json";

export const supportedLocal = [
  {
    language: "English",
    code: "en",
  },
  {
    language: "Spanish",
    code: "es",
  },
  {
    language: "French",
    code: "fr",
  },
  {
    language: "Italian",
    code: "it",
  },
  {
    language: "German",
    code: "de",
  },
  {
    language: "Portuguese",
    code: "pt",
  },
  {
    language: "Dutch",
    code: "nl",
  },
  {
    language: "Swedish",
    code: "sv",
  },
  {
    language: "Chinese",
    code: "zh",
  },
  {
    language: "Japanese",
    code: "ja",
  },
];

const defaultLanguageCode = window.navigator.language.slice(0, 2);
const locale = useLocalStorage("rapid-purge/locale", defaultLanguageCode, {
  shallow: true,
});
// Ensure we have this language on the list or fallback to english
if (!supportedLocal.find((o) => o.code === locale.value)) {
  locale.value = "en";
}
/* @vite-ignore */
const messages = await import(/* @vite-ignore */ `./${locale.value}.json`).then(
  (r) => r?.default || r
);

const i18n = createI18n({
  legacy: false, // you must set `false`, to use Composition API
  locale: locale.value,
  fallbackLocale: "en",
  messages: {
    ...(locale.value !== "en" ? { [locale.value]: messages } : {}),
    en,
  },
});

export function setLocale(newLocale) {
  if (supportedLocal.find((o) => o.code === newLocale)) {
    locale.value = newLocale;
    i18n.global.locale.value = newLocale;
    loadLocaleMessages(newLocale);
  } else {
    console.warn(
      `Locale ${newLocale} is not supported, falling back to English`
    );
    locale.value = "en";
    i18n.global.locale.value = "en";
  }
}

// load user request .json file
async function loadLocaleMessages(locale) {
  // load locale messages
  const messages = await import(/* @vite-ignore */ `./${locale}.json`).then(
    (r) => r?.default || r
  );

  // set locale and locale message
  i18n.global.setLocaleMessage(locale, messages);
}

export default i18n;
