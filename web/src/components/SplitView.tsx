"use client";

import {
  type KeyboardEvent,
  type PointerEvent,
  type ReactNode,
  useEffect,
  useRef,
  useState,
} from "react";

type Props = {
  left: ReactNode;
  right: ReactNode;
  initial?: number;
  min?: number;
};

export function SplitView({ left, right, initial = 0.5, min = 0.3 }: Props) {
  const rootRef = useRef<HTMLDivElement>(null);
  const [ratio, setRatio] = useState(initial);
  const [isMobile, setIsMobile] = useState(false);
  const separatorSize = isMobile ? 12 : 6;

  useEffect(() => {
    const mediaQuery = window.matchMedia("(max-width: 767px)");
    const updateLayout = () => setIsMobile(mediaQuery.matches);

    updateLayout();
    mediaQuery.addEventListener("change", updateLayout);

    return () => mediaQuery.removeEventListener("change", updateLayout);
  }, []);

  const onPointerDown = (e: PointerEvent<HTMLHRElement>) => {
    e.currentTarget.setPointerCapture(e.pointerId);
  };

  const onPointerMove = (e: PointerEvent<HTMLHRElement>) => {
    if (!e.currentTarget.hasPointerCapture(e.pointerId)) {
      return;
    }

    const rect = rootRef.current?.getBoundingClientRect();

    if (!rect) {
      return;
    }

    const r = isMobile
      ? (e.clientY - rect.top) / rect.height
      : (e.clientX - rect.left) / rect.width;

    setRatio(Math.min(1 - min, Math.max(min, r)));
  };

  const onKeyDown = (e: KeyboardEvent<HTMLHRElement>) => {
    const decrease = isMobile ? e.key === "ArrowUp" : e.key === "ArrowLeft";
    const increase = isMobile ? e.key === "ArrowDown" : e.key === "ArrowRight";

    if (!decrease && !increase) {
      return;
    }

    e.preventDefault();
    setRatio((current) =>
      Math.min(1 - min, Math.max(min, current + (increase ? 0.03 : -0.03))),
    );
  };

  return (
    <div
      ref={rootRef}
      className="grid size-full min-h-0 overflow-hidden"
      style={{
        gridTemplateAreas: isMobile
          ? '"right" "separator" "left"'
          : '"left separator right"',
        gridTemplateColumns: isMobile
          ? "minmax(0, 1fr)"
          : `${ratio}fr ${separatorSize}px ${1 - ratio}fr`,
        gridTemplateRows: isMobile
          ? `${ratio}fr ${separatorSize}px ${1 - ratio}fr`
          : "minmax(0, 1fr)",
      }}
    >
      <div
        className="relative min-h-0 min-w-0 overflow-hidden"
        style={{ gridArea: "left" }}
      >
        {left}
      </div>

      <hr
        tabIndex={0}
        aria-label="Resize editor and preview"
        aria-orientation={isMobile ? "horizontal" : "vertical"}
        aria-valuemin={Math.round(min * 100)}
        aria-valuemax={Math.round((1 - min) * 100)}
        aria-valuenow={Math.round(ratio * 100)}
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onKeyDown={onKeyDown}
        className={`m-0 touch-none border-0 bg-[#171c27] transition-colors hover:bg-[#75d6c5]/50 focus-visible:bg-[#75d6c5]/50 focus-visible:outline-none ${
          isMobile ? "cursor-row-resize" : "cursor-col-resize"
        }`}
        style={{ gridArea: "separator" }}
      />

      <div
        className="relative min-h-0 min-w-0 overflow-hidden"
        style={{ gridArea: "right" }}
      >
        {right}
      </div>
    </div>
  );
}
