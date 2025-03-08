part of 'generated.dart';

@immutable
class SampleNumberInput {
  const SampleNumberInput({
    required this.letter,
    required this.dummyOne,
    this.dummyTwo,
    required this.dummyThree,
  });

  static SampleNumberInput deserialize(BinaryDeserializer deserializer) {
    deserializer.increaseContainerDepth();
    final instance = SampleNumberInput(
      letter: deserializer.deserializeString(),
      dummyOne: deserializer.deserializeUint32(),
      dummyTwo: TraitHelpers.deserializeOptionSampleSchema(deserializer),
      dummyThree: TraitHelpers.deserializeVectorI32(deserializer),
    );
    deserializer.decreaseContainerDepth();
    return instance;
  }

  static SampleNumberInput bincodeDeserialize(Uint8List input) {
    final deserializer = BincodeDeserializer(input);
    final value = SampleNumberInput.deserialize(deserializer);
    if (deserializer.offset < input.length) {
      throw Exception('Some input bytes were not read');
    }
    return value;
  }

  final String letter;
  final int dummyOne;
  final SampleSchema? dummyTwo;
  final List<int> dummyThree;

  SampleNumberInput copyWith({
    String? letter,
    int? dummyOne,
    SampleSchema? Function()? dummyTwo,
    List<int>? dummyThree,
  }) {
    return SampleNumberInput(
      letter: letter ?? this.letter,
      dummyOne: dummyOne ?? this.dummyOne,
      dummyTwo: dummyTwo == null ? this.dummyTwo : dummyTwo(),
      dummyThree: dummyThree ?? this.dummyThree,
    );
  }

  void serialize(BinarySerializer serializer) {
    serializer.increaseContainerDepth();
    serializer.serializeString(letter);
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

    return other is SampleNumberInput
      && letter == other.letter
      && dummyOne == other.dummyOne
      && dummyTwo == other.dummyTwo
      && listEquals(dummyThree, other.dummyThree);
  }

  @override
  int get hashCode => Object.hash(
        letter,
        dummyOne,
        dummyTwo,
        dummyThree,
      );

  @override
  String toString() {
    String? fullString;

    assert(() {
      fullString = '$runtimeType('
        'letter: $letter, '
        'dummyOne: $dummyOne, '
        'dummyTwo: $dummyTwo, '
        'dummyThree: $dummyThree'
        ')';
      return true;
    }());

    return fullString ?? 'SampleNumberInput';
  }
}
