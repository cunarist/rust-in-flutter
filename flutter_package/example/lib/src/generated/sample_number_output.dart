part of 'generated.dart';

@immutable
class SampleNumberOutput {
  static final rustSignalStream =
      sampleNumberOutputStreamController.stream.asBroadcastStream();

  const SampleNumberOutput({
    required this.currentNumber,
    required this.dummyOne,
    this.dummyTwo,
    required this.dummyThree,
  });

  static SampleNumberOutput deserialize(BinaryDeserializer deserializer) {
    deserializer.increaseContainerDepth();
    final instance = SampleNumberOutput(
      currentNumber: deserializer.deserializeInt32(),
      dummyOne: deserializer.deserializeUint32(),
      dummyTwo: TraitHelpers.deserializeOptionSampleSchema(deserializer),
      dummyThree: TraitHelpers.deserializeVectorI32(deserializer),
    );
    deserializer.decreaseContainerDepth();
    return instance;
  }

  static SampleNumberOutput bincodeDeserialize(Uint8List input) {
    final deserializer = BincodeDeserializer(input);
    final value = SampleNumberOutput.deserialize(deserializer);
    if (deserializer.offset < input.length) {
      throw Exception('Some input bytes were not read');
    }
    return value;
  }

  final int currentNumber;
  final int dummyOne;
  final SampleSchema? dummyTwo;
  final List<int> dummyThree;

  SampleNumberOutput copyWith({
    int? currentNumber,
    int? dummyOne,
    SampleSchema? Function()? dummyTwo,
    List<int>? dummyThree,
  }) {
    return SampleNumberOutput(
      currentNumber: currentNumber ?? this.currentNumber,
      dummyOne: dummyOne ?? this.dummyOne,
      dummyTwo: dummyTwo == null ? this.dummyTwo : dummyTwo(),
      dummyThree: dummyThree ?? this.dummyThree,
    );
  }

  void serialize(BinarySerializer serializer) {
    serializer.increaseContainerDepth();
    serializer.serializeInt32(currentNumber);
    serializer.serializeUint32(dummyOne);
    TraitHelpers.serializeOptionSampleSchema(dummyTwo, serializer);
    TraitHelpers.serializeVectorI32(dummyThree, serializer);
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

    return other is SampleNumberOutput
      && currentNumber == other.currentNumber
      && dummyOne == other.dummyOne
      && dummyTwo == other.dummyTwo
      && listEquals(dummyThree, other.dummyThree);
  }

  @override
  int get hashCode => Object.hash(
        currentNumber,
        dummyOne,
        dummyTwo,
        dummyThree,
      );

  @override
  String toString() {
    String? fullString;

    assert(() {
      fullString = '$runtimeType('
        'currentNumber: $currentNumber, '
        'dummyOne: $dummyOne, '
        'dummyTwo: $dummyTwo, '
        'dummyThree: $dummyThree'
        ')';
      return true;
    }());

    return fullString ?? 'SampleNumberOutput';
  }
}

final sampleNumberOutputStreamController =
    StreamController<RustSignal<SampleNumberOutput>>();
