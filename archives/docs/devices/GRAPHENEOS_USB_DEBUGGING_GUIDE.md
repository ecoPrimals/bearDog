# GrapheneOS Pixel 8 USB Debugging Setup Guide

**Date:** 2025-01-08  
**Status:** 🔍 **TROUBLESHOOTING USB CONNECTION**  
**Device:** Pixel 8 with GrapheneOS  
**Host:** Pop!_OS 22.04 LTS

## 🔍 **Current Status**

✅ **USB Hardware Detection:** Working  
- Device detected: `Bus 002 Device 002: ID 18d1:4ee1 Google Inc. Nexus/Pixel Device (MTP)`
- USB connection established
- Currently in MTP (Media Transfer Protocol) mode

❌ **ADB Connection:** Not established  
- `adb devices` shows no devices
- Need to enable USB debugging on GrapheneOS

## 📱 **GrapheneOS USB Debugging Steps**

### **Step 1: Enable Developer Options**
On your Pixel 8 with GrapheneOS:

1. Go to **Settings** → **About phone**
2. Tap **Build number** 7 times rapidly
3. Enter your PIN/password when prompted
4. You'll see "You are now a developer!" message

### **Step 2: Enable USB Debugging (GrapheneOS Specific)**
GrapheneOS has enhanced security features that affect USB debugging:

1. Go to **Settings** → **System** → **Developer options**
2. Enable **USB debugging**
3. **IMPORTANT:** Enable **Wireless debugging** as well (GrapheneOS fallback)
4. Set **Default USB configuration** to **File transfer / Android Auto**
5. Enable **Verify apps over USB** (if available)

### **Step 3: GrapheneOS Security Considerations**
GrapheneOS has additional security measures:

1. **USB Debugging Security:**
   - GrapheneOS may disable USB debugging after reboot
   - Check if "USB debugging" is still enabled after connecting

2. **USB Restricted Mode:**
   - GrapheneOS has stricter USB policies
   - May need to unlock device when connecting USB

3. **Network Security:**
   - GrapheneOS may block some network debugging features
   - Wireless debugging might be more reliable

## 🔧 **Pop!_OS Host Configuration**

### **Step 4: Configure ADB on Pop!_OS**
```bash
# Restart ADB server
sudo adb kill-server
sudo adb start-server

# Check ADB version
adb version

# Add user to plugdev group (if needed)
sudo usermod -a -G plugdev $USER

# Create/update udev rules for Google devices
sudo tee /etc/udev/rules.d/51-android.rules << 'EOF'
# Google Pixel devices
SUBSYSTEM=="usb", ATTR{idVendor}=="18d1", MODE="0666", GROUP="plugdev"
# Google Pixel 8 specific
SUBSYSTEM=="usb", ATTR{idVendor}=="18d1", ATTR{idProduct}=="4ee1", MODE="0666", GROUP="plugdev"
SUBSYSTEM=="usb", ATTR{idVendor}=="18d1", ATTR{idProduct}=="4ee2", MODE="0666", GROUP="plugdev"
EOF

# Reload udev rules
sudo udevadm control --reload-rules
sudo udevadm trigger

# Restart ADB
adb kill-server
adb start-server
```

### **Step 5: Test Connection**
```bash
# Check if device appears
adb devices -l

# If device shows as "unauthorized", accept the prompt on phone
# GrapheneOS will show "Allow USB debugging?" dialog
```

## 🛠️ **GrapheneOS Troubleshooting**

### **Common Issues & Solutions:**

#### **Issue 1: Device Not Detected**
```bash
# Check USB connection
lsusb | grep -i google

# Try different USB cable/port
# GrapheneOS is sensitive to cable quality

# Check dmesg for USB events
dmesg | tail -20
```

#### **Issue 2: "Unauthorized" Device**
- GrapheneOS security dialog may not appear immediately
- Try disconnecting and reconnecting USB
- Ensure device is unlocked when connecting
- Check for dialog in notification shade

#### **Issue 3: GrapheneOS USB Restrictions**
```bash
# Try wireless debugging instead
adb connect <phone_ip>:5555

# Or use network ADB bridge
adb tcpip 5555
```

#### **Issue 4: Persistent MTP Mode**
- GrapheneOS may default to MTP for security
- Change USB mode in developer options
- Try "PTP" or "MIDI" modes if available

## 🔒 **GrapheneOS Security Features**

### **Enhanced Security Measures:**
1. **Stricter USB Policies** - More restrictive than stock Android
2. **Automatic USB Debugging Disable** - May turn off after reboot
3. **Enhanced Authorization** - More secure pairing process
4. **Network Restrictions** - Limited network debugging access

### **Working with GrapheneOS Security:**
- Always unlock device before connecting USB
- Accept security dialogs promptly
- Consider using wireless debugging for development
- Be patient with authorization process

## 🎯 **Next Steps for BearDog Testing**

Once USB debugging is working:

### **Phase 1: Device Verification**
```bash
# Verify connection
adb devices -l
adb shell getprop ro.product.model
adb shell getprop ro.build.version.release

# Check GrapheneOS version
adb shell getprop ro.build.display.id
```

### **Phase 2: Titan M Detection**
```bash
# Check StrongBox availability
adb shell getprop ro.hardware.keystore
adb shell ls /dev/trusty-ipc-dev0

# Verify security features
adb shell getprop ro.hardware.bootctrl
```

### **Phase 3: BearDog Deployment**
```bash
# Deploy BearDog with GrapheneOS optimizations
./scripts/build_android_pixel8.sh --graphene-optimized --enable-strongbox

# Run HSM benchmarks
cargo run --example pixel8_hsm_benchmark
```

## 📋 **Immediate Action Items**

1. **Enable Developer Options** on Pixel 8
2. **Enable USB Debugging** in Developer Options  
3. **Configure udev rules** on Pop!_OS
4. **Test ADB connection**
5. **Accept authorization dialog** on GrapheneOS

**Current Priority:** Get basic ADB connection working first, then proceed with BearDog testing.

---

**GrapheneOS Documentation:**  
- [GrapheneOS Usage Guide](https://grapheneos.org/usage)
- [GrapheneOS Developer Features](https://grapheneos.org/features#developer-features)

Ready to proceed with USB debugging setup! 🚀 