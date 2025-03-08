import 'dart:ffi';
import 'dart:typed_data';
import 'dart:async';
import 'dart:isolate';
import 'dart:convert';
import 'load_os.dart';
import 'interface.dart';

/// Sets the exact file path of the dynamic library
/// compiled from the `hub` crate.
void setCompiledLibPathReal(String path) {
  setDynamicLibPath(path);
}

Future<void> prepareInterfaceReal(
  AssignRustSignal assignRustSignal,
) async {
  // Prepare ports for communication over isolates.
  final rustSignalPort = ReceivePort();

  // Listen to Rust via isolate port.
  rustSignalPort.listen((rustSignalRaw) {
    final endpoint = rustSignalRaw[0];
    var messageBytes = rustSignalRaw[1];
    var binary = rustSignalRaw[2];
    if (binary == null) {
      // Rust will send null if the vector is empty.
      // Converting is needed on the Dart side.
      binary = Uint8List(0);
    }
    if (rustSignalRaw[0] == 'RinfPrint') {
      // -1 is a special message ID for Rust reports.
      String rustReport = utf8.decode(binary);
      print(rustReport);
      return;
    }
    if (messageBytes == null) {
      // Rust will send null if the vector is empty.
      // Converting is needed on the Dart side.
      messageBytes = Uint8List(0);
    }
    assignRustSignal[endpoint]!(messageBytes, binary);
  });

  // Make Rust prepare its isolate to send data to Dart.
  // This is handled by `allo_isolate`.
  rustLibrary.prepareIsolate(
    NativeApi.postCObject,
    rustSignalPort.sendPort.nativePort,
  );
}

void startRustLogicReal() {
  rustLibrary.startRustLogic();
}

void stopRustLogicReal() {
  rustLibrary.stopRustLogic();
}

void sendDartSignalReal(
  String endpointSymbol,
  Uint8List messageBytes,
  Uint8List binary,
) {
  final rustLibraryLocal = rustLibrary as RustLibraryLocal;
  rustLibraryLocal.sendDartSignal(endpointSymbol, messageBytes, binary);
}
