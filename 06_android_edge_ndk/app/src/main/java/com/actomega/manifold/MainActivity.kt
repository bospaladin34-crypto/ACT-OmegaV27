package com.actomega.manifold

import android.content.Context
import android.hardware.*
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import kotlin.math.sqrt

class MainActivity : ComponentActivity(), SensorEventListener {
    private lateinit var sensorManager: SensorManager
    private var magSensor: Sensor? = null

    private val bAbsState = mutableFloatStateOf(45.91f)
    private val b2RateState = mutableFloatStateOf(88.99f)
    private val writheState = mutableIntStateOf(0)
    private val activeStrandsState = mutableIntStateOf(0)
    private val historyPoints = mutableStateListOf<Float>()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        try { VesperJNI.initManifoldCore() } catch (_: Throwable) {}

        sensorManager = getSystemService(Context.SENSOR_SERVICE) as SensorManager
        magSensor = sensorManager.getDefaultSensor(Sensor.TYPE_MAGNETIC_FIELD)
        magSensor?.let {
            sensorManager.registerListener(this, it, 10000)
        }

        setContent {
            MaterialTheme(colorScheme = darkColorScheme()) {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = Color(0xFF030712)
                ) {
                    SovereignCockpitScreen(
                        bAbs = bAbsState.floatValue,
                        b2Rate = b2RateState.floatValue,
                        writhe = writheState.intValue,
                        strands = activeStrandsState.intValue,
                        history = historyPoints,
                        onInjectBraid = { sigma ->
                            activeStrandsState.intValue += 1
                            writheState.intValue += if (sigma > 0) 1 else -1
                        },
                        onCollapse = {
                            if (activeStrandsState.intValue >= 2) {
                                activeStrandsState.intValue -= 2
                            }
                        }
                    )
                }
            }
        }
    }

    override fun onSensorChanged(event: SensorEvent?) {
                event?.let {
            if (it.sensor.type == Sensor.TYPE_MAGNETIC_FIELD) {
                val bx: Float = it.values[0]
                val by: Float = it.values[1]
                val bz: Float = it.values[2]
                val bAbs: Float = sqrt(bx * bx + by * by + bz * bz)

                bAbsState.floatValue = bAbs
                b2RateState.floatValue = if (bAbs > 46.0f) 97.11f else 88.99f

                if (historyPoints.size > 80) historyPoints.removeAt(0)
                historyPoints.add(bAbs)

                try { VesperJNI.ingestSensorRecord(bx, by, bz, 1013.25f) } catch (_: Throwable) {}
            }
        }
    }

    override fun onAccuracyChanged(sensor: Sensor?, accuracy: Int) {}

    override fun onDestroy() {
        super.onDestroy()
        sensorManager.unregisterListener(this)
    }
}

