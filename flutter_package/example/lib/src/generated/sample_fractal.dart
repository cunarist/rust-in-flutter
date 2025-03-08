part of 'generated.dart';

@immutable
class SampleFractal {
  static final rustSignalStream =
      sampleFractalStreamController.stream.asBroadcastStream();

  const SampleFractal({
    required this.currentScale,
    this.dummy,
  });

  static SampleFractal deserialize(BinaryDeserializer deserializer) {
    deserializer.increaseContainerDepth();
    final instance = SampleFractal(
      currentScale: deserializer.deserializeFloat64(),
      dummy: TraitHelpers.deserializeOptionSampleSchema(deserializer),
    );
    deserializer.decreaseContainerDepth();
    return instance;
  }

  static SampleFractal bincodeDeserialize(Uint8List input) {
    final deserializer = BincodeDeserializer(input);
    final value = SampleFractal.deserialize(deserializer);
    if (deserializer.offset < input.length) {
      throw Exception('Some input bytes were not read');
    }
    return value;
  }

  final double currentScale;
  final SampleSchema? dummy;

  SampleFractal copyWith({
    double? currentScale,
    SampleSchema? Function()? dummy,
  }) {
    return SampleFractal(
      currentScale: currentScale ?? this.currentScale,
      dummy: dummy == null ? this.dummy : dummy(),
    );
  }

  void serialize(BinarySerializer serializer) {
    serializer.increaseContainerDepth();
    serializer.serializeFloat64(currentScale);
    TraitHelpers.serializeOptionSampleSchema(dummy, serializer);
    serializer.decreaseContainerDepth();
  }

  Uint8List bincodeSerialize() {
      final serializer = BincodeSerializer();
      serialize(serializer);
      return serializer.bytes;
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    if (other.runtimeType != runtimeType) return false;

    return other is SampleFractal
      && currentScale == other.currentScale
      && dummy == other.dummy;
  }

  @override
  int get hashCode => Object.hash(
        currentScale,
        dummy,
      );

  @override
  String toString() {
    String? fullString;

    assert(() {
      fullString = '$runtimeType('
        'currentScale: $currentScale, '
        'dummy: $dummy'
        ')';
      return true;
    }());

    return fullString ?? 'SampleFractal';
  }
}

final sampleFractalStreamController =
    StreamController<RustSignal<SampleFractal>>();
