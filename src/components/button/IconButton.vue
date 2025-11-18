<script setup>
import { RouterLink } from "vue-router";

// --- Use ---
const slots = useSlots();
const { t } = useI18n();

// --- Props & models ---
const props = defineProps({
  text: {
    type: [String, null],
    default: null,
  },
  icon: {
    type: [Function, Object],
    required: false,
    default: null,
  },
  size: {
    type: String,
    default: "size-6",
  },
  iconSize: {
    type: String,
    default: "size-4",
  },
  shortcutKey: {
    type: [Object, null],
    default: null,
  },
  to: {
    type: [String, null],
    default: null,
  },
  disabled: {
    type: Boolean,
    required: false,
  },
  isOpen: {
    type: Boolean,
    default: false,
  },
  isShortcutButton: {
    type: Boolean,
    default: false,
  },
});

// --- Emits ---
const emit = defineEmits(["click"]);

// --- Vars ---
const buttonRef = ref();

// --- Handlers ---
// None

// --- Watchers & computed ---
const iconClass = computed(() => {
  var c = `${props.iconSize}`;
  if (props.disabled) c += " opacity-60";
  return c;
});

// --- Lifecycle hooks & related ---
// None
defineExpose({
  buttonRef,
});
</script>

<template>
  <span>
    <BaseTooltip :disabled="!(props.text || slots.content)">
      <component :is="props.to ? RouterLink : 'span'" :to="props.to">
        <button
          ref="buttonRef"
          class="flex items-center justify-center rounded-lg transition-[border,background-color,color,opacity,box-shadow] duration-300 hover:text-main-text-hover focus-visible:outline-none focus:ring-2 focus:ring-primary/20 focus:ring-offset-2"
          :class="{
            'size-8': props.size === 'size-8',
            'size-6': props.size === 'size-6',
            'size-5': props.size === 'size-5',
            'size-4': props.size === 'size-4',
            'cursor-not-allowed opacity-60': props.disabled,
            'hover:bg-main-unselected-hover hover:text-main-text-hover hover:brightness-90 dark:hover:brightness-125':
              !props.disabled & !props.isOpen,
            'focus:bg-main focus:text-main-text': isShortcutButton,
            'bg-main-unselected-hover text-main-text-hover brightness-90 dark:brightness-125 ring-2 ring-primary/20':
              props.isOpen,
          }"
          :disabled="props.disabled"
          @click="emit('click', $event)"
        >
          <slot name="icon">
            <component :is="props.icon" :class="iconClass" />
          </slot>
        </button>
      </component>
      <template #content>
        <slot name="content">
          <div class="flex max-w-[180px] items-center justify-between text-nav">
            {{ props.text }}
            <div
              v-if="props.shortcutKey"
              class="text-main-text/70 ml-2 inline-flex items-center justify-center rounded border border-divider bg-main px-1.5 text-[10px] font-medium uppercase tracking-wide shadow-sm"
            >
              <span v-if="props.shortcutKey.requireCtrl">{{
                t("Command key")
              }}</span>
              <span v-if="props.shortcutKey.requireShift">{{
                t("Shift key")
              }}</span>
              <span v-if="props.shortcutKey.requireAlt">{{
                t("Alt key")
              }}</span>
              <span>{{ props.shortcutKey.key }}</span>
            </div>
          </div>
        </slot>
      </template>
    </BaseTooltip>
  </span>
</template>
