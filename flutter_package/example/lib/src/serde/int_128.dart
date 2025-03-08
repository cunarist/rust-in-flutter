// Copyright (c) Facebook, Inc. and its affiliates
// SPDX-License-Identifier: MIT OR Apache-2.0

part of 'serde.dart';

@immutable
class Int128 {
  Int128(this.high, this.low);

  factory Int128.parse(String num, {int? radix}) {
    return Int128.fromBigInt(BigInt.parse(num, radix: radix));
  }

  factory Int128.fromBigInt(BigInt num) {
    final input = num.toSigned(128);
    final high = (input >> 64).toSigned(64);
    final low = (input & BigInt.parse('0xFFFFFFFFFFFFFFFF')).toSigned(64);
    return Int128(high, low);
  }

  final BigInt high;
  final BigInt low;

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    if (other.runtimeType != runtimeType) return false;

    return other is Int128 &&
        high.toSigned(64) == other.high.toSigned(64) &&
        low.toUnsigned(64) == other.low.toUnsigned(64);
  }

  @override
  int get hashCode => Object.hash(
        high,
        low,
      );

  @override
  String toString() {
    return toBigInt().toString();
  }

  BigInt toBigInt() => (high.toSigned(64) << 64) + low.toUnsigned(64);
}
