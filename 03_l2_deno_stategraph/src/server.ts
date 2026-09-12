// server.ts - ACT-Omega v27.0 L2 Deno StateGraph Server & ADB Hardware Bridge
import { serve } from "https://deno.land/std@0.224.0/http/server.ts";

const PORT = 8098;
const HUD_HTML = Deno.readTextFileSync("../00_orchestration_ps51/visualizer/act_omega_unified_hud.html");
const MOBILE_HTML = Deno.readTextFileSync("../00_orchestration_ps51/visualizer/mobile_hud.html");

console.log(`\x1b[36m=================================================================\x1b[0m`);
console.log(`\x1b[1m\x1b[32m [ACT-OMEGA V27.0]: SERVER & ADB BRIDGE ONLINE @ PORT ${PORT}\x1b[0m`);
console.log(` \x1b[33m[HUD COCKPIT]\x1b[0m     : http://localhost:${PORT}/hud`);
console.log(` \x1b[33m[MOBILE VIEW]\x1b[0m     : http://localhost:${PORT}/mobile`);
console.log(` \x1b[33m[ADB CONTROLLER]\x1b[0m  : /api/adb/status, /api/adb/battery, /api/adb/pull`);
console.log(`\x1b[36m=================================================================\x1b[0m\n`);

serve(async (req: Request) => {
  const url = new URL(req.url);

  // 1. API: ADB Device Status
  if (url.pathname === "/api/adb/status") {
    try {
      const proc = new Deno.Command("adb.exe", { args: ["devices"] }).outputSync();
      const stdout = new TextDecoder().decode(proc.stdout);
      const isConnected = stdout.includes("\tdevice");
      return new Response(JSON.stringify({ connected: isConnected, raw: stdout }), {
        headers: { "content-type": "application/json", "access-control-allow-origin": "*" },
      });
    } catch (e) {
      return new Response(JSON.stringify({ connected: false, error: String(e) }), {
        headers: { "content-type": "application/json", "access-control-allow-origin": "*" },
      });
    }
  }

  // 2. API: Live Battery & Hardware Dumpsys over ADB
  if (url.pathname === "/api/adb/battery") {
    try {
      const proc = new Deno.Command("adb.exe", { args: ["shell", "dumpsys", "battery"] }).outputSync();
      const stdout = new TextDecoder().decode(proc.stdout);
      const res: Record<string, any> = {};
      stdout.split("\n").forEach((line) => {
        if (line.includes(":")) {
          const [k, v] = line.split(":", 2);
          res[k.trim()] = v.trim();
        }
      });
      return new Response(JSON.stringify(res), {
        headers: { "content-type": "application/json", "access-control-allow-origin": "*" },
      });
    } catch (e) {
      return new Response(JSON.stringify({ error: String(e) }), {
        headers: { "content-type": "application/json", "access-control-allow-origin": "*" },
      });
    }
  }

  // 3. API: Trigger adb reverse tcp:8098 tcp:8098
  if (url.pathname === "/api/adb/reverse") {
    try {
      const proc = new Deno.Command("adb.exe", { args: ["reverse", "tcp:8098", "tcp:8098"] }).outputSync();
      const stdout = new TextDecoder().decode(proc.stdout);
      return new Response(JSON.stringify({ success: true, output: stdout }), {
        headers: { "content-type": "application/json", "access-control-allow-origin": "*" },
      });
    } catch (e) {
      return new Response(JSON.stringify({ success: false, error: String(e) }), {
        headers: { "content-type": "application/json", "access-control-allow-origin": "*" },
      });
    }
  }

  // 4. API: Pull Field Ledger from Phone
  if (url.pathname === "/api/adb/pull") {
    try {
      const proc = new Deno.Command("adb.exe", {
        args: ["pull", "/sdcard/Download/missoula_field_expedition.jsonl", "../data/open/missoula_field_expedition.jsonl"],
      }).outputSync();
      const stdout = new TextDecoder().decode(proc.stdout);
      return new Response(JSON.stringify({ success: true, output: stdout }), {
        headers: { "content-type": "application/json", "access-control-allow-origin": "*" },
      });
    } catch (e) {
      return new Response(JSON.stringify({ success: false, error: String(e) }), {
        headers: { "content-type": "application/json", "access-control-allow-origin": "*" },
      });
    }
  }

  // 5. WebSocket Ingress (/ws)
  if (url.pathname === "/ws") {
    if (req.headers.get("upgrade") !== "websocket") {
      return new Response("Expected WebSocket", { status: 400 });
    }
    const { socket, response } = Deno.upgradeWebSocket(req);
    socket.onopen = () => {
      console.log(`\x1b[32m[WS LATCHED]\x1b[0m: Ingress stream active.`);
    };
    socket.onmessage = (e) => {
      try {
        const data = JSON.parse(e.data);
        if (data.event === "MOBILE_ORIENT") {
          // Relayed live
        }
      } catch (_) {}
    };
    return response;
  }

  // 6. Mobile View (/mobile)
  if (url.pathname === "/mobile") {
    return new Response(MOBILE_HTML, { headers: { "content-type": "text/html; charset=utf-8" } });
  }

  // 7. Cockpit (/hud or /)
  return new Response(HUD_HTML, { headers: { "content-type": "text/html; charset=utf-8" } });
}, { port: PORT, hostname: "0.0.0.0" });