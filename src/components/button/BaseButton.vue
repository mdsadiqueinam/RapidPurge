<script setup>
import { ArrowPathIcon } from "@heroicons/vue/24/outline";

// --- Use ---

// --- Props & models ---
const props = defineProps({
  variant: {
    type: String,
    default: "primary",
    validator(value) {
      return [
        "primary",
        "danger",
        "secondary",
        "outline",
        "text",
        "text-link",
        "transparent",
        "filter",
      ].includes(value);
    },
  },
  type: {
    type: String,
    default: "button",
  },
  disabled: {
    type: Boolean,
    default: false,
  },
  iconOnly: {
    type: Boolean,
    default: false,
  },
  as: {
    type: String,
    default: "button",
    validator(value) {
      return ["button", "a", "RouterLink"].includes(value);
    },
  },
  size: {
    type: String,
    default: "md",
    validator(value) {
      return ["xs", "sm", "md", "lg", "xl", "2xl", "3xl"].includes(value);
    },
  },
  isOpen: {
    type: Boolean,
    default: false,
  },
  isLoading: {
    type: Boolean,
    default: false,
  },
  compRef: {
    type: Function,
    default: () => {},
  },
  round: {
    type: Boolean,
    default: false,
  },
});

// --- Emits ---
// None

// --- Vars ---
// variant specific classes
const variantClassMap = {
  primary:
    "gap-1 bg-primary text-primary-text border border-divider dark:border-black hover:enabled:bg-primary-hover hover:enabled:text-primary-text-hover focus:enabled:ring-2 focus:enabled:ring-primary/20 focus:enabled:ring-offset-2 transition-[border,background-color,color,opacity,box-shadow] duration-300",
  danger:
    "bg-bad text-white hover:enabled:brightness-90 dark:hover:enabled:brightness-125 focus:enabled:ring-2 focus:enabled:ring-danger/20 focus:enabled:ring-offset-2 transition-[background-color,box-shadow] duration-300",
  secondary:
    "gap-1 bg-sidebar text-sidebar-text border border-divider hover:enabled:bg-sidebar-hover hover:enabled:text-sidebar-text-hover hover:enabled:border-divider-hover focus:enabled:ring-2 focus:enabled:ring-primary/20 focus:enabled:ring-offset-2 transition-[border,background-color,color,opacity,box-shadow] duration-300",
  outline:
    "gap-1 border border-divider-outline disabled:opacity-60 text-black dark:text-white hover:enabled:bg-main-selected hover:enabled:text-sidebar-text-hover focus:enabled:ring-2 focus:enabled:ring-primary/20 focus:enabled:ring-offset-2 transition-[border,background-color,color,opacity,box-shadow] duration-300",
  text: "gap-1 border-0 px-0! bg-transparent text-sidebar-text hover:enabled:text-sidebar-text-hover focus:enabled:ring-2 focus:enabled:ring-primary/20 focus:enabled:ring-offset-2 rounded transition-[color,box-shadow] duration-300",
  "text-link":
    "gap-1 border-0 px-0! bg-transparent text-link-text font-semibold hover:enabled:brightness-90 dark:hover:enabled:brightness-125 focus:enabled:ring-2 focus:enabled:ring-primary/20 focus:enabled:ring-offset-2 rounded transition-[color,box-shadow] duration-300",
  transparent:
    "hover:enabled:brightness-90 dark:hover:enabled:brightness-125 focus:enabled:ring-2 focus:enabled:ring-primary/20 focus:enabled:ring-offset-2 transition-[background-color,box-shadow] duration-300",
  filter:
    "h-8 items-center gap-1.5 rounded-lg border border-divider bg-main text-sm text-main-text transition-[border,background-color,color,opacity,box-shadow] duration-300 hover:enabled:border-divider-hover hover:enabled:bg-main-unselected-hover hover:enabled:text-main-text-hover hover:enabled:brightness-90 dark:hover:enabled:brightness-125 focus:enabled:ring-2 focus:enabled:ring-primary/20 focus:enabled:ring-offset-2",
};

// These are usually the same as hover effect but with !
const variantIsOpenClassMap = {
  primary: "!bg-primary-hover !text-primary-text-hover",
  danger: "brightness-90! !dark:brightness-125",
  secondary: "!bg-sidebar-hover !text-sidebar-text-hover !border-divider-hover",
  outline: "border-black! !dark:border-white",
  text: "!text-sidebar-text-hover",
  "text-link": "brightness-90! dark:brightness-125!",
  transparent: "",
  filter:
    "!border-divider-hover !bg-main-unselected-hover !text-main-text-hover",
};

// --- Handlers ---
// None

// --- Watchers & computed ---
const heightClass = computed(() => {
  switch (props.size) {
    case "xs":
      return "h-4";
    case "sm":
      return "h-6";
    case "lg":
      return "h-10";
    case "xl":
      return "h-12";
    case "2xl":
      return "h-14";
    case "3xl":
      return "h-16";
    case "md":
    default:
      return "h-8";
  }
});

const widthClass = computed(() => {
  if (!props.iconOnly) return "";
  switch (props.size) {
    case "xs":
      return "w-4";
    case "sm":
      return "w-6";
    case "md":
      return "w-8";
    case "lg":
      return "w-10";
    case "xl":
      return "w-12";
    case "2xl":
      return "w-14";
    case "3xl":
      return "w-16";
    default:
      return "w-10";
  }
});

const fontClass = computed(() => {
  switch (props.size) {
    case "xs":
      return "text-10";
    case "sm":
      return "text-12";
    case "lg":
      return "text-16";
    case "xl":
      return "text-[18px]";
    case "2xl":
      return "text-[20px]";
    case "3xl":
      return "text-[24px]";
    case "md":
    default:
      return "text-baseline";
  }
});

const paddingClass = computed(() => {
  if (props.iconOnly) return "";
  switch (props.size) {
    case "xs":
      return "px-1";
    case "sm":
      return "px-2";
    case "lg":
      return "px-4";
    case "xl":
      return "px-5";
    case "2xl":
      return "px-6";
    case "3xl":
      return "px-7";
    case "md":
    default:
      return "px-3";
  }
});

const borderRadiusClass = computed(() => {
  if (props.round) return "rounded-full";
  if (props.iconOnly) return "";
  return "rounded-md";
});

const baseClasses = computed(() => {
  return [
    "inline-flex items-center justify-center disabled:cursor-not-allowed disabled:opacity-80 whitespace-nowrap select-none",
    borderRadiusClass.value,
    fontClass.value,
    heightClass.value,
    widthClass.value,
    paddingClass.value,
  ];
});

const classes = computed(() => {
  const classes = [baseClasses.value, variantClassMap[props.variant]];

  if (props.iconOnly) {
    const textSizeClass =
      props.size === "xs"
        ? "text-10"
        : props.size === "sm"
        ? "text-12"
        : props.size === "lg"
        ? "text-20"
        : props.size === "xl"
        ? "text-[24px]"
        : props.size === "2xl"
        ? "text-[28px]"
        : props.size === "3xl"
        ? "text-[32px]"
        : "text-20";
    const borderClass = props.round ? "rounded-full!" : "rounded-lg!";
    classes.push(
      `${borderClass} ${widthClass.value} ${heightClass.value} ${textSizeClass}`
    );
  }

  if (props.isOpen) {
    classes.push(variantIsOpenClassMap[props.variant]);
  }

  return classes;
});

// --- Lifecycle hooks & related ---
// None
const buttonRef = ref();

defineExpose({
  buttonRef,
});
</script>

<template>
  <component
    :is="props.as"
    ref="buttonRef"
    as="button"
    :disabled="props.disabled || props.isLoading"
    :class="classes"
    :type="type"
  >
    <slot v-if="!isLoading" name="icon" />
    <template v-else>
      <ArrowPathIcon class="size-4 animate-spin" />
    </template>
    <slot />
    <slot name="append" />
  </component>
</template>
