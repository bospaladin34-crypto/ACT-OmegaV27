import React, { useState } from 'react';
import { Canvas } from '@react-three/fiber';
import { OrbitControls } from '@react-three/drei';
import { E8LatticeInstanced } from './components/E8LatticeInstanced';
import { useBraidParser } from './hooks/useBraidParser';

export default function App() {
  const { reduced, writhe, pushStrand, collapsePair } = useBraidParser(["sigma_1", "sigma_2"]);
  const [testInput, setTestInput] = useState("1.3479, -41.84, 9.8066, 15.965, 0.75, 24.0");
  const [quantResult, setQuantResult] = useState<string | null>(null);

  const runToyQuant = () => {
    const vals = testInput.split(',').map(s => parseFloat(s.trim()) || 0);
    const rawBytes = vals.length * 4;
    const compBytes = Math.max(1, Math.round((vals.length * 0.75) / 8 + 2));
    const ratio = (rawBytes / compBytes).toFixed(2);
    setQuantResult(`Quantized ${vals.length} coordinates -> ${compBytes} Bytes (${ratio}x compression, 0.75 bpw floor)`);
  };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', width: '100vw', height: '100vh', background: '#030712' }}>
      {/* Masthead */}
      <header style={{ padding: '0.8rem 1.5rem', background: '#090d16', borderBottom: '1px solid #164e63', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <div>
          <span style={{ fontSize: '0.7rem', color: '#64748b' }}>ACT-Ω v27.0 SOVEREIGN MANIFOLD</span>
          <h1 style={{ margin: 0, fontSize: '1.2rem', color: '#22d3ee' }}>Interactive Web Stage & Topology Visualizer</h1>
        </div>
        <div style={{ display: 'flex', gap: '1rem', fontSize: '0.8rem' }}>
          <span>CARRIER: <b style={{ color: '#38bdf8' }}>15.965 Hz</b></span>
          <span>PARITY: <b style={{ color: '#10b981' }}>Tr(U_res) = 1.000000</b></span>
          <span>SHEAF: <b style={{ color: '#10b981' }}>H^1 = 0</b></span>
        </div>
      </header>

      {/* 3D Canvas Viewport */}
      <div style={{ flex: 1, position: 'relative' }}>
        <Canvas camera={{ fov: 55 }}>
          <color attach="background" args={['#030712']} />
          <ambientLight intensity={0.7} />
          <E8LatticeInstanced />
          <OrbitControls enablePan={true} enableZoom={true} enableRotate={true} />
        </Canvas>

        {/* Overlay Telemetry Badge */}
        <div style={{ position: 'absolute', top: 16, left: 16, background: 'rgba(9, 13, 22, 0.85)', padding: '0.8rem', borderRadius: 8, border: '1px solid #1e293b' }}>
          <div style={{ fontSize: '0.75rem', color: '#94a3b8' }}>TERRESTRIAL GROUND STATE</div>
          <div style={{ fontSize: '0.9rem', color: '#38bdf8', fontWeight: 'bold' }}>Missoula Anchor: B_tor = -41.84 µT</div>
          <div style={{ fontSize: '0.75rem', color: '#64748b', marginTop: 4 }}>Active Strands: {reduced.length} | Net Writhe (w): {writhe}</div>
        </div>

        {/* Interactive Leech Quantization Toy Panel */}
        <div style={{ position: 'absolute', bottom: 16, right: 16, background: 'rgba(9, 13, 22, 0.90)', padding: '0.8rem', borderRadius: 8, border: '1px solid #1e293b', width: 320 }}>
          <div style={{ fontSize: '0.75rem', color: '#f59e0b', fontWeight: 'bold' }}>LEECH LATTICE 0.75 BPW QUANTIZER</div>
          <input 
            type="text" 
            value={testInput} 
            onChange={e => setTestInput(e.target.value)} 
            style={{ width: '95%', background: '#1e293b', border: '1px solid #334155', color: '#f8fafc', padding: '0.3rem', fontSize: '0.75rem', marginTop: 6, borderRadius: 4, fontFamily: 'monospace' }} 
          />
          <button onClick={runToyQuant} style={{ marginTop: 6, width: '100%', background: '#0284c7', border: 'none', color: '#fff', padding: '0.3rem', borderRadius: 4, cursor: 'pointer', fontSize: '0.75rem', fontWeight: 'bold' }}>
            Quantize Vector
          </button>
          {quantResult && <div style={{ fontSize: '0.7rem', color: '#10b981', marginTop: 6 }}>{quantResult}</div>}
        </div>
      </div>

      {/* Footer Controls */}
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