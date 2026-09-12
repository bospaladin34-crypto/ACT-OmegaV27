#!/data/data/com.termux/files/usr/bin/bash
SESSION=${1:-"Missoula_BeltGravels_Survey"}
echo "=================================================================="
echo " [ACT-OMEGA V27.0]: STARTING UNTETHERED FIELD EXPEDITION         "
echo " Session: $SESSION"
echo "=================================================================="

# Acquire Android WakeLock to prevent Doze mode CPU/sensor sleeping
echo "-> Acquiring Termux WakeLock..."
termux-wake-lock

echo "-> Initializing high-frequency field logger..."
deno run --allow-net --allow-read --allow-write --allow-run /data/local/tmp/missoula_field_expedition.ts "$SESSION"