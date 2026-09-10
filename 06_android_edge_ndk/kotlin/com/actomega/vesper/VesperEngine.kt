package com.actomega.vesper

import java.io.Closeable
import java.nio.ByteBuffer

/**
 * ACT-Omega v27.0 Android 17 NDK Engine Wrapper
 * Target Silicon: Google Pixel 10 (Tensor G5 NPU) & Pixel 8 (Tensor G3)
 */
class VesperEngine : Closeable {
    private var nativeHandle: Long = 0L

    companion object {
        init {
            try {
                System.loadLibrary("vesper_ffi")
            } catch (e: UnsatisfiedLinkError) {
                System.err.println("[WARN]: libvesper_ffi.so not found in standard lib path: ${e.message}")
            }
        }
    }

    init {
        nativeHandle = createEngine()
    }

    fun verifyParity(): Float {
        return verifyParity(nativeHandle)
    }

    fun transformZeroCopy(buffer: ByteBuffer): ByteBuffer {
        require(buffer.isDirect) { "Buffer must be allocated via ByteBuffer.allocateDirect()" }
        return transformDirectBuffer(buffer, buffer.capacity(), nativeHandle)
    }

    override fun close() {
        if (nativeHandle != 0L) {
            freeEngine(nativeHandle)
            nativeHandle = 0L
        }
    }

    // Native JNI Declarations
    private external fun createEngine(): Long
    private external fun verifyParity(handle: Long): Float
    private external fun transformDirectBuffer(buffer: ByteBuffer, capacity: Int, handle: Long): ByteBuffer
    private external fun freeEngine(handle: Long)
}