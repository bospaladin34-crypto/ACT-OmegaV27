// Deno Invariant & StateGraph Super-Step Tests (Zero-Dependency Offline)

import { VesperEngineHandle } from "../src/ffi_bridge.ts";
import { ContinuityCheckpointer, ChatContinuityState } from "../src/continuity_checkpointer.ts";
import { PregelOrchestrator } from "../src/stategraph.ts";

function assertEquals<T>(actual: T, expected: T, msg?: string) {
  if (actual !== expected) {
    throw new Error(msg || `Assertion failed: expected ${expected}, got ${actual}`);
  }
}

Deno.test("L2 Invariant: Vesper C-ABI Parity Lock == 1.000000", () => {
  using engine = new VesperEngineHandle();
  const parity = engine.verifyParity();
  assertEquals(parity, 1.000000);
});

Deno.test("L2 StateGraph: Pregel Super-Step Execution & KV Checkpoint", async () => {
  const orchestrator = new PregelOrchestrator();
  const state = await orchestrator.executeSingleSuperStep(
    "TEST_SESSION_01",
    "sigma_2 sigma_3 sigma_1 sigma_4 sigma_5^-1"
  );

  assertEquals(state.parityTrace, 1.000000);
  assertEquals(state.invariantsLocked, true);
  assertEquals(state.superStepEpoch, 1);
});

Deno.test("L2 Chat Continuity: Checkpoint Save & Restore", async () => {
  const checkpointer = new ContinuityCheckpointer();
  await checkpointer.initialize();

  const testState: ChatContinuityState = {
    sessionId: "SESSION_RESTORE_TEST",
    superStepEpoch: 42,
    activeBraidWord: "sigma_1 sigma_2",
    topologicalCharge: 1,
    parityTrace: 1.000000,
    phaseDelta: 0.17259029,
    landauerJoules: 0.035,
    invariantsLocked: true,
    timestamp: new Date().toISOString(),
  };

  await checkpointer.saveCheckpoint(testState);
  const restored = await checkpointer.restoreCheckpoint("SESSION_RESTORE_TEST");

  assertEquals(restored?.sessionId, "SESSION_RESTORE_TEST");
  assertEquals(restored?.superStepEpoch, 42);
  assertEquals(restored?.parityTrace, 1.000000);

  checkpointer.close();
});