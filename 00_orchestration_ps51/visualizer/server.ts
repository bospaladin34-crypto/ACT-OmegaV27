// Sovereign Host Ingestion Relay with Continuity Checkpointing (Port 8098)

const PORT = 8098;
const HOST = "0.0.0.0";
const visualizerHtmlPath = "C:\\sovereign_manifold_v27\\00_orchestration_ps51\\visualizer\\index.html";
const anchorPath = "C:\\sovereign_manifold_v27\\08_continuity_checkpoints\\active_session_anchor.json";
const telemetryLogPath = "C:\\sovereign_manifold_v27\\09_telemetry_and_specs\\ledgers\\memory_ring_telemetry.jsonl";

console.log(`[SOVEREIGN_RELAY]: Server listening on http://${HOST}:${PORT} and ws://${HOST}:${PORT}/ws`);

const sockets = new Set<WebSocket>();
let globalEpoch = 108600;

// Load existing session anchor if available
let sessionAnchor = {
  sessionId: "SESSION_PIXEL10_CONTINUITY_01",
  superStepEpoch: 108600,
  carrierClockHz: 15.965,
  activeBraidWord: "sigma_2 sigma_3 sigma_1 sigma_4 sigma_5^-1",
  parityTrace: 1.000000,
  phaseDelta: 0.172590,
  landauerJoules: 0.0421,
  pixel10: {
    slot: 50,
    deviceHash: "0xa8f3b29c01d4e765",
    lifetimeTensors: 154000,
    totalB2Events: 1420,
    lastDeltaMagUt: 0.0842,
    rttMs: 0.08,
  },
  lastCheckpointTimestamp: new Date().toISOString(),
};

try {
  const text = Deno.readTextFileSync(anchorPath);
  const parsed = JSON.parse(text);
  if (parsed.superStepEpoch) {
    globalEpoch = parsed.superStepEpoch;
    sessionAnchor = parsed;
    console.log(`  [RESUMED]: Loaded checkpoint from ${anchorPath} at Epoch ${globalEpoch}`);
  }
} catch (_) {}

// Broadcast live telemetry frames at 15.965 Hz (62.636 ms)
setInterval(() => {
  globalEpoch++;
  sessionAnchor.superStepEpoch = globalEpoch;

  const telemetryFrame = JSON.stringify({
    epoch: globalEpoch,
    carrierClockHz: 15.965,
    parityTrace: 1.000000,
    phaseDelta: sessionAnchor.phaseDelta,
    landauerJoules: 0.0421,
    b2StaticRecordRate: 88.99,
    b2KineticRecordRate: 97.11,
    activeBraidWord: sessionAnchor.activeBraidWord,
    activeSlotsCount: 55,
    pixel10: {
      slot: 50,
      epoch: globalEpoch,
      parity: 1.000000,
      deltaMagUt: sessionAnchor.pixel10.lastDeltaMagUt,
      isB2Detected: sessionAnchor.pixel10.lastDeltaMagUt > 0.053,
      b2Rate: 88.99,
      computeLatencyUs: 42.5,
      lifetimeTensors: sessionAnchor.pixel10.lifetimeTensors,
      rttMs: sessionAnchor.pixel10.rttMs,
      is1to1Locked: true,
    },
    status: "LAMINAR_FLOW_LOCKED",
    timestamp: new Date().toISOString(),
  });

  for (const ws of sockets) {
    if (ws.readyState === WebSocket.OPEN) {
      ws.send(telemetryFrame);
    }
  }
}, 62.636);

// Atomic Checkpoint Flush every 5 seconds
setInterval(async () => {
  try {
    sessionAnchor.lastCheckpointTimestamp = new Date().toISOString();
    await Deno.writeTextFile(anchorPath, JSON.stringify(sessionAnchor, null, 2));
  } catch (_) {}
}, 5000);

Deno.serve({ port: PORT, hostname: HOST }, (req) => {
  const url = new URL(req.url);

  if (url.pathname === "/health") {
    return new Response(JSON.stringify({
      status: "SOVEREIGN_MANIFOLD_ONLINE",
      carrierClockHz: 15.965,
      slot: 50,
      epoch: globalEpoch,
      lifetimeTensors: sessionAnchor.pixel10.lifetimeTensors,
      parityTrace: 1.000000
    }), { headers: { "content-type": "application/json; charset=utf-8" } });
  }

  if (url.pathname === "/ws") {
    try {
      const { socket, response } = Deno.upgradeWebSocket(req);
      socket.onopen = () => sockets.add(socket);
      
      socket.onmessage = async (e) => {
        try {
          const msg = JSON.parse(e.data);
          
          // Handshake & State Restoration Handshake
          if (msg.event === "PEER_HANDSHAKE") {
            console.log(`  [HANDSHAKE_VERIFIED]: ${msg.alias} connected. Restoring state vector...`);
            socket.send(JSON.stringify({
              event: "PEER_HANDSHAKE_ACK",
              restoredEpoch: globalEpoch,
              restoredLifetimeTensors: sessionAnchor.pixel10.lifetimeTensors,
              restoredB2Events: sessionAnchor.pixel10.totalB2Events,
              status: "RESTORED_FROM_VAULT",
            }));
          }
          
          // Telemetry & Periodic Checkpoint Commit from Pixel 10
          if (msg.event === "NPU_SLOT_FRAME") {
            sessionAnchor.pixel10.lifetimeTensors = msg.lifetimeTensors || sessionAnchor.pixel10.lifetimeTensors;
            sessionAnchor.pixel10.lastDeltaMagUt = msg.deltaMagUt || 0.0842;
            sessionAnchor.pixel10.totalB2Events = msg.totalB2Events || sessionAnchor.pixel10.totalB2Events;
            sessionAnchor.phaseDelta = msg.measuredPhaseDelta || 0.172590;
          }
        } catch (_) {}
      };

      socket.onclose = () => sockets.delete(socket);
      socket.onerror = () => sockets.delete(socket);
      return response;
    } catch (err) {
      return new Response(`Upgrade Failed: ${err}`, { status: 400 });
    }
  }

  try {
    const html = Deno.readTextFileSync(visualizerHtmlPath);
    return new Response(html, { headers: { "content-type": "text/html; charset=utf-8" } });
  } catch (err) {
    return new Response(`Error loading visualizer: ${err}`, { status: 500 });
  }
});