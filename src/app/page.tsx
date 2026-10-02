"use client";

import { CodeEditor, ShaderOutput } from "@/components";
import { useState } from "react";

export default function Home() {
  const [code, setCode] = useState<string>("");

  return (
    <main className="flex h-screen w-screen bg-black">
      <CodeEditor onChange={setCode} />
      <ShaderOutput code={code} />
    </main>
  );
}
