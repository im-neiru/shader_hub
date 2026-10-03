"use client";

import type { BeforeMount, EditorProps } from "@monaco-editor/react";
import dynamic from "next/dynamic";
import { useCallback, useEffect, useRef, useState } from "react";
import { useGpuWasm } from "@/lib/hooks";

const MonacoEditor = dynamic(() => import("@monaco-editor/react"), {
  ssr: false,
  loading: () => <div style={{ padding: 16 }}>Loading shader code editor</div>,
});

const EDITOR_OPTIONS: EditorProps["options"] = {
  minimap: {
    enabled: false,
  },
  fontSize: 14,
  fontFamily:
    "Cascadia Code, JetBrains Mono, SFMono-Regular, Consolas, monospace",
  fontLigatures: true,
  automaticLayout: true,
  scrollBeyondLastLine: false,
  tabSize: 4,
  padding: {
    top: 14,
    bottom: 14,
  },
  lineNumbers: "on",
  lineNumbersMinChars: 3,
  renderLineHighlight: "line",
  roundedSelection: false,
  cursorBlinking: "smooth",
  cursorSmoothCaretAnimation: "on",
  smoothScrolling: true,
  folding: true,
  foldingHighlight: false,
  showFoldingControls: "mouseover",
  bracketPairColorization: {
    enabled: true,
  },
  guides: {
    indentation: true,
    bracketPairs: true,
    highlightActiveIndentation: true,
  },
  renderWhitespace: "selection",
  overviewRulerBorder: false,
  hideCursorInOverviewRuler: true,
  wordWrap: "off",
  contextmenu: true,
  renderValidationDecorations: "on",
  suggest: {
    showIcons: true,
  },
  quickSuggestions: true,
  inlineSuggest: {
    enabled: true,
  },
  stickyScroll: {
    enabled: true,
  },
};

