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

  function startScan() {
    const event = new Channel();

    event.onmessage = (msg) => {
      scanState.value = msg.event;
      scanData.value = msg.data;
      if (msg.event === "progress") {
        currentPathStr.value = msg.data.currentPathStr;
        junkFound.value = formatBytes(msg.data.junkFound);
      } else {
        const jsonData = { ...msg.data };
        jsonData.nodes.forEach((node) => {
          node.nodes.data = node.nodes.data.map((item) => {
            return {
              ...item,
              children: undefined,
            };
          });
        });
        // download the scanData as json file
        const dataStr = JSON.stringify(jsonData, null, 2);
        const blob = new Blob([dataStr], { type: "application/json" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = "flash_scan_result.json";
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
      }
    };

    invoke("flash_scan", { event });
  }

  function resetState() {
    scanData.value = null;
    scanState.value = null;
    currentPathStr.value = "";
    junkFound.value = "0 B";
  }

  return {
    scanning,
    finished,
    startScan,
    currentPathStr,
    junkFound,
    scanData,
    scanState,
    resetState,
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
