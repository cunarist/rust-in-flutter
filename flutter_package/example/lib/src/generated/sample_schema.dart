part of 'generated.dart';

@immutable
class SampleSchema {
  const SampleSchema({
    required this.sampleFieldOne,
    required this.sampleFieldTwo,
  });

  static SampleSchema deserialize(BinaryDeserializer deserializer) {
    deserializer.increaseContainerDepth();
    final instance = SampleSchema(
      sampleFieldOne: deserializer.deserializeBool(),
      sampleFieldTwo: deserializer.deserializeBool(),
    );
    deserializer.decreaseContainerDepth();
    return instance;
  }

  static SampleSchema bincodeDeserialize(Uint8List input) {
    final deserializer = BincodeDeserializer(input);
    final value = SampleSchema.deserialize(deserializer);
    if (deserializer.offset < input.length) {
      throw Exception('Some input bytes were not read');
    }
    return value;
  }

  final bool sampleFieldOne;
  final bool sampleFieldTwo;

  SampleSchema copyWith({
    bool? sampleFieldOne,
    bool? sampleFieldTwo,
  }) {
    return SampleSchema(
      sampleFieldOne: sampleFieldOne ?? this.sampleFieldOne,
      sampleFieldTwo: sampleFieldTwo ?? this.sampleFieldTwo,
    );
  }

  void serialize(BinarySerializer serializer) {
    serializer.increaseContainerDepth();
    serializer.serializeBool(sampleFieldOne);
    serializer.serializeBool(sampleFieldTwo);
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

    return other is SampleSchema
      && sampleFieldOne == other.sampleFieldOne
      && sampleFieldTwo == other.sampleFieldTwo;
  }

  @override
  int get hashCode => Object.hash(
        sampleFieldOne,
        sampleFieldTwo,
      );

  @override
  String toString() {
    String? fullString;

    assert(() {
      fullString = '$runtimeType('
        'sampleFieldOne: $sampleFieldOne, '
        'sampleFieldTwo: $sampleFieldTwo'
        ')';
      return true;
    }());

    return fullString ?? 'SampleSchema';
  }
}
