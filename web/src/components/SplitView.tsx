"use client";
import { useRef, useState, type ReactNode, type PointerEvent } from "react";

type Props = {
  left: ReactNode;
  right: ReactNode;
  initial?: number;
  min?: number;
};

export function SplitView({ left, right, initial = 0.5, min = 0.3 }: Props) {
  const rootRef = useRef<HTMLDivElement>(null);
  const [ratio, setRatio] = useState(initial);

  const onPointerDown = (e: PointerEvent<HTMLDivElement>) => {
    e.currentTarget.setPointerCapture(e.pointerId);
  };

  const onPointerMove = (e: PointerEvent<HTMLDivElement>) => {
    if (!e.currentTarget.hasPointerCapture(e.pointerId)) return;

    const rect = rootRef.current!.getBoundingClientRect();
    const r = (e.clientX - rect.left) / rect.width;

    setRatio(Math.min(1 - min, Math.max(min, r)));
  };

  return (
    <div
      ref={rootRef}
      style={{
        display: "grid",
        gridTemplateColumns: `${ratio}fr 6px ${1 - ratio}fr`,
        width: "100%",
        height: "100%",
      }}
    >
      <div style={{ position: "relative", overflow: "hidden", minWidth: 0 }}>
        {left}
      </div>
      <div
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        style={{
          cursor: "col-resize",
          background: "#333",
          touchAction: "none",
        }}
      />
      <div style={{ position: "relative", overflow: "hidden", minWidth: 0 }}>
        {right}
      </div>
    </div>
  );
}
