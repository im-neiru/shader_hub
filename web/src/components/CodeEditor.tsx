"use client";

import dynamic from "next/dynamic";
import { useCallback, useState } from "react";
import type { EditorProps } from "@monaco-editor/react";

const MonacoEditor = dynamic(() => import("@monaco-editor/react"), {
  ssr: false,
  loading: () => <div style={{ padding: 16 }}>Loading shader code editor</div>,
});

export const DEFAULT_WGSL = `

// sample shader

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>( 0.0,  0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>( 0.5, -0.5),
    );
    return vec4<f32>(positions[index], 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 1.0, 1.0);
}
`;

const EDITOR_OPTIONS: EditorProps["options"] = {
  minimap: { enabled: false },
  fontSize: 14,
  automaticLayout: true,
  scrollBeyondLastLine: false,
  tabSize: 4,
};

type CodeEditorProps = {
  initialCode?: string;
  onChange?: (code: string) => void;
};

export default function CodeEditor({
  initialCode = DEFAULT_WGSL,
  onChange,
}: CodeEditorProps) {
  const [code, setCode] = useState(initialCode);

  const handleChange = useCallback(
    (value: string | undefined) => {
      const next = value ?? "";
      setCode(next);
      onChange?.(next);
    },
    [onChange],
  );

  return (
    <MonacoEditor
      height="100%"
      language="wgsl"
      theme="vs-dark"
      value={code}
      onChange={handleChange}
      options={EDITOR_OPTIONS}
    />
  );
}
