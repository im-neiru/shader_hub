"use client";

import { type Ref, useEffect, useImperativeHandle, useRef } from "react";

import { useGpuWasm } from "@/lib/hooks";

type GpuWasm = NonNullable<ReturnType<typeof useGpuWasm>>;
type GpuRenderer = Awaited<ReturnType<GpuWasm["Renderer"]["create"]>>;

export type ShaderOutputHandle = {
  setWgsl: (wgsl: string) => void;
};

type ShaderOutputProps = {
  controllerRef?: Ref<ShaderOutputHandle>;
};

export default function ShaderOutput({ controllerRef }: ShaderOutputProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const gpuWasm = useGpuWasm();

  const wgslRef = useRef("");
  const rendererRef = useRef<GpuRenderer | undefined>(undefined);

  useImperativeHandle(
    controllerRef,
    () => ({
      setWgsl(wgsl) {
        wgslRef.current = wgsl;

        const renderer = rendererRef.current;
        if (!renderer) {
          return;
        }

        renderer.setWgsl(wgsl);
      },
    }),
    [],
  );

  useEffect(() => {
    if (!gpuWasm) {
      return;
    }

    const wasm = gpuWasm;

    let cancelled = false;
    let observer: ResizeObserver | undefined;
    let animationFrame = 0;

    async function initialize() {
      const canvas = canvasRef.current;
      const container = canvas?.parentElement;

      if (!canvas || !container || cancelled) {
        return;
      }

      const renderer = await wasm.Renderer.create(canvas);

      if (cancelled) {
        return;
      }

      rendererRef.current = renderer;

      let width = 0;
      let height = 0;

      const resize = () => {
        if (cancelled) {
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

      if (wgslRef.current) {
        renderer.setWgsl(wgslRef.current);
      }

      const render = () => {
        if (cancelled) {
          return;
        }

        renderer.render(canvas);
        animationFrame = requestAnimationFrame(render);
      };

      animationFrame = requestAnimationFrame(render);
    }

    void initialize();

    return () => {
      cancelled = true;

      cancelAnimationFrame(animationFrame);
      observer?.disconnect();

      observer = undefined;
      rendererRef.current = undefined;
    };
  }, [gpuWasm]);

  return <canvas ref={canvasRef} className="block h-full w-full" />;
}