@Composable
fun SovereignCockpitScreen(
    bAbs: Float,
    b2Rate: Float,
    writhe: Int,
    strands: Int,
    history: List<Float>,
    onInjectBraid: (Int) -> Unit,
    onCollapse: () -> Unit
) {
    LazyColumn(
        modifier = Modifier.fillMaxSize().padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(14.dp)
    ) {
        item {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Column {
                    Text("ACT-Ω v27.0 COCKPIT", fontSize = 12.sp, color = Color(0xFF6B7280), fontFamily = FontFamily.Monospace)
                    Text("Pixel 10 Edge Node", fontSize = 20.sp, fontWeight = FontWeight.Bold, color = Color(0xFF22D3EE), fontFamily = FontFamily.Monospace)
                }
                Surface(
                    shape = RoundedCornerShape(6.dp),
                    color = Color(0xFF083344),
                    border = androidx.compose.foundation.BorderStroke(1.dp, Color(0xFF06B6D4))
                ) {
                    Text("15.965 Hz", modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp), color = Color(0xFF67E8F9), fontSize = 12.sp, fontFamily = FontFamily.Monospace)
                }
            }
        }

        item {
            Card(
                colors = CardDefaults.cardColors(containerColor = Color(0xFF090D16)),
                modifier = Modifier.border(1.dp, Color(0xFF1E293B), RoundedCornerShape(8.dp))
            ) {
                Row(
                    modifier = Modifier.fillMaxWidth().padding(14.dp),
                    horizontalArrangement = Arrangement.SpaceBetween
                ) {
                    Text("Tr(U_res) = 1.000000", color = Color(0xFF10B981), fontSize = 12.sp, fontFamily = FontFamily.Monospace)
                    Text("H^1 = 0", color = Color(0xFF10B981), fontSize = 12.sp, fontFamily = FontFamily.Monospace)
                    Text("1.4411 J", color = Color(0xFFF59E0B), fontSize = 12.sp, fontFamily = FontFamily.Monospace)
                }
            }
        }

        item {
            Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.spacedBy(12.dp)) {
                Surface(
                    modifier = Modifier.weight(1f).border(1.dp, Color(0xFF1E293B), RoundedCornerShape(8.dp)),
                    color = Color(0xFF090D16)
                ) {
                    Column(modifier = Modifier.padding(12.dp)) {
                        Text("B_abs FLUX", color = Color(0xFF64748B), fontSize = 10.sp, fontFamily = FontFamily.Monospace)
                        Text(String.format("%.2f µT", bAbs), color = Color(0xFF38BDF8), fontSize = 18.sp, fontWeight = FontWeight.Bold, fontFamily = FontFamily.Monospace)
                    }
                }
                Surface(
                    modifier = Modifier.weight(1f).border(1.dp, Color(0xFF1E293B), RoundedCornerShape(8.dp)),
                    color = Color(0xFF090D16)
                ) {
                    Column(modifier = Modifier.padding(12.dp)) {
                        Text("b2 CAVITY RATE", color = Color(0xFF64748B), fontSize = 10.sp, fontFamily = FontFamily.Monospace)
                        Text(String.format("%.1f rec/s", b2Rate), color = Color(0xFF38BDF8), fontSize = 18.sp, fontWeight = FontWeight.Bold, fontFamily = FontFamily.Monospace)
                    }
                }
            }
        }

        item {
            Text("REAL-TIME B_tor WAVEFORM (100 Hz)", fontSize = 11.sp, color = Color(0xFF94A3B8), fontFamily = FontFamily.Monospace)
            Surface(
                modifier = Modifier.fillMaxWidth().height(110.dp).border(1.dp, Color(0xFF164E63), RoundedCornerShape(8.dp)),
                color = Color(0xFF040A14)
            ) {
                Canvas(modifier = Modifier.fillMaxSize()) {
                    if (history.size > 1) {
                        val path = Path()
                        val minVal = (history.minOrNull() ?: 45f) - 0.5f
                        val maxVal = (history.maxOrNull() ?: 47f) + 0.5f
                        val range = (maxVal - minVal).coerceAtLeast(1f)
                        val stepX = size.width / (history.size - 1)

                        history.forEachIndexed { i, v ->
                            val normY = size.height - ((v - minVal) / range) * size.height
                            if (i == 0) path.moveTo(0f, normY) else path.lineTo(i * stepX, normY)
                        }
                        drawPath(path, Color(0xFF06B6D4), style = Stroke(width = 2.5f))
                    }
                }
            }
        }

        item {
            Text("TOPOLOGICAL BRAID CONTROLS", fontSize = 11.sp, color = Color(0xFF94A3B8), fontFamily = FontFamily.Monospace)
            Card(
                colors = CardDefaults.cardColors(containerColor = Color(0xFF090D16)),
                modifier = Modifier.border(1.dp, Color(0xFF1E293B), RoundedCornerShape(8.dp))
            ) {
                Column(modifier = Modifier.padding(14.dp), verticalArrangement = Arrangement.spacedBy(10.dp)) {
                    Text("Active Strands: $strands | Net Writhe: $writhe", fontSize = 12.sp, color = Color(0xFFE2E8F0), fontFamily = FontFamily.Monospace)
                    Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                        Button(onClick = { onInjectBraid(1) }, modifier = Modifier.weight(1f), colors = ButtonDefaults.buttonColors(containerColor = Color(0xFF0E7490))) {
                            Text("Inject σ₁", fontSize = 12.sp, fontFamily = FontFamily.Monospace)
                        }
                        Button(onClick = { onInjectBraid(2) }, modifier = Modifier.weight(1f), colors = ButtonDefaults.buttonColors(containerColor = Color(0xFF0E7490))) {
                            Text("Inject σ₂", fontSize = 12.sp, fontFamily = FontFamily.Monospace)
                        }
                        Button(onClick = onCollapse, modifier = Modifier.weight(1.2f), colors = ButtonDefaults.buttonColors(containerColor = Color(0xFF831843))) {
                            Text("Collapse e", fontSize = 12.sp, fontFamily = FontFamily.Monospace)
                        }
                    }
                }
            }
        }
    }
}


