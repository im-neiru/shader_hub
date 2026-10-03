"use client";

import { CaretDown, CaretUp, DotsSix } from "@phosphor-icons/react";
import {
  type KeyboardEvent,
  type ReactNode,
  type PointerEvent as ReactPointerEvent,
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
  const [isDockOpen, setIsDockOpen] = useState(true);
  const separatorSize = isMobile ? 44 : 6;

  useEffect(() => {
    const mediaQuery = window.matchMedia("(max-width: 767px)");
    const updateLayout = () => setIsMobile(mediaQuery.matches);

    updateLayout();
    mediaQuery.addEventListener("change", updateLayout);

    return () => mediaQuery.removeEventListener("change", updateLayout);
  }, []);

  const onPointerDown = (e: ReactPointerEvent<HTMLButtonElement>) => {
    e.currentTarget.setPointerCapture(e.pointerId);
  };

  const onPointerMove = (e: ReactPointerEvent<HTMLButtonElement>) => {
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

    setRatio(Math.min(0.8, Math.max(min, r)));
  };

  const onKeyDown = (e: KeyboardEvent<HTMLButtonElement>) => {
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
        gridTemplateAreas: isMobile && !isDockOpen
          ? '"right" "separator"'
          : isMobile
          ? '"right" "separator" "left"'
          : '"left separator right"',
        gridTemplateColumns: isMobile
          ? "minmax(0, 1fr)"
          : `${ratio}fr ${separatorSize}px ${1 - ratio}fr`,
        gridTemplateRows: isMobile
          ? isDockOpen
            ? `${ratio}fr ${separatorSize}px ${1 - ratio}fr`
            : `minmax(0, 1fr) ${separatorSize}px`
          : "minmax(0, 1fr)",
      }}
    >
      {(!isMobile || isDockOpen) && (
        <div
          className="relative min-h-0 min-w-0 overflow-hidden"
          style={{ gridArea: "left" }}
        >
          {left}
        </div>
      )}

      <div
        className={`relative flex min-h-0 min-w-0 items-center justify-center bg-[#0d1016] ${
          isMobile
            ? "border-y border-[#252d3a] px-3"
            : "bg-[#171c27]"
        }`}
        style={{ gridArea: "separator" }}
      >
        <button
          type="button"
          aria-label="Resize editor and preview"
          onPointerDown={onPointerDown}
          onPointerMove={onPointerMove}
          onKeyDown={onKeyDown}
          className={`flex touch-none items-center justify-center text-[#778295] hover:text-[#75d6c5] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-[#75d6c5] ${
            isMobile
              ? "h-full min-w-14 flex-1 cursor-row-resize"
              : "absolute inset-0 h-full w-full cursor-col-resize"
          }`}
        >
          <DotsSix
            aria-hidden="true"
            size={isMobile ? 26 : 20}
            weight="bold"
            className={isMobile ? "" : "rotate-90"}
          />
        </button>
        {isMobile && (
          <button
            type="button"
            aria-label={isDockOpen ? "Collapse editor dock" : "Expand editor dock"}
            onClick={() => setIsDockOpen((open) => !open)}
            className="absolute right-3 z-10 flex size-9 items-center justify-center rounded-md border border-[#252d3a] bg-[#111620] text-[#a6afbf] hover:border-[#75d6c5]/50 hover:text-[#75d6c5] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[#75d6c5]"
          >
            {isDockOpen ? (
              <CaretDown aria-hidden="true" size={18} weight="bold" />
            ) : (
              <CaretUp aria-hidden="true" size={18} weight="bold" />
            )}
          </button>
        )}
      </div>

      <div
        className="relative min-h-0 min-w-0 overflow-hidden"
        style={{ gridArea: "right" }}
      >
        {right}
      </div>
    </div>
  );
}
