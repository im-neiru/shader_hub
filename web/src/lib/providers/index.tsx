import { PropsWithChildren } from "react";
import { GpuWasmProvider } from "./GpuWasmProvider";

export function Providers({ children }: PropsWithChildren) {
  return <GpuWasmProvider>{children}</GpuWasmProvider>;
}
