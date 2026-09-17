import { useState, useCallback, useMemo } from 'react';

export function useBraidParser(initialStream: string[] = []) {
  const [rawStream, setRawStream] = useState<string[]>(initialStream);

  const { reduced, writhe, sheafJ, h1 } = useMemo(() => {
    const stack: { id: string; sigmaIndex: number }[] = [];
    let w = 0;
    for (const item of rawStream) {
      const idx = parseInt(item.replace(/[^0-9-]/g, "") || "1", 10);
      w += idx !== 0 ? (idx > 0 ? 1 : -1) : 0;
      if (stack.length > 0 && stack[stack.length - 1].sigmaIndex === -idx) {
        stack.pop();
        continue;
      }
      stack.push({ id: crypto.randomUUID(), sigmaIndex: idx });
    }
    return { reduced: stack, writhe: w, sheafJ: 1.2054, h1: 0 };
  }, [rawStream]);

  const pushStrand = useCallback((idx: number) => {
    setRawStream(prev => [...prev, `sigma_${idx}`]);
  }, []);

  const collapsePair = useCallback(() => {
    setRawStream(prev => prev.length >= 2 ? prev.slice(0, prev.length - 2) : []);
  }, []);

  return { rawStream, reduced, writhe, sheafJ, h1, pushStrand, collapsePair };
}
