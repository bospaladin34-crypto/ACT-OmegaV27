import { assertEquals } from "https://deno.land/std@0.224.0/assert/mod.ts";
import { OllamaSieveAdapter } from "../src/ollama_sieve_tool.ts";

Deno.test("Domain 4/6: Ollama Tool Gating & Topological Sieve Verification", () => {
  const adapter = new OllamaSieveAdapter();

  // Test 1: Grounded Physical Output Sieve
  const validOutput = "Topological metric shear produces flat galactic rotation curves through vacuum drag.";
  const resValid = adapter.sieveProposition("VESPER-BASE:latest", validOutput);

  assertEquals(resValid.sheafConsistent, true);
  assertEquals(resValid.penroseCollapsePassed, true);
  assertEquals(resValid.parityTrace, 1.000000);
  assertEquals(resValid.snappedRoots.length > 0, true);

  // Test 2: Hallucination / Contradictory Claim Clamping
  const hallucinatedOutput = "A perpetual motion machine operates with zero dissipation in vacuum.";
  const resHallucination = adapter.sieveProposition("VESPER-BASE:latest", hallucinatedOutput);

  assertEquals(resHallucination.sheafConsistent, false);
  assertEquals(resHallucination.penroseCollapsePassed, false);
  assertEquals(resHallucination.parityTrace, 0.000000);

  console.log("\n==================================================================");
  console.log(" [PASS]: Domain 6 Subordinate Tool Boundary Gating Verified");
  console.log(" [PASS]: Grounded Proposition Passed Sieve -> Parity: 1.000000");
  console.log(" [PASS]: Hallucinated Proposition Clamped -> Parity: 0.000000 (Quarantined)");
  console.log("==================================================================\n");
});