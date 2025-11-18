<script setup>
import { TransitionRoot } from "@headlessui/vue";
import { useParentElement } from "@vueuse/core";

// --- Props & models ---
const props = defineProps({
  showOnHover: {
    type: Boolean,
    default: true,
  },
  showOnClick: {
    type: Boolean,
    default: false,
  },
  content: {
    type: String,
    default: "",
  },
  disabled: {
    type: Boolean,
    default: false,
  },
  delay: {
    type: Number,
    default: 600,
  },
  placement: {
    type: String,
    default: "top",
  },
  defaultClass: {
    type: String,
    default: "",
  },
  /**
   * Configure a target element to trigger component toggle; 'true' means it will use the parent DOM element,
   * 'false' means it will use the default slot as a target element;
   * By using a String (CSS selector) or a DOM element it attaches the events to the specified DOM element (if it exists)
   */
  target: {
    type: [Boolean, String, Object],
    default: false,
  },
});

// --- Emits ---
// None

// --- Vars ---
const show = defineModel({ type: Boolean, default: false });

const parentEl = useParentElement();
const hovered = ref(false);
const clicked = ref(false);
const targetRef = useTemplateRef("targetRef");
const tooltipRef = useTemplateRef("tooltipRef");

// Touch device detection
const isTouchDevice = ref(
  "ontouchstart" in window || navigator.maxTouchPoints > 0
);
const tooltipPosition = reactive({
  left: 0,
  top: 0,
  arrowLeft: 0,
  arrowTop: 0,
  arrowPlacement: "",
  arrowStyle: {},
  transform: "translateY(0%)",
});
const triggerElement = computed(() => {
  if (!props.target) {
    // for backward compatibility when target is false it will accept a slot
    return targetRef.value;
  }

  if (props.target === true) {
    // returm parent HTML element
    return parentEl.value;
  }

  if (typeof props.target === "string") {
    // return HTML element by selector
    return document.querySelector(props.target);
  }

  if (typeof props.target === "object") {
    // return HTML element by reference
    return props.target;
  }

  return null;
});

// --- Handlers ---
const mouseOverTimer = useTimeoutFn(
  () => {
    hovered.value = true;
    nextTick(debouncedUpdatePosition);
  },
  computed(() => props.delay),
  { immediate: false }
);

const mouseOutTimer = useTimeoutFn(() => {
  hovered.value = false;
  mouseOverTimer.stop();
}, 200);

function mouseOver() {
  mouseOutTimer.stop();
  mouseOverTimer.start();
}

function mouseOut() {
  mouseOutTimer.start();
}

function onClicked() {
  clicked.value = true;
  nextTick(updateTooltipPosition);
}

function onClickOutside() {
  clicked.value = false;
}

const debouncedUpdatePosition = useDebounceFn(updateTooltipPosition, 10);

function updateTooltipPosition() {
  if (!triggerElement.value || !tooltipRef.value || !shouldShow.value) return;

  const triggerRect = triggerElement.value.getBoundingClientRect();
  const tooltipRect = tooltipRef.value.getBoundingClientRect();
  const offset = 10;
  const arrowSize = 5;

  const viewportWidth = window.innerWidth;
  const viewportHeight = window.innerHeight;

  // Calculate available space in each direction
  const spaceAbove = triggerRect.top;
  const spaceBelow = viewportHeight - triggerRect.bottom;
  const spaceLeft = triggerRect.left;
  const spaceRight = viewportWidth - triggerRect.right;

  // Determine the best placement based on available space
  let finalPlacement = props.placement;

  // Auto-flip logic: if there's not enough space, try alternative placements
  const needsFlip = {
    top: spaceAbove < tooltipRect.height + offset,
    bottom: spaceBelow < tooltipRect.height + offset,
    left: spaceLeft < tooltipRect.width + offset,
    right: spaceRight < tooltipRect.width + offset,
  };

  if (needsFlip[finalPlacement]) {
    // Try opposite placement first
    const oppositePlacements = {
      top: "bottom",
      bottom: "top",
      left: "right",
      right: "left",
    };

    const oppositePlacement = oppositePlacements[finalPlacement];
    if (!needsFlip[oppositePlacement]) {
      finalPlacement = oppositePlacement;
    } else {
      // If opposite also doesn't fit, choose the side with more space
      if (finalPlacement === "top" || finalPlacement === "bottom") {
        finalPlacement = spaceAbove > spaceBelow ? "top" : "bottom";
      } else {
        finalPlacement = spaceLeft > spaceRight ? "left" : "right";
      }
    }
  }

  function horizontalPlacementStrategy() {
    let left = triggerRect.left + triggerRect.width / 2 - tooltipRect.width / 2;

    // Clamp horizontal position to viewport
    const clampedLeft = Math.max(
      offset,
      Math.min(left, viewportWidth - tooltipRect.width - offset)
    );

    // Calculate arrow position relative to tooltip
    const arrowLeft =
      Math.max(
        arrowSize * 2,
        Math.min(
          triggerRect.left + triggerRect.width / 2 - clampedLeft,
          tooltipRect.width - arrowSize * 2
        )
      ) - arrowSize;

    return {
      left: clampedLeft,
      arrowLeft,
    };
  }

  function verticalPlacementStrategy() {
    // For left/right placement with CSS transforms, we just need the center point
    const top = triggerRect.top + triggerRect.height / 2;
    const arrowTop = -arrowSize; // Arrow will be positioned at 50% via CSS

    return {
      top,
      arrowTop,
    };
  }

  // Calculate position based on final placement
  const placementStrategy = {
    top: () => {
      const horizontalStrategy = horizontalPlacementStrategy();
      // Position tooltip above target - use CSS transform for better multi-line positioning
      const top = triggerRect.top - offset;

      return {
        left: horizontalStrategy.left,
        top,
        arrowLeft: horizontalStrategy.arrowLeft,
        arrowTop: tooltipRect.height - arrowSize,
        arrowPlacement: "bottom",
        transform: "translateY(-100%)",
      };
    },
    bottom: () => {
      const horizontalStrategy = horizontalPlacementStrategy();
      const top = triggerRect.bottom + offset;

      return {
        left: horizontalStrategy.left,
        top,
        arrowLeft: horizontalStrategy.arrowLeft,
        arrowTop: -arrowSize,
        arrowPlacement: "top",
        transform: "translateY(0%)",
      };
    },
    left: () => {
      const verticalStrategy = verticalPlacementStrategy();
      const left = triggerRect.left - offset;

      return {
        left,
        top: verticalStrategy.top,
        arrowLeft: -arrowSize, // Arrow positioned via CSS at 50%
        arrowTop: verticalStrategy.arrowTop,
        arrowPlacement: "right",
        transform: "translateX(-100%) translateY(-50%)",
      };
    },
    right: () => {
      const verticalStrategy = verticalPlacementStrategy();
      const left = triggerRect.right + offset;

      return {
        left,
        top: verticalStrategy.top,
        arrowLeft: -arrowSize,
        arrowTop: verticalStrategy.arrowTop,
        arrowPlacement: "left",
        transform: "translateX(0%) translateY(-50%)",
      };
    },
  };

  // Apply positioning
  const result = (placementStrategy[finalPlacement] || placementStrategy.top)();

  // Apply the calculated position
  tooltipPosition.left = result.left;
  tooltipPosition.top = result.top;
  tooltipPosition.arrowLeft = result.arrowLeft;
  tooltipPosition.arrowTop = result.arrowTop;
  tooltipPosition.arrowPlacement = result.arrowPlacement;
  tooltipPosition.transform = result.transform;

  tooltipPosition.arrowStyle = ["bottom", "top"].includes(
    tooltipPosition.arrowPlacement
  )
    ? { left: `${tooltipPosition.arrowLeft}px` }
    : {}; // Left/right arrows use CSS positioning
}

