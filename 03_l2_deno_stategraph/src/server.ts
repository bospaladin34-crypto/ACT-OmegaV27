// ACT-Omega v27.0 - L2 Deno Telemetry Server (Task 58 Ingress)
import { serve } from "https://deno.land/std@0.224.0/http/server.ts";

let latestTelemetry = {
  type: "phone_telemetry",
  alpha: 0,
  beta: 0,
  gamma: 0,
  ax: 0,
  ay: 0,
  az: 9.80665,
  mx: -13.335,
  my: 13.640,
  mz: -41.841,
  magTotal: 45.98,
  accelTotal: 9.80665,
  timestamp: Date.now()
};

const corsHeaders = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Methods": "GET, POST, OPTIONS",
  "Access-Control-Allow-Headers": "Content-Type",
};

serve(async (req: Request) => {
  const url = new URL(req.url);
  const pathname = url.pathname;

  if (req.method === "OPTIONS") {
    return new Response(null, { headers: corsHeaders });
  }

  if (req.method === "POST" && pathname === "/api/telemetry/orientation") {
    try {
      const body = await req.json();
      latestTelemetry = {
        type: "phone_telemetry",
        alpha: Number(body.alpha) || 0,
        beta: Number(body.beta) || 0,
        gamma: Number(body.gamma) || 0,
        ax: Number(body.ax) || 0,
        ay: Number(body.ay) || 0,
        az: typeof body.az !== 'undefined' ? Number(body.az) : 9.80665,
        mx: typeof body.mx !== 'undefined' ? Number(body.mx) : -13.335,
        my: typeof body.my !== 'undefined' ? Number(body.my) : 13.640,
        mz: typeof body.mz !== 'undefined' ? Number(body.mz) : -41.841,
        magTotal: Number(body.magTotal) || 45.98,
        accelTotal: Number(body.accelTotal) || 9.80665,
        timestamp: Date.now()
      };
      return new Response(JSON.stringify({ status: "ok" }), { headers: corsHeaders });
    } catch {
      return new Response(JSON.stringify({ error: "Invalid JSON" }), { status: 400, headers: corsHeaders });
    }
  }

  if (req.method === "GET" && pathname === "/api/telemetry/orientation") {
    return new Response(JSON.stringify(latestTelemetry), {
      headers: { ...corsHeaders, "Content-Type": "application/json" }
    });
  }

  if (pathname === "/mobile" || pathname === "/mobile_hud.html") {
    try {
      const content = await Deno.readTextFile("C:/sovereign_manifold_v27/00_orchestration_ps51/visualizer/mobile_hud.html");
      return new Response(content, {
        headers: { "Content-Type": "text/html; charset=utf-8" }
      });
    } catch (e) {
      return new Response("mobile_hud.html not found: " + e, { status: 404 });
    }
  }

  return new Response("ACT-Omega v27.0 Telemetry Ingress Active", {
    headers: { ...corsHeaders, "Content-Type": "text/plain" }
  });
}, { port: 8098 });

console.log("ACT-Omega Task 58 9-DOF Ingress Server listening on http://127.0.0.1:8098");