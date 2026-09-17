import { useFrame } from '@react-three/fiber';
import { useRef, useMemo } from 'react';
import * as THREE from 'three';

const TAU_S = 0.062636;

interface Props {
  euler?: { alpha: number; beta: number; gamma: number } | null;
}

export function E8LatticeInstanced({ euler }: Props) {
  const meshRef = useRef<THREE.InstancedMesh>(null);
  const acc = useRef(0);

  useFrame((_, delta) => {
    acc.current += delta;
    if (acc.current >= TAU_S) {
      acc.current %= TAU_S;
      if (meshRef.current) {
        if (euler && (euler.alpha !== 0 || euler.beta !== 0 || euler.gamma !== 0)) {
          // Direct physical gimbal binding from phone
          meshRef.current.rotation.y = THREE.MathUtils.degToRad(euler.alpha);
          meshRef.current.rotation.x = THREE.MathUtils.degToRad(euler.beta);
          meshRef.current.rotation.z = THREE.MathUtils.degToRad(euler.gamma);
        } else {
          meshRef.current.rotation.y += 0.015;
        }
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