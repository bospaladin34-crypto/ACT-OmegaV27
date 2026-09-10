#!/bin/bash
# ===================================================================================
# Install Termux API package for Direct Hardware Sensor HAL access on Android 17
# ===================================================================================

echo "================================================================="
echo " [ACT-OMEGA V27.0]: CONFIGURING PIXEL 10 SENSOR HAL PACKAGES"
echo "================================================================="

pkg update -y
pkg install -y termux-api

echo "  [OK]: termux-api package installed."
echo "  [TEST]: Polling hardware sensor list..."
termux-sensor -l

echo "================================================================="