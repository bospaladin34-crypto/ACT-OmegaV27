// server.ts - ACT-Omega v27.0 L2 Deno StateGraph Server
// Serves HUD Web App, Mobile Web View, and multiplexes WebSockets on port 8098

import { serve } from "https://deno.land/std@0.224.0/http/server.ts";

const PORT = 8098;
const HUD_HTML = Deno.readTextFileSync("../00_orchestration_ps51/visualizer/act_omega_unified_hud.html");
const MOBILE_HTML = Deno.readTextFileSync("../00_orchestration_ps51/visualizer/mobile_hud.html");

console.log(`\x1b[36m=================================================================\x1b[0m`);
console.log(`\x1b[1m\x1b[32m [ACT-OMEGA V27.0]: L2 STATEGRAPH SERVER ONLINE @ PORT ${PORT}\x1b[0m`);
console.log(` \x1b[33m[WORKSTATION HUD]\x1b[0m : http://localhost:${PORT}/hud`);
console.log(` \x1b[33m[MOBILE COCKPIT]\x1b[0m  : http://localhost:${PORT}/mobile`);
console.log(` \x1b[33m[WEBSOCKET]\x1b[0m       : ws://localhost:${PORT}/ws`);
console.log(`\x1b[36m=================================================================\x1b[0m\n`);

serve((req: Request) => {
  const url = new URL(req.url);

  // 1. WebSocket Ingress (/ws)
  if (url.pathname === "/ws") {
    if (req.headers.get("upgrade") !== "websocket") {
      return new Response("Expected WebSocket", { status: 400 });
    }
    const { socket, response } = Deno.upgradeWebSocket(req);
    socket.onopen = () => {
      console.log(`\x1b[32m[WS CONNECTED]\x1b[0m: Ingress client attached to Hydro-Bus.`);
    };
    socket.onmessage = (e) => {
      try {
        const data = JSON.parse(e.data);
        if (data.event === "MOBILE_ORIENT") {
          // Relays live phone orientation to workstation
        }
      } catch (_) {}
    };
    return response;
  }

  // 2. Mobile Cockpit View (/mobile)
  if (url.pathname === "/mobile") {
    return new Response(MOBILE_HTML, {
      headers: { "content-type": "text/html; charset=utf-8" },
    });
  }

  // 3. Workstation HUD (/ or /hud)
  return new Response(HUD_HTML, {
    headers: { "content-type": "text/html; charset=utf-8" },
  });
}, { port: PORT });