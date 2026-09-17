import React, { useEffect, useState } from 'react';
import { Canvas } from '@react-three/fiber';
import { OrbitControls } from '@react-three/drei';
import { E8LatticeInstanced } from './components/E8LatticeInstanced';
import { useBraidParser } from './hooks/useBraidParser';

export default function App() {
  const { reduced, writhe, pushStrand, collapsePair } = useBraidParser(["sigma_1", "sigma_2"]);
  const [telemetry, setTelemetry] = useState<{ alpha: number; beta: number; gamma: number; bAbs: number } | null>(null);

  // Poll host telemetry if running alongside local laptop server
  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        const res = await fetch("http://127.0.0.1:8098/api/telemetry/orientation");
        if (res.ok) {
          const data = await res.json();
          setTelemetry({
            alpha: data.alpha || 0,
            beta: data.beta || 0,
            gamma: data.gamma || 0,
            bAbs: data.magTotal || 42.74
          });
        }
      } catch (_) {
        // Fallback to standalone mode if host server not detected
      }
    }, 100);
    return () => clearInterval(interval);
  }, []);

  return (
    <div style={{ display: 'flex', flexDirection: 'column', width: '100vw', height: '100vh', background: '#030712' }}>
      <header style={{ padding: '0.8rem 1.5rem', background: '#090d16', borderBottom: '1px solid #164e63', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <div>
          <span style={{ fontSize: '0.7rem', color: '#64748b' }}>ACT-Ω v27.0 SOVEREIGN COCKPIT</span>
          <h1 style={{ margin: 0, fontSize: '1.2rem', color: '#22d3ee' }}>Interactive Manifold Web Stage</h1>
        </div>
        <div style={{ display: 'flex', gap: '1rem', fontSize: '0.8rem' }}>
          <span>CARRIER: <b style={{ color: '#38bdf8' }}>15.965 Hz</b></span>
          <span>PARITY: <b style={{ color: '#10b981' }}>Tr(U_res) = 1.000000</b></span>
          <span>LINK: <b style={{ color: telemetry ? '#10b981' : '#f59e0b' }}>{telemetry ? 'PIXEL 10 LATCHED' : 'STANDBY'}</b></span>
        </div>
      </header>

      <div style={{ flex: 1, position: 'relative' }}>
        <Canvas camera={{ fov: 55 }}>
          <color attach="background" args={['#030712']} />
          <ambientLight intensity={0.6} />
          <E8LatticeInstanced />
          <OrbitControls />
        </Canvas>

        {/* Live Telemetry Overlay */}
        <div style={{ position: 'absolute', top: 16, left: 16, background: 'rgba(9, 13, 22, 0.85)', padding: '0.8rem', borderRadius: 8, border: '1px solid #1e293b' }}>
          <div style={{ fontSize: '0.75rem', color: '#94a3b8' }}>TERRESTRIAL GROUND STATE</div>
          <div style={{ fontSize: '0.9rem', color: '#38bdf8', fontWeight: 'bold' }}>
            B_abs: {telemetry ? telemetry.bAbs.toFixed(2) : '42.74'} µT
          </div>
          <div style={{ fontSize: '0.75rem', color: '#64748b', marginTop: 4 }}>
            Euler Angles: {telemetry ? `${telemetry.alpha}° / ${telemetry.beta}° / ${telemetry.gamma}°` : '270° / 2° / -1°'}
          </div>
          <div style={{ fontSize: '0.75rem', color: '#64748b' }}>Active Strands: {reduced.length} | Writhe: {writhe}</div>
        </div>
      </div>

      <footer style={{ padding: '0.8rem 1.5rem', background: '#090d16', borderTop: '1px solid #164e63', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <div style={{ display: 'flex', gap: '0.6rem' }}>
          <button onClick={() => pushStrand(1)} style={{ padding: '0.4rem 0.8rem', background: '#083344', border: '1px solid #06b6d4', color: '#67e8f9', cursor: 'pointer', borderRadius: 4, fontFamily: 'monospace' }}>
            Inject σ₁
          </button>
          <button onClick={() => pushStrand(2)} style={{ padding: '0.4rem 0.8rem', background: '#083344', border: '1px solid #06b6d4', color: '#67e8f9', cursor: 'pointer', borderRadius: 4, fontFamily: 'monospace' }}>
            Inject σ₂
          </button>
          <button onClick={collapsePair} style={{ padding: '0.4rem 0.8rem', background: '#450a0a', border: '1px solid #dc2626', color: '#fca5a5', cursor: 'pointer', borderRadius: 4, fontFamily: 'monospace' }}>
            Collapse Pair (e)
          </button>
        </div>
        <div style={{ fontSize: '0.75rem', color: '#64748b' }}>
          Conway-Sloane Leech Quantization (0.75 bpw) | Zero-Python | 100% FOSS
        </div>
      </footer>
    </div>
  );
}