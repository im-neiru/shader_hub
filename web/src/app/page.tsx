"use client";

import { useCallback, useRef } from "react";
import CodeEditor, { DEFAULT_WGSL } from "@/components/CodeEditor";
import { ShaderOutput, SplitView } from "@/components";
import type { ShaderOutputHandle } from "@/components/ShaderOutput";

export default function Home() {
  const outputRef = useRef<ShaderOutputHandle>(null);

  const handleChange = useCallback((wgsl: string) => {
    outputRef.current?.setWgsl(wgsl);
  }, []);

  return (
    <main className="flex h-screen w-screen bg-black">
      <SplitView
        left={<CodeEditor initialCode={DEFAULT_WGSL} onChange={handleChange} />}
        right={<ShaderOutput initialWgsl={DEFAULT_WGSL} controllerRef={outputRef} />}
      />
    </main>
  );
}
