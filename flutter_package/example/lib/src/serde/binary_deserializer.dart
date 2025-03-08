// Copyright (c) Facebook, Inc. and its affiliates
// SPDX-License-Identifier: MIT OR Apache-2.0

part of 'serde.dart';

abstract class BinaryDeserializer {
  BinaryDeserializer({
    required Uint8List input,
    required this.containerDepthBudget,
  }) : input = ByteData.view(input.buffer);

  @protected
  final ByteData input;
  int _offset = 0;
  int containerDepthBudget;

  int get offset {
    return _offset;
  }

  bool deserializeBool() {
    final result = input.getUint8(_offset);
    _offset += 1;
    if (result == 0) {
      return false;
    } else if (result == 1) {
      return true;
    } else {
      throw Exception(
        'Invalid boolean: expected 0 or 1, but got ${result}',
      );
    }
  }

  Unit deserializeUnit() {
    return const Unit();
  }

  int deserializeUint8() {
    final result = input.getUint8(_offset);
    _offset += 1;
    return result;
  }

  int deserializeUint16() {
    final result = input.getUint16(_offset, Endian.little);
    _offset += 2;
    return result;
  }

  int deserializeUint32() {
    final result = input.getUint32(_offset, Endian.little);
    _offset += 4;
    return result;
  }

  Uint64 deserializeUint64() {
    final number = _bytesToBigInt(8, signed: false);
    _offset += 8;
    return Uint64(number);
  }

  int deserializeInt8() {
    final result = input.getInt8(_offset);
    _offset += 1;
    return result;
  }

  int deserializeInt16() {
    final result = input.getInt16(_offset, Endian.little);
    _offset += 2;
    return result;
  }

  int deserializeInt32() {
    final result = input.getInt32(_offset, Endian.little);
    _offset += 4;
    return result;
  }

  int deserializeInt64() {
    final result = input.getInt64(_offset, Endian.little);
    _offset += 8;
    return result;
  }

  double deserializeFloat32() {
    final result = input.getFloat32(_offset, Endian.little);
    _offset += 4;
    return result;
  }

  double deserializeFloat64() {
    final result = input.getFloat64(_offset, Endian.little);
    _offset += 8;
    return result;
  }

  Bytes deserializeBytes() {
    return Bytes(deserializeUint8List());
  }

  Uint8List deserializeUint8List() {
    final len = deserializeLength();
    if (len < 0 || len > maxInt) {
      throw Exception('The length of an array cannot exceed MAXINT');
    }
    final content = Uint8List(len);
    for (var i = 0; i < len; i++) {
      content[i] = deserializeUint8();
    }
    return content;
  }

  bool deserializeOptionTag() {
    return deserializeBool();
  }

  int deserializeChar() {
    return deserializeInt64();
  }

  int deserializeVariantIndex();

  String deserializeString() {
    return utf8.decode(deserializeUint8List());
  }

  int deserializeLength();

  Int128 deserializeInt128() {
    final low = deserializeUint64();
    final high = deserializeUint64();
    return Int128(high.toBigInt(), low.toBigInt());
  }

  Uint128 deserializeUint128() {
    final low = deserializeUint64();
    final high = deserializeUint64();
    return Uint128(high.toBigInt(), low.toBigInt());
  }

  void checkThatKeySlicesAreIncreasing(Slice key1, Slice key2);

  BigInt _bytesToBigInt(int byteLength, {required bool signed}) {
    BigInt number = BigInt.from(0);
    for (int i = 0; (i < byteLength); i++) {
      // big endian
      // number += BigInt.from(bytes[byteLength - i - 1]) << (8 * i);

      // little endian
      number += BigInt.from(input.getUint8(_offset + i)) << (8 * i);
    }

    if (signed) {
      return number.toSigned(byteLength * 8);
    } else {
      return number.toUnsigned(byteLength * 8);
    }
  }

  void increaseContainerDepth() {
    if (containerDepthBudget == 0) {
      throw Exception('exceeded maximum container depth');
    }
    containerDepthBudget -= 1;
  }

  void decreaseContainerDepth() {
    containerDepthBudget += 1;
  }
}