const handleBeforeMount: BeforeMount = (monaco) => {
  monaco.editor.defineTheme("shader-dark", {
    base: "vs-dark",
    inherit: true,
    rules: [
      { token: "keyword", foreground: "D39AE8", fontStyle: "bold" },
      { token: "keyword.control", foreground: "D39AE8", fontStyle: "bold" },
      { token: "keyword.operator", foreground: "D39AE8" },
      { token: "type", foreground: "75D6C5" },
      { token: "type.identifier", foreground: "75D6C5" },
      { token: "number", foreground: "B9D58D" },
      { token: "number.hex", foreground: "B9D58D" },
      { token: "string", foreground: "F0A58A" },
      { token: "string.escape", foreground: "F6C177" },
      { token: "comment", foreground: "687386", fontStyle: "italic" },
      { token: "delimiter", foreground: "8D96A6" },
      { token: "delimiter.bracket", foreground: "A6AFBF" },
      { token: "operator", foreground: "D6DBE5" },
      { token: "function", foreground: "8FC7FF" },
      { token: "function.declaration", foreground: "8FC7FF" },
      { token: "variable", foreground: "D6DBE5" },
      { token: "constant", foreground: "BFCBFF" },
      { token: "annotation", foreground: "C6A0E8" },
    ],
    colors: {
      "editor.background": "#0A0C10",
      "editor.foreground": "#D9DCE3",
      "editorPane.background": "#0A0C10",
      "editorGutter.background": "#0A0C10",
      "editorGutter.foreground": "#343B48",
      "editorLineNumber.foreground": "#394151",
      "editorLineNumber.activeForeground": "#858FA0",
      "editorCursor.foreground": "#C9D4FF",
      "editorCursor.background": "#0A0C10",
      "editor.selectionBackground": "#263149",
      "editor.inactiveSelectionBackground": "#1A2234",
      "editor.selectionHighlightBackground": "#202941",
      "editor.lineHighlightBackground": "#111620",
      "editor.lineHighlightBorder": "#171C27",
      "editorIndentGuide.background1": "#171C25",
      "editorIndentGuide.activeBackground1": "#293140",
      "editorWhitespace.foreground": "#2C333F",
      "editorBracketMatch.background": "#1C2535",
      "editorBracketMatch.border": "#46536A",
      "editor.foldBackground": "#121721",
      "editorOverviewRuler.border": "#0A0C10",
      "editorOverviewRuler.currentContentForeground": "#59667C",
      "editorOverviewRuler.findMatchForeground": "#7563A8",
      "editorOverviewRuler.rangeHighlightForeground": "#3C4860",
      "editorWidget.background": "#0D1016",
      "editorWidget.foreground": "#D8DCE5",
      "editorWidget.border": "#202733",
      "editorHoverWidget.background": "#0D1016",
      "editorHoverWidget.border": "#293140",
      "editorSuggestWidget.background": "#0D1016",
      "editorSuggestWidget.border": "#252D3A",
      "editorSuggestWidget.foreground": "#D8DCE5",
      "editorSuggestWidget.selectedBackground": "#1B2433",
      "editorSuggestWidget.highlightForeground": "#9BC8FF",
      "editorSuggestWidget.focusHighlightForeground": "#9BC8FF",
      "editorBracketHighlight.foreground1": "#8FC7FF",
      "editorBracketHighlight.foreground2": "#75D6C5",
      "editorBracketHighlight.foreground3": "#C6A0E8",
      "editorBracketHighlight.foreground4": "#F0A58A",
      "editorBracketHighlight.foreground5": "#B9D58D",
      "editorBracketHighlight.foreground6": "#D8B77C",
      "editorBracketHighlight.unexpectedBracket.foreground": "#E07A91",
      "editorError.foreground": "#F38BA8",
      "editorWarning.foreground": "#E7C77C",
      "editorInfo.foreground": "#8FC7FF",
      "editorHint.foreground": "#75D6C5",
      "editorCodeLens.foreground": "#626C7C",
      "editorCodeLensLine.foreground": "#202733",
      "editorRuler.foreground": "#171C24",
      "editorUnnecessaryCode.opacity": "#00000066",
      "editorInlayHint.background": "#121720",
      "editorInlayHint.foreground": "#778295",
      "editorStickyScroll.background": "#0D1016",
      "editorStickyScrollHover.background": "#111620",
      "editorStickyScroll.border": "#1E2632",
      "editorIndentGuide.background2": "#151A22",
      "editorIndentGuide.activeBackground2": "#242C39",
      "editorGutter.addedBackground": "#75D6C5",
      "editorGutter.modifiedBackground": "#8FC7FF",
      "editorGutter.deletedBackground": "#F38BA8",
      "scrollbarSlider.background": "#2A313D88",
      "scrollbarSlider.hoverBackground": "#39425099",
      "scrollbarSlider.activeBackground": "#485263AA",
      "editorGroup.border": "#151A22",
      "editorGroupHeader.tabsBackground": "#090B0F",
      "editorGroupHeader.noTabsBackground": "#090B0F",
      "tab.activeBackground": "#0D1016",
      "tab.activeForeground": "#E1E4EA",
      "tab.inactiveBackground": "#090B0F",
      "tab.inactiveForeground": "#70798A",
      "tab.border": "#151A22",
      "tab.activeBorder": "#75D6C5",
      "input.background": "#0C0F14",
      "input.foreground": "#D8DCE5",
      "input.border": "#252D3A",
      "input.placeholderForeground": "#626C7C",
      focusBorder: "#3F4B60",
      "dropdown.background": "#0D1016",
      "dropdown.foreground": "#D8DCE5",
      "dropdown.border": "#252D3A",
      "button.background": "#1A2331",
      "button.foreground": "#E1E4EA",
      "button.hoverBackground": "#222D3D",
      "list.activeSelectionBackground": "#1B2433",
      "list.activeSelectionForeground": "#E1E4EA",
      "list.inactiveSelectionBackground": "#151C29",
      "list.inactiveSelectionForeground": "#D8DCE5",
      "list.hoverBackground": "#111822",
      "list.focusBackground": "#182131",
      "list.highlightForeground": "#9BC8FF",
    },
  });
};

type CodeEditorProps = {
  onChange?: (code: string) => void;
};

export default function CodeEditor({ onChange }: CodeEditorProps) {
  const gpuWasm = useGpuWasm();

  const [code, setCode] = useState(gpuWasm?.getDefaultWgsl() ?? "");
  const hasEditedRef = useRef(false);
  const onChangeRef = useRef(onChange);

  onChangeRef.current = onChange;

  useEffect(() => {
    if (gpuWasm && !hasEditedRef.current) {
      setCode(gpuWasm.getDefaultWgsl());
    }
  }, [gpuWasm]);

  const handleChange = useCallback((value: string | undefined) => {
    const next = value ?? "";
    hasEditedRef.current = true;
    setCode(next);
    onChangeRef.current?.(next);
  }, []);

  return (
    <MonacoEditor
      height="100%"
      language="wgsl"
      theme="shader-dark"
      value={code}
      onChange={handleChange}
      beforeMount={handleBeforeMount}
      options={EDITOR_OPTIONS}
    />
  );
}
