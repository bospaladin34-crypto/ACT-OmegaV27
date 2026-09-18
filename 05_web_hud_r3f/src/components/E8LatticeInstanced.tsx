import { useFrame } from '@react-three/fiber';
import { useRef, useMemo } from 'react';
import * as THREE from 'three';

const TAU_S = 0.062636;

export function E8LatticeInstanced() {
  const meshRef = useRef<THREE.InstancedMesh>(null);
  const acc = useRef(0);

  const roots = useMemo(() => {
    const arr = new Float32Array(240 * 3);
    for (let i = 0; i < 240; i++) {
      const phi = (i / 240) * Math.PI * 2;
      const theta = (i % 16) * (Math.PI / 8);
      arr[i * 3] = Math.cos(phi) * Math.sin(theta) * 2.0;
      arr[i * 3 + 1] = Math.sin(phi) * Math.sin(theta) * 2.0;
      arr[i * 3 + 2] = Math.cos(theta) * 2.0;
    }
    return arr;
  }, []);

  useFrame((_, delta) => {
    acc.current += delta;
    if (acc.current >= TAU_S) {
      acc.current %= TAU_S;
      if (meshRef.current) {
        meshRef.current.rotation.y += 0.015;
        meshRef.current.rotation.x += 0.008;
      }
    }
  });

  return (
    <instancedMesh ref={meshRef} args={[undefined, undefined, 240]}>
      <sphereGeometry />
      <meshBasicMaterial color={0x22d3ee} wireframe />
    </instancedMesh>
  );
}