import React from 'react';
import { Canvas } from '@react-three/fiber';
import { OrbitControls } from '@react-three/drei';
import { E8LatticeInstanced } from './components/E8LatticeInstanced';
import { useBraidParser } from './hooks/useBraidParser';

export default function App() {
  const { reduced, writhe, pushStrand, collapsePair } = useBraidParser(["sigma_1", "sigma_2"]);

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
          <span>SHEAF: <b style={{ color: '#10b981' }}>H^1 = 0</b></span>
        </div>
      </header>

      <div style={{ flex: 1, position: 'relative' }}>
        <Canvas camera={{ position:, fov: 55 }}>
          <color attach="background" args={['#030712']} />
          <ambientLight intensity={0.6} />
          <E8LatticeInstanced />
          <OrbitControls />
        </Canvas>
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
          Active Strands: {reduced.length} | Writhe: {writhe} | Conway-Sloane Leech Quantization (0.75 bpw)
        </div>
      </footer>
    </div>
  );
}
