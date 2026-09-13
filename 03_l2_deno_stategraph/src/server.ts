// server.ts - ACT-Omega v27.0 L2 Deno StateGraph Server & ADB Controller
import { serve } from "https://deno.land/std@0.224.0/http/server.ts";

const PORT = 8098;
const HUD_PATH = "../00_orchestration_ps51/visualizer/act_omega_unified_hud.html";
const MOBILE_PATH = "../00_orchestration_ps51/visualizer/mobile_hud.html";
const LOG_PATH = "../data/open/missoula_field_expedition.jsonl";

console.log(`\x1b[36m=================================================================\x1b[0m`);
console.log(`\x1b[1m\x1b[32m [ACT-OMEGA V27.0]: AUTONOMOUS INGRESS SERVER ONLINE @ ${PORT}\x1b[0m`);
console.log(`\x1b[36m=================================================================\x1b[0m\n`);

serve(async (req: Request) => {
  const url = new URL(req.url);

  // API: ADB Device Status
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

  // API: Live Battery Dumpsys
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

  // API: Latch ADB Reverse (tcp:8098)
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

  // API: Auto-Pull Field Ledger from Phone
  if (url.pathname === "/api/adb/pull") {
    try {
      const proc = new Deno.Command("adb.exe", {
        args: ["pull", "/sdcard/Download/missoula_field_expedition.jsonl", LOG_PATH],
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

  // API: Fetch Parsed Frames for Client Map Dynamic Reload
  if (url.pathname === "/api/data/frames") {
    try {
      const text = Deno.readTextFileSync(LOG_PATH);
      const frames = text
        .split("\n")
        .filter((l) => l.trim().startsWith("{") && l.trim().endsWith("}"))
        .map((l) => JSON.parse(l));
      return new Response(JSON.stringify(frames), {
        headers: { "content-type": "application/json", "access-control-allow-origin": "*" },
      });
    } catch (_) {
      return new Response("[]", {
        headers: { "content-type": "application/json", "access-control-allow-origin": "*" },
      });
    }
  }

  // Static Views
  if (url.pathname === "/mobile") {
    const html = Deno.readTextFileSync(MOBILE_PATH);
    return new Response(html, { headers: { "content-type": "text/html; charset=utf-8" } });
  }

  const hud = Deno.readTextFileSync(HUD_PATH);
  return new Response(hud, { headers: { "content-type": "text/html; charset=utf-8" } });
}, { port: PORT, hostname: "0.0.0.0" });