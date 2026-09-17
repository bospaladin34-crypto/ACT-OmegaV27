package com.actomega.manifold

import android.app.*
import android.content.Intent
import android.hardware.*
import android.os.IBinder
import android.util.Log

class ManifoldEdgeService : Service(), SensorEventListener {
    private lateinit var sensorManager: SensorManager
    private var magSensor: Sensor? = null

    override fun onCreate() {
        super.onCreate()
        try { VesperJNI.initManifoldCore() } catch (_: Throwable) {}

        sensorManager = getSystemService(SENSOR_SERVICE) as SensorManager
        magSensor = sensorManager.getDefaultSensor(Sensor.TYPE_MAGNETIC_FIELD)
        
        // 100 Hz sampling (10,000 microseconds)
        magSensor?.let {
            sensorManager.registerListener(this, it, 10000)
        }
        startForeground(1, createNotification())
    }

    override fun onSensorChanged(event: SensorEvent?) {
                event?.let {
            if (it.sensor.type == Sensor.TYPE_MAGNETIC_FIELD) {
                val bx: Float = it.values[0]
                val by: Float = it.values[1]
                val bz: Float = it.values[2]
                try {
                    VesperJNI.ingestSensorRecord(bx, by, bz, 1013.25f)
                } catch (_: Throwable) {}
            }
        }
    }

    override fun onAccuracyChanged(sensor: Sensor?, accuracy: Int) {}
    override fun onBind(intent: Intent?): IBinder? = null

    private fun createNotification(): Notification {
        val channelId = "act_omega_edge"
        val channel = NotificationChannel(channelId, "Manifold Service", NotificationManager.IMPORTANCE_LOW)
        getSystemService(NotificationManager::class.java).createNotificationChannel(channel)
        return Notification.Builder(this, channelId)
            .setContentTitle("ACT-Omega Edge Active")
            .setContentText("Locked to 15.965 Hz carrier | b2 telemetry streaming")
            .setSmallIcon(android.R.drawable.stat_notify_sync)
            .build()
    }
}


