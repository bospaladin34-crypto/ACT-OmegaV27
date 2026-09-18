package com.actomega.manifold

import android.app.*
import android.content.Intent
import android.hardware.*
import android.os.IBinder
import android.util.Log
import okhttp3.*
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.RequestBody.Companion.toRequestBody
import java.net.DatagramPacket
import java.net.DatagramSocket
import java.net.InetAddress
import java.nio.ByteBuffer
import java.nio.ByteOrder
import java.util.concurrent.Executors

class ManifoldEdgeService : Service(), SensorEventListener {
    private lateinit var sensorManager: SensorManager
    private var magSensor: Sensor? = null
    private var accelSensor: Sensor? = null
    private var pressureSensor: Sensor? = null
    private var rotationSensor: Sensor? = null

    // Caches for 9-DOF + Pressure
    private var ax = 0f; private var ay = 0f; private var az = 0f
    private var rx = 0f; private var ry = 0f; private var rz = 0f
    private var p = 1013.25f

    // Networking
    private var udpSocket: DatagramSocket? = null
    private var broadcastAddress: InetAddress? = null
    private val executor = Executors.newSingleThreadExecutor()
    private var webSocket: WebSocket? = null
    private val client = OkHttpClient()

    override fun onCreate() {
        super.onCreate()
        try { VesperJNI.initManifoldCore() } catch (_: Throwable) {}

        sensorManager = getSystemService(SENSOR_SERVICE) as SensorManager
        magSensor = sensorManager.getDefaultSensor(Sensor.TYPE_MAGNETIC_FIELD)
        accelSensor = sensorManager.getDefaultSensor(Sensor.TYPE_ACCELEROMETER)
        pressureSensor = sensorManager.getDefaultSensor(Sensor.TYPE_PRESSURE)
        rotationSensor = sensorManager.getDefaultSensor(Sensor.TYPE_ROTATION_VECTOR)
        
        // Register listeners (10,000 microseconds = 100 Hz)
        val rate = 10000
        magSensor?.let { sensorManager.registerListener(this, it, rate) }
        accelSensor?.let { sensorManager.registerListener(this, it, rate) }
        pressureSensor?.let { sensorManager.registerListener(this, it, rate) }
        rotationSensor?.let { sensorManager.registerListener(this, it, rate) }

        startForeground(1, createNotification())
        
        initNetworking()
    }

    private fun initNetworking() {
        executor.execute {
            try {
                udpSocket = DatagramSocket()
                udpSocket?.broadcast = true
                broadcastAddress = InetAddress.getByName("255.255.255.255")
            } catch (e: Exception) {
                e.printStackTrace()
            }

            try {
                val request = Request.Builder().url("ws://127.0.0.1:8098").build()
                webSocket = client.newWebSocket(request, object : WebSocketListener() {
                    override fun onOpen(webSocket: WebSocket, response: Response) {
                        Log.i("ManifoldEdgeService", "WebSocket opened")
                    }
                    override fun onFailure(webSocket: WebSocket, t: Throwable, response: Response?) {
                        Log.e("ManifoldEdgeService", "WebSocket failed", t)
                    }
                })
            } catch (e: Exception) {
                e.printStackTrace()
            }
        }
    }

    override fun onSensorChanged(event: SensorEvent?) {
        event?.let {
            when (it.sensor.type) {
                Sensor.TYPE_ACCELEROMETER -> {
                    ax = it.values[0]
                    ay = it.values[1]
                    az = it.values[2]
                }
                Sensor.TYPE_ROTATION_VECTOR -> {
                    val rotMatrix = FloatArray(9)
                    SensorManager.getRotationMatrixFromVector(rotMatrix, it.values)
                    val orientation = FloatArray(3)
                    SensorManager.getOrientation(rotMatrix, orientation)
                    
                    rx = Math.toDegrees(orientation[0].toDouble()).toFloat() // yaw
                    ry = Math.toDegrees(orientation[1].toDouble()).toFloat() // pitch
                    rz = Math.toDegrees(orientation[2].toDouble()).toFloat() // roll
                }
                Sensor.TYPE_PRESSURE -> {
                    p = it.values[0]
                }
                Sensor.TYPE_MAGNETIC_FIELD -> {
                    val bx = it.values[0]
                    val by = it.values[1]
                    val bz = it.values[2]
                    
                    // Math acceleration
                    try {
                        VesperJNI.ingestSensorRecord(bx, by, bz, ax, ay, az, rx, ry, rz, p)
                    } catch (_: Throwable) {}

                    // Broadcast over P2P network
                    broadcastTelemetry(bx, by, bz, ax, ay, az, rx, ry, rz, p)
                }
            }
        }
    }
    
    private fun broadcastTelemetry(bx: Float, by: Float, bz: Float, ax: Float, ay: Float, az: Float, rx: Float, ry: Float, rz: Float, p: Float) {
        // Send JSON over WebSocket
        val json = "{\"bx\":${bx},\"by\":${by},\"bz\":${bz},\"ax\":${ax},\"ay\":${ay},\"az\":${az},\"rx\":${rx},\"ry\":${ry},\"rz\":${rz},\"p\":${p}}"
        webSocket?.send(json)

        // Send JSON over HTTP POST
        executor.execute {
            try {
                val mediaType = "application/json; charset=utf-8".toMediaType()
                val orientationJson = "{\"alpha\":${rx},\"beta\":${ry},\"gamma\":${rz}}"
                val body = orientationJson.toRequestBody(mediaType)
                val request = Request.Builder()
                    .url("http://127.0.0.1:8098/api/telemetry/orientation")
                    .post(body)
                    .build()
                client.newCall(request).execute().close()
            } catch (e: Exception) {
                // Ignore transient errors
            }
        }

        // Send binary over UDP
        executor.execute {
            try {
                val buffer = ByteBuffer.allocate(40).order(ByteOrder.LITTLE_ENDIAN)
                buffer.putFloat(bx).putFloat(by).putFloat(bz)
                buffer.putFloat(ax).putFloat(ay).putFloat(az)
                buffer.putFloat(rx).putFloat(ry).putFloat(rz)
                buffer.putFloat(p)
                
                val packet = DatagramPacket(buffer.array(), buffer.capacity(), broadcastAddress, 8098)
                udpSocket?.send(packet)
            } catch (e: Exception) {
                // Ignore transient errors
            }
        }
    }

    override fun onAccuracyChanged(sensor: Sensor?, accuracy: Int) {}
    override fun onBind(intent: Intent?): IBinder? = null

    override fun onDestroy() {
        super.onDestroy()
        sensorManager.unregisterListener(this)
        webSocket?.close(1000, null)
        udpSocket?.close()
        executor.shutdown()
    }

    private fun createNotification(): Notification {
        val channelId = "act_omega_edge"
        val channel = NotificationChannel(channelId, "Manifold Service", NotificationManager.IMPORTANCE_LOW)
        getSystemService(NotificationManager::class.java).createNotificationChannel(channel)
        return Notification.Builder(this, channelId)
            .setContentTitle("ACT-Omega Edge Active")
            .setContentText("9-DOF Telemetry Streaming via P2P")
            .setSmallIcon(android.R.drawable.stat_notify_sync)
            .build()
    }
}
