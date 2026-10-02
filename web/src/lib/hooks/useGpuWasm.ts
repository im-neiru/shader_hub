import { useContext } from "react";

import { GpuWasmContext } from "../providers/GpuWasmProvider";

export function useGpuWasm() {
  const context = useContext(GpuWasmContext);

  if (context === undefined) {
    throw new Error("useGpuWasm must be used inside GpuWasmProvider");
  }

  return context;
}
