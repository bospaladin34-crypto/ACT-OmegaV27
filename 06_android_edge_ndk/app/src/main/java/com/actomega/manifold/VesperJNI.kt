package com.actomega.manifold

object VesperJNI {
    init {
        try {
            System.loadLibrary("vesper_ffi")
        } catch (e: Throwable) {
            // Fallback for simulation mode
        }
    }

    external fun initManifoldCore(): Boolean
    external fun quantizeLeech24(input: FloatArray, output: ByteArray): Float
    external fun ingestSensorRecord(bx: Float, by: Float, bz: Float, ax: Float, ay: Float, az: Float, rx: Float, ry: Float, rz: Float, p: Float): Int
}
