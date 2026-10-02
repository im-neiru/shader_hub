"use client";

import { type PointerEvent, type ReactNode, useRef, useState } from "react";

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
    if (!e.currentTarget.hasPointerCapture(e.pointerId)) {
      return;
    }

    const rect = rootRef.current?.getBoundingClientRect();

    if (!rect) {
      return;
    }

    const r = (e.clientX - rect.left) / rect.width;

    setRatio(Math.min(1 - min, Math.max(min, r)));
  };

  return (
    <div
      ref={rootRef}
      className="grid size-full"
      style={{
        gridTemplateColumns: `${ratio}fr 6px ${1 - ratio}fr`,
      }}
    >
      <div className="relative min-w-0 overflow-hidden">{left}</div>

      <div
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        className="cursor-col-resize touch-none bg-neutral-800"
      />

      <div className="relative min-w-0 overflow-hidden">{right}</div>
    </div>
  );
}
