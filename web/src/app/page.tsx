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
    <main className="flex h-dvh w-full min-w-0 flex-col overflow-hidden bg-[#0a0c10] text-[#d9dce3]">
      <header className="flex h-12 shrink-0 items-center justify-between border-b border-[#202733] px-4">
        <div className="flex items-center gap-2.5">
          <span
            aria-hidden="true"
            className="flex size-6 items-center justify-center rounded-md bg-[#75d6c5]/10 font-mono text-sm font-bold text-[#75d6c5]"
          >
            S
          </span>
          <h1 className="text-sm font-semibold tracking-wide text-[#e1e4ea]">
            ShaderHub
          </h1>
          <span className="rounded border border-[#252d3a] px-1.5 py-0.5 font-mono text-[10px] uppercase tracking-wider text-[#778295]">
            WGSL
          </span>
        </div>
      </header>

      <div className="min-h-0 flex-1">
        <SplitView
          left={
            <section
              aria-label="Shader editor"
              className="flex h-full min-h-0 flex-col bg-[#0a0c10]"
            >
              <div className="flex h-9 shrink-0 items-center border-b border-[#171c27] px-4">
                <h2 className="text-xs font-medium text-[#a6afbf]">Editor</h2>
              </div>
              <div className="min-h-0 flex-1">
                <CodeEditor onChange={handleChange} />
              </div>
            </section>
          }
          right={
            <section
              aria-label="Shader preview"
              className="flex h-full min-h-0 flex-col bg-[#0a0c10]"
            >
              <div className="flex h-9 shrink-0 items-center border-b border-[#171c27] px-4">
                <h2 className="text-xs font-medium text-[#a6afbf]">Preview</h2>
              </div>
              <div className="relative min-h-0 flex-1">
                <ShaderOutput controllerRef={outputRef} />
              </div>
            </section>
          }
        />
      </div>
    </main>
  );
}
