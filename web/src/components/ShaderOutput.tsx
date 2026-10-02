"use client";

import { type Ref, useEffect, useImperativeHandle, useRef, useState } from "react";

import { useGpuWasm } from "@/lib/hooks";

type GpuWasm = NonNullable<ReturnType<typeof useGpuWasm>>;
type GpuRenderer = Awaited<ReturnType<GpuWasm["Renderer"]["create"]>>;

export type ShaderOutputHandle = {
  setWgsl: (wgsl: string) => void;
};

type ShaderOutputProps = {
  initialWgsl: string;
  controllerRef?: Ref<ShaderOutputHandle>;
};

export default function ShaderOutput({ initialWgsl, controllerRef }: ShaderOutputProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const gpuWasm = useGpuWasm();

  const [shaderError, setShaderError] = useState<string | null>(null);
  const wgslRef = useRef(initialWgsl);
  const rendererRef = useRef<GpuRenderer | undefined>(undefined);
  const compileTimerRef = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  const compileQueueRef = useRef<Promise<void>>(Promise.resolve());
  const isRebuildingRef = useRef(false);
  const pendingResizeRef = useRef<[number, number] | null>(null);

  const scheduleCompile = () => {
    if (compileTimerRef.current) {
      clearTimeout(compileTimerRef.current);
    }

    compileTimerRef.current = setTimeout(() => {
      const renderer = rendererRef.current;
      if (!renderer) {
        return;
      }

      const source = wgslRef.current;
      compileQueueRef.current = compileQueueRef.current.then(async () => {
        isRebuildingRef.current = true;

        try {
          await renderer.setWgsl(source);
          if (rendererRef.current === renderer && wgslRef.current === source) {
            setShaderError(null);
          }
        } catch (error) {
          if (rendererRef.current === renderer && wgslRef.current === source) {
            setShaderError(error instanceof Error ? error.message : String(error));
          }
        } finally {
          isRebuildingRef.current = false;

          const pendingSize = pendingResizeRef.current;
          pendingResizeRef.current = null;

          if (pendingSize && rendererRef.current === renderer) {
            renderer.resize(...pendingSize);
          }
        }
      });
    }, 200);
  };

  useImperativeHandle(
    controllerRef,
    () => ({
      setWgsl(wgsl) {
        wgslRef.current = wgsl;
        scheduleCompile();
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

      const renderer = await wasm.Renderer.create(canvas, initialWgsl);

      if (cancelled) {
        return;
      }

      rendererRef.current = renderer;

      if (wgslRef.current !== initialWgsl) {
        scheduleCompile();
      }

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

        if (isRebuildingRef.current) {
          pendingResizeRef.current = [nextWidth, nextHeight];
        } else {
          renderer.resize(nextWidth, nextHeight);
        }
      };

      observer = new ResizeObserver(resize);
      observer.observe(container);

      resize();

      const render = () => {
        if (cancelled) {
          return;
        }

        if (!isRebuildingRef.current) {
          renderer.render(canvas);
        }
        animationFrame = requestAnimationFrame(render);
      };

      animationFrame = requestAnimationFrame(render);
    }

    void initialize();

    return () => {
      cancelled = true;

      cancelAnimationFrame(animationFrame);
      if (compileTimerRef.current) {
        clearTimeout(compileTimerRef.current);
      }
      observer?.disconnect();

      observer = undefined;
      rendererRef.current = undefined;
    };
  }, [gpuWasm, initialWgsl]);

  return (
    <>
      <canvas ref={canvasRef} className="block h-full w-full" />
      {shaderError && (
        <pre
          role="alert"
          className="absolute inset-x-3 bottom-3 max-h-40 overflow-auto whitespace-pre-wrap rounded-sm bg-red-950/95 p-3 font-mono text-xs text-red-100"
        >
          {shaderError}
        </pre>
      )}
    </>
  );
}
