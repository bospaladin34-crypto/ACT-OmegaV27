// Deno WebSocket Telemetry Stream Unit Test

function assertEquals<T>(actual: T, expected: T, msg?: string) {
  if (actual !== expected) {
    throw new Error(msg || `Assertion failed: expected ${expected}, got ${actual}`);
  }
}

Deno.test("HUD Stream: Frame JSON Serialization & Invariants", () => {
  const sampleFrame = {
    epoch: 105401,
    carrierClockHz: 15.965,
    parityTrace: 1.000000,
    phaseDelta: 0.172590,
    landauerJoules: 0.0421,
    b2StaticRecordRate: 88.99,
  };

  const serialized = JSON.stringify(sampleFrame);
  const parsed = JSON.parse(serialized);

  assertEquals(parsed.carrierClockHz, 15.965);
  assertEquals(parsed.parityTrace, 1.000000);
  assertEquals(parsed.b2StaticRecordRate, 88.99);
});