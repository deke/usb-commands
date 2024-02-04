## Execute a command whenever a USB device connects or disconnects.

---

This allows you to, for example switch the input of your display when using a USB switch.
I originally had a M1 Macbook and a PC sharing my Samsung Odyssey G9.
I tried [display-switch](https://github.com/haimgel/display-switch), but it did not fully support M1s.
After looking at it's issues, I came across a [comment](https://github.com/haimgel/ddc-macos-rs/issues/2#issuecomment-1356873695) that used [m1ddc](https://github.com/waydabber/m1ddc) to change the inputs.
However, they needed to comment out some of code in [display-switch](https://github.com/haimgel/display-switch) to get it to work.  Anyhow, I decided it would be a fun Friday evening activity to create a more simplified program that listens for a USB device to connect or disconnect and then execute an associated command.
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

## Configuration

```ini
device_id = "<DEVICE_ID>"
execute_on_connect = "SOME COMMAND"
execute_on_disconnect = "SOME OTHER COMMAND"
```

### Example

```ini
device_id = "<DEVICE_ID>"
execute_on_connect = "/usr/local/bin/m1ddc display <UUID> set input <n>"
execute_on_disconnect = "/usr/local/bin/m1ddc display <UUID> set input <n>"
```

#### Or even simpler

```ini

```

## Find your USB device_id

### Mac

You can find the USB device ID on your Mac by using the built-in System Information application.

1. Open Spotlight by pressing Command + Space.
2. Type System Information and press Enter to open the application.
3. In the System Information window, look for Hardware in the left sidebar and expand it if needed.
4. Click on USB under the Hardware dropdown. This will display all USB devices currently connected to your Mac.
5. If you haven't yet connected your device, do so now and press Cmd + R.
6. Select your USB device from the list. The information about the device, including the Product ID and Vendor ID, will be displayed on the right.
7. The Product ID and Vendor ID together make up the USB device ID. They are usually displayed in hexadecimal format, like this: `0x1234`. We only need the part after the `Ox`, so in the example, `1234`
   - ***Example***: Vendor ID: `0x1a40` and Product ID: `0x0801` become `device_id`: `1a40:0801`

#### Optionally use lsusb

```bash
$ brew install lsusb
$ lsusb > unplugged
# plug in usb device
$ lsusb > plugged
$ diff unplugged plugged
```

### Windows

You can find the USB device ID on your Windows machine by using the built-in Device Manager.

1. Open the Start Menu by pressing the Windows key.
2. Type Device Manager and press Enter to open the application.
3. In the Device Manager window, look for Universal Serial Bus controllers and expand it.
4. Right-click on your USB device from the list and select Properties.
5. In the Properties window, go to the Details tab.
6. In the Property dropdown, select Hardware Ids.
7. The information displayed in the Value box includes the Vendor ID (VID) and Product ID (PID).
8. The Product ID and Vendor ID together make up the USB device ID. They are usually displayed in this format: USB\VID_1234&PID_5678. We only need the parts after the underscores.
   - ***Example***: `USB\VID_1A40&PID_0801` becomes `device_id`: `1a40:0801`

## Manually Install

[Install Rust](https://www.rust-lang.org/tools/install)

---

### Clone

```bash
git clone https://github.com/deke/usb-commands.git
cd usb-commands
```

---

### Build

if on M1 Mac

```bash
rustup target add aarch64-apple-darwin
cargo build --target=aarch64-apple-darwin --release
```

else

```bash
cargo build --release
```

---

### Auto Start

#### Mac

Write your ini file to `~/Library/Application Support/dev.deke.usb-commands/usb-commands.ini`
if M1 Mac

```bash
cp target/aarch64-apple-darwin/release/usb-commands /usr/local/bin 
```

else

```bash
cp target/release/usb-commands /usr/local/bin/
```

then

```bash
cp dev.deke.usb-commands.daemon.plist ~/Library/LaunchAgents/
launchctl load ~/Library/LaunchAgents/dev.deke.usb-commands.daemon.plist
```

#### Windows

Write your ini file to `C:\Users\\dekek\\AppData\\Roaming\\dev.deke.usb-commands/usb-commands.ini`

```bash
copy target\release\usb-commands.exe %APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup
```

---

## Debugging

At present, there is not much in the way of debugging. I recommend running this in the terminal and watching it's output.  Once you have it working, copy the binary to it's proper location.

### Running for debug purposes

if M1 Mac

```bash
cargo run --target=aarch64-apple-darwin 
```

else

```bash
cargo run
```