// --- Watchers & computed ---
/**
 * The tooltip should show if:
 * 1. The show prop is explicitly set
 * 2. The showOnClick prop is set and the tooltip was clicked
 * 3. Otherwise, show the tooltip if the showOnHover prop is true and the tooltip is hovered
 */

const shouldShow = computed(() => {
  // Don't show tooltip on touch devices
  if (isTouchDevice.value) {
    return false;
  }

  if (props.disabled) {
    return false;
  }

  if (show.value !== false) {
    return show.value;
  }
  if (props.showOnClick) {
    return clicked.value;
  }

  return props.showOnHover && hovered.value;
});

// --- Lifecycle hooks & related ---
useEventListener("resize", debouncedUpdatePosition);
useEventListener("scroll", debouncedUpdatePosition, {
  capture: true,
  passive: true,
});
useEventListener("click", onClickOutside);

useEventListener(triggerElement, "mouseenter", mouseOver);
useEventListener(triggerElement, "mouseleave", mouseOut);
useEventListener(triggerElement, "click", onClicked);

watch(shouldShow, (value) => {
  if (value) {
    nextTick(updateTooltipPosition);
  }
});
</script>

<template>
  <span
    v-if="!props.target"
    ref="targetRef"
    class="inline-block"
    :class="props.defaultClass"
  >
    <slot />
  </span>

  <Teleport v-if="shouldShow" to="body">
    <TransitionRoot
      :show="true"
      enter="transition-opacity duration-75"
      enterFrom="opacity-0"
      enterTo="opacity-100"
      leave="transition-opacity duration-150"
      leaveFrom="opacity-100"
      leaveTo="opacity-0"
    >
      <div
        ref="tooltipRef"
        class="fixed z-9999 flex min-h-8 max-w-[400px] items-center rounded-lg border border-divider-hover bg-main-unselected text-sm text-sidebar-text shadow-lg focus:outline-none"
        :style="{
          left: `${tooltipPosition.left}px`,
          top: `${tooltipPosition.top}px`,
          transform: tooltipPosition.transform,
        }"
        @mouseenter="mouseOver"
        @mouseleave="mouseOut"
      >
        <div
          class="absolute h-2.5 w-2.5 rotate-45 border-inherit bg-main-unselected"
          :class="{
            'bottom-[-5px] border-b border-r':
              tooltipPosition.arrowPlacement === 'bottom',
            'right-[-5px] top-1/2 -translate-y-1/2 border-r border-t':
              tooltipPosition.arrowPlacement === 'right',
            'left-[-5px] top-1/2 -translate-y-1/2 border-b border-l':
              tooltipPosition.arrowPlacement === 'left',
            'top-[-5px] border-l border-t':
              tooltipPosition.arrowPlacement === 'top',
          }"
          :style="tooltipPosition.arrowStyle"
        ></div>
        <slot name="customContent">
          <div
            class="flex max-h-[33vh] flex-col items-start justify-start overflow-auto px-2 py-1 text-nav leading-relaxed"
          >
            <slot name="content">{{ props.content }}</slot>
          </div>
        </slot>
      </div>
    </TransitionRoot>
  </Teleport>
</template>
