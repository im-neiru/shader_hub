"use client";

import { CodeEditor, ShaderOutput, SplitView } from "@/components";

export default function Home() {
  return (
    <main className="flex h-screen w-screen bg-black">
      <SplitView left={<CodeEditor />} right={<ShaderOutput code={""} />} />
    </main>
  );
}
