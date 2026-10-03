"use client";

import { useCallback, useRef } from "react";
import { ShaderOutput, SplitView } from "@/components";
import CodeEditor from "@/components/CodeEditor";
import type { ShaderOutputHandle } from "@/components/ShaderOutput";

export default function Home() {
  const outputRef = useRef<ShaderOutputHandle>(null);

  const handleChange = useCallback((wgsl: string) => {
    outputRef.current?.setWgsl(wgsl);
  }, []);

  return (
    <main className="flex h-screen w-screen bg-black  flex-col">
      <div className="h-10 border-b border-border-border flex items-center px-2">
        <div className="text-xl font-bold p-3">ShaderHub</div>
      </div>
      <SplitView
        left={<CodeEditor onChange={handleChange} />}
        right={<ShaderOutput controllerRef={outputRef} />}
      />
    </main>
  );
}
