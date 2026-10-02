"use client";

import {
  createContext,
  useContext,
  useEffect,
  useRef,
  useSyncExternalStore,
} from "react";

type WasmModule = typeof import("@/wasm");

type GpuWasmStore = {
  getSnapshot: () => WasmModule | null;
  subscribe: (listener: () => void) => () => void;
  initialize: () => void;
};

export const GpuWasmContext = createContext<WasmModule | null | undefined>(
  undefined,
);

function createGpuWasmStore(): GpuWasmStore {
  let gpuWasm: WasmModule | null = null;
  let wasmPromise: Promise<void> | undefined;

  const listeners = new Set<() => void>();

  return {
    getSnapshot: () => gpuWasm,

    subscribe(listener) {
      listeners.add(listener);

      return () => {
        listeners.delete(listener);
      };
    },

    initialize() {
      wasmPromise ??= (async () => {
        const wasm = await import("@/wasm");

        await wasm.default();

        gpuWasm = wasm;

        for (const listener of listeners) {
          listener();
        }
      })();
    },
  };
}

export function GpuWasmProvider({ children }: { children: React.ReactNode }) {
  const storeRef = useRef<GpuWasmStore | null>(null);

  if (storeRef.current === null) {
    storeRef.current = createGpuWasmStore();
  }

  const store = storeRef.current;

  const gpuWasm = useSyncExternalStore(
    store.subscribe,
    store.getSnapshot,
    () => null,
  );

  useEffect(() => {
    store.initialize();
  }, [store]);

  return (
    <GpuWasmContext.Provider value={gpuWasm}>
      {children}
    </GpuWasmContext.Provider>
  );
}
