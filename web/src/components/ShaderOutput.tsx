"use client";

import { useEffect, useRef } from "react";
import { useGpuWasm } from "@/lib/hooks";

type ShaderOutputProps = {
  code: string;
};

export default function ShaderOutput({ code }: ShaderOutputProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const gpuWasm = useGpuWasm();

  useEffect(() => {
    if (!gpuWasm) {
      return;
    }

    const wasm = gpuWasm;
    let cancelled = false;
    let renderer: import("@/wasm").Renderer | undefined;
    let observer: ResizeObserver | undefined;

    async function initialize() {
      const canvas = canvasRef.current;
      const container = canvas?.parentElement;

      if (!canvas || !container || cancelled) {
        return;
      }

      renderer = await wasm.Renderer.create(canvas);

      if (cancelled) {
        renderer = undefined;
        return;
      }

      let width = 0;
      let height = 0;

      const resize = () => {
        if (!renderer || cancelled) {
          return;
        }

        const nextWidth = Math.max(1, container.clientWidth);
        const nextHeight = Math.max(1, container.clientHeight);

        if (nextWidth === width && nextHeight === height) {
          return;
        }

        width = nextWidth;
        height = nextHeight;

        canvas.width = nextWidth;
        canvas.height = nextHeight;

        renderer.resize(nextWidth, nextHeight);
      };

      observer = new ResizeObserver(resize);
      observer.observe(container);

      resize();
    }

    void initialize();

    return () => {
      cancelled = true;
      observer?.disconnect();
      observer = undefined;
      renderer = undefined;
    };
  }, [gpuWasm]);

  return <canvas ref={canvasRef} className="block w-full h-full" />;
}
