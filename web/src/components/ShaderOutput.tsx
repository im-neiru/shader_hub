'use client';

import { useEffect, useRef } from 'react';

type ShaderOutputProps = {
  code: string;
};

export default function ShaderOutput({ code }: ShaderOutputProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    let cancelled = false;
    let renderer: import('@/wasm').Renderer | undefined;

    async function initialize() {
      const wasm = await import('@/wasm');

      await wasm.default();

      const canvas = canvasRef.current;
      if (!canvas || cancelled) {
        return;
      }

      renderer = await wasm.Renderer.create(canvas);

      if (cancelled) {
        renderer = undefined;
        return;
      }


      console.log(code);
    }

    void initialize();

    return () => {
      cancelled = true;
      renderer = undefined;
    };
  }, [code]);

  return (
    <canvas
      ref={canvasRef}
      width={1280}
      height={720}
    />
  );
}
