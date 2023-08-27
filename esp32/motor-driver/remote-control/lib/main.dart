import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_blue_plus/flutter_blue_plus.dart';
import 'package:flutter_joystick/flutter_joystick.dart';
import 'package:permission_handler/permission_handler.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();

  SystemChrome.setPreferredOrientations([
    DeviceOrientation.landscapeRight,
    DeviceOrientation.landscapeLeft,
  ]);

  SystemChrome.setEnabledSystemUIMode(SystemUiMode.manual, overlays: [SystemUiOverlay.bottom]);

  [
    Permission.location,
    Permission.storage,
    Permission.bluetooth,
    Permission.bluetoothConnect,
    Permission.bluetoothScan,
  ].request().then((status) {
    runApp(
      const MaterialApp(
        debugShowCheckedModeBanner: false,
        home: JoystickApp(),
      ),
    );
  });
}

class JoystickApp extends StatefulWidget {
  const JoystickApp({Key? key, this.adapterState}) : super(key: key);

  final BluetoothAdapterState? adapterState;

  @override
  JoystickAppState createState() => JoystickAppState();
}

enum JoystickPosition { left, right }

class JoystickAppState extends State<JoystickApp> {
  BluetoothDevice? device;
  BluetoothService? service;
  BluetoothCharacteristic? startCharacteristic;
  List<int> previousState = [0, 0, 0, 0];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SafeArea(
        child: Stack(
          children: [
            Align(
              alignment: const Alignment(-0.8, 0),
              child: Joystick(
                mode: JoystickMode.horizontal,
                period: const Duration(milliseconds: 20),
                listener: (details) async {
                  await writeDirection(JoystickPosition.left, details.x, details.y);
                },
              ),
            ),
            Align(
              alignment: const Alignment(0, 0),
              child: ElevatedButton(
                  child: const Text("Connect"),
                  onPressed: () async {
                    await FlutterBluePlus.turnOn();
                    await FlutterBluePlus.stopScan();
                    FlutterBluePlus.startScan(timeout: const Duration(seconds: 15), androidUsesFineLocation: false);

                    for (var device in await FlutterBluePlus.connectedSystemDevices) {
                      debugPrint("==================== Disconnecting from ${device.localName}");
                      await device.disconnect();
                    }

                    try {
                      var subscription = await FlutterBluePlus.scanResults
                          .expand((element) => element)
                          .firstWhere((ScanResult element) => element.device.localName == "Hot Wheels");

                      if (subscription == null) {
                        debugPrint("==================== Device not found");
                        return;
                      }

                      await FlutterBluePlus.stopScan();
                      await subscription.device.connect();
                      await subscription.device.discoverServices();

                      for (var service in subscription.device.servicesList ?? []) {
                        debugPrint("==================== Services: $service");
                      }

                      setState(() {
                        device = subscription.device;

                        service = subscription.device.servicesList?.firstWhere((BluetoothService service) {
                          return service.serviceUuid == Guid("00000000-0000-0000-0000-000000000000");
                        });

                        startCharacteristic = service?.characteristics.firstWhere((BluetoothCharacteristic characteristic) {
                          return characteristic.uuid == Guid("00000000-0000-0000-0000-000000000001");
                        });
                      });
                    } catch (e) {
                      debugPrint("==================== Error: $e");
                    }
                  }),
            ),
            Align(
              alignment: const Alignment(0.8, 0),
              child: Joystick(
                mode: JoystickMode.vertical,
                period: const Duration(milliseconds: 20),
                listener: (details) async {
                  await writeDirection(JoystickPosition.right, details.x, details.y);
                },
              ),
            ),
          ],
        ),
      ),
    );
  }

  Future<void> writeDirection(JoystickPosition position, double x, double y) async {
    var xValue = (x.abs() * 100).clamp(0, 100).toInt();
    var yValue = (y.abs() * 100).clamp(0, 100).toInt();

    var xSignal = xValue == 0 ? 2 : x.isNegative ? 2 : 1;
    var ySignal = yValue == 0 ? 2 : y.isNegative ? 2 : 1;

    var payload = [0, 0, 0, 0];

    if (position == JoystickPosition.left) {
      payload = [xSignal, xValue, previousState[2], previousState[3]];
    } else {
      payload = [previousState[0], previousState[1], ySignal, yValue];
    }

    /**
     * Dont need to write if the payload is the same as the previous state
     */
    if (listEquals(payload, previousState)) {
      debugPrint("==================== Skipping: $payload");
    } else {
      setState(() {
        previousState = payload;
      });

      debugPrint("==================== Writing: $payload");

      startCharacteristic?.write(payload, withoutResponse: true);
    }
  }
}
