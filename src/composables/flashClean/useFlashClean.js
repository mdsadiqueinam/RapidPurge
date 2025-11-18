import { formatBytes } from "@root/utils/bytes";
import { Channel, invoke } from "@tauri-apps/api/core";

const symbol = Symbol("FlashClean");

function FlashCleanState() {
  const scanData = shallowRef(null);
  const scanState = shallowRef(null);
  const currentPathStr = shallowRef("");
  const junkFound = shallowRef("0 B");
  const scanning = computed(() => scanState.value === "progress");
  const finished = computed(() => scanState.value === "finished");

  const event = new Channel();

  event.onmessage = (msg) => {
    scanState.value = msg.event;
    scanData.value = msg.data;
    if (msg.event === "progress") {
      currentPathStr.value = msg.data.currentPathStr;
      junkFound.value = formatBytes(msg.data.junkFound);
    }
  };

  function startScan() {
    invoke("flash_scan", { event });
  }

  return {
    scanning,
    finished,
    startScan,
    currentPathStr,
    junkFound,
  };
}

export function provideFlashClean() {
  const state = FlashCleanState();
  provide(symbol, state);
  return state;
}

/**
 *
 * @returns {ReturnType<typeof FlashCleanState>}
 */
export function useFlashClean() {
  return inject(symbol);
}
