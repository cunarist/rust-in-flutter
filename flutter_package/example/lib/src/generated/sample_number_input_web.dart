// ignore_for_file:unused_import,unused_element
import 'dart:typed_data';
import 'package:rinf/rinf.dart';
import 'generated.dart';

extension SampleNumberInputDartSignalExt on SampleNumberInput {
  void sendSignalToRust() {
    final messageBytes = this.bincodeSerialize();
    final binary = Uint8List(0);
    sendDartSignal(
      'rinf_send_dart_signal_sample_number_input',
      messageBytes,
      binary,
    );
  }
}
