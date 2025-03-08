// ignore_for_file:unused_import,unused_element
import 'dart:ffi';
import 'dart:typed_data';
import 'package:rinf/rinf.dart';
import 'generated.dart';

typedef _SendDartSignalExtern = Void Function(
  Pointer<Uint8>,
  UintPtr,
  Pointer<Uint8>,
  UintPtr,
);

extension SampleNumberInputDartSignalExt on SampleNumberInput {
  @Native<_SendDartSignalExtern>(
    isLeaf: true,
    symbol: 'rinf_send_dart_signal_sample_number_input',
  )
  external static void sendDartSignalExtern(
    Pointer<Uint8> messageBytesAddress,
    int messageBytesLength,
    Pointer<Uint8> binaryAddress,
    int binaryLength,
  );

  void sendSignalToRust() {
    final messageBytes = this.bincodeSerialize();
    final binary = Uint8List(0);
    if (useLocalSpaceSymbols) {
      sendDartSignal(
        'rinf_send_dart_signal_sample_number_input',
        messageBytes,
        binary,
      );
    } else {
      sendDartSignalExtern(
        messageBytes.address,
        messageBytes.length,
        binary.address,
        binary.length,
      );
    }
  }
}
