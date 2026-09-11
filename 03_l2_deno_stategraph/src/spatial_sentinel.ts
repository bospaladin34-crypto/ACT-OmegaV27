// spatial_sentinel.ts - Task 30 Virtualized Edge Sentinel Node (Slot 51)
// Provides real-time spatial differential gradient analysis against Slot 50 (Pixel 10)

export interface SentinelFrame {
  slot: number;
  role: string;
  epoch: number;
  timestamp: string;
  referenceMag: { x: number; y: number; z: number };
  referencePressureHpa: number;
  spatialGradientNormUt: number;
  spatialPressureDeltaHpa: number;
  isNonLocalShearDetected: boolean;
  parityTrace: number;
}

export class SpatialSentinelNode {
  public static readonly ASSIGNED_SLOT = 51;
  public static readonly MISSOULA_B_TOR = { x: -13.335, y: 13.640, z: -41.841 };
  public static readonly MISSOULA_PRESSURE_HPA = 905.20;

  public computeGradient(pixel10Frame: any, epoch: number): SentinelFrame {
    const pMag = pixel10Frame.currentMag || SpatialSentinelNode.MISSOULA_B_TOR;
    const pPressure = pixel10Frame.ambientPressureHpa || SpatialSentinelNode.MISSOULA_PRESSURE_HPA;

    const dx = pMag.x - SpatialSentinelNode.MISSOULA_B_TOR.x;
    const dy = pMag.y - SpatialSentinelNode.MISSOULA_B_TOR.y;
    const dz = pMag.z - SpatialSentinelNode.MISSOULA_B_TOR.z;
    const gradNorm = Math.sqrt(dx * dx + dy * dy + dz * dz);
    const dP = Math.abs(pPressure - SpatialSentinelNode.MISSOULA_PRESSURE_HPA);

    return {
      slot: SpatialSentinelNode.ASSIGNED_SLOT,
      role: "VIRTUAL_EDGE_SENTINEL",
      epoch,
      timestamp: new Date().toISOString(),
      referenceMag: { ...SpatialSentinelNode.MISSOULA_B_TOR },
      referencePressureHpa: SpatialSentinelNode.MISSOULA_PRESSURE_HPA,
      spatialGradientNormUt: parseFloat(gradNorm.toFixed(5)),
      spatialPressureDeltaHpa: parseFloat(dP.toFixed(4)),
      isNonLocalShearDetected: gradNorm > 0.053,
      parityTrace: 1.000000,
    };
  }
}