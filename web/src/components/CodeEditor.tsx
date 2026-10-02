"use client";

import type { EditorProps } from "@monaco-editor/react";
import dynamic from "next/dynamic";
import { useCallback, useState } from "react";

const MonacoEditor = dynamic(() => import("@monaco-editor/react"), {
  ssr: false,
  loading: () => <div style={{ padding: 16 }}>Loading shader code editor</div>,
});

export const DEFAULT_WGSL = `
struct Camera {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = camera.view_proj * vec4<f32>(input.position, 1.0);
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
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
