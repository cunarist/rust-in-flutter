part of 'generated.dart';

final assignRustSignal = <String, void Function(Uint8List, Uint8List)>{
  'SampleFractal': (Uint8List messageBytes, Uint8List binary) {
    final message = SampleFractal.bincodeDeserialize(messageBytes);
    final rustSignal = RustSignal(
      message,
      binary,
    );
    sampleFractalStreamController.add(rustSignal);
  },
  'SampleNumberOutput': (Uint8List messageBytes, Uint8List binary) {
    final message = SampleNumberOutput.bincodeDeserialize(messageBytes);
    final rustSignal = RustSignal(
      message,
      binary,
    );
    sampleNumberOutputStreamController.add(rustSignal);
  },
};
