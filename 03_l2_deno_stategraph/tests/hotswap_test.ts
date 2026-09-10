// Deno Dynamic Hot-Swap Unit Test

function assertEquals<T>(actual: T, expected: T, msg?: string) {
  if (actual !== expected) {
    throw new Error(msg || `Assertion failed: expected ${expected}, got ${actual}`);
  }
}

import { HotSwapManager } from "../src/hotswap_manager.ts";

Deno.test("HotSwap: Engage Bypass, Canary Audit & Atomic Latch", () => {
  const manager = new HotSwapManager();
  
  const bypassOk = manager.engageBypass(14);
  assertEquals(bypassOk, true);

  const mockCandidate = { verifyParity: () => 1.000000 };
  const canaryOk = manager.auditCanaryInvariants(mockCandidate);
  assertEquals(canaryOk, true);

  const latchOk = manager.commitLatch(14);
  assertEquals(latchOk, true);
});