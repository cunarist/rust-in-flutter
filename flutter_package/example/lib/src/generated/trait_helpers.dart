part of 'generated.dart';
class TraitHelpers {
  static void serializeOptionSampleSchema(SampleSchema? value, BinarySerializer serializer) {
    if (value == null) {
        serializer.serializeOptionTag(false);
    } else {
        serializer.serializeOptionTag(true);
        value.serialize(serializer);
    }
  }

  static SampleSchema? deserializeOptionSampleSchema(BinaryDeserializer deserializer) {
    final tag = deserializer.deserializeOptionTag();
    if (tag) {
        return SampleSchema.deserialize(deserializer);
    } else {
        return null;
    }
  }

  static void serializeVectorI32(List<int> value, BinarySerializer serializer) {
    serializer.serializeLength(value.length);
    for (final item in value) {
        serializer.serializeInt32(item);
    }
  }

  static List<int> deserializeVectorI32(BinaryDeserializer deserializer) {
    final length = deserializer.deserializeLength();
    return List.generate(length, (_i) => deserializer.deserializeInt32());
  }

}

