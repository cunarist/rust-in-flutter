// Copyright (c) Facebook, Inc. and its affiliates
// SPDX-License-Identifier: MIT OR Apache-2.0

part of 'serde.dart';

///
/// A Dart type to represent the Rust u128 type.
@immutable
class Uint128 {
  Uint128(this.high, this.low);

  factory Uint128.parse(String num, {int? radix}) {
    return Uint128.fromBigInt(BigInt.parse(num, radix: radix));
  }

  factory Uint128.fromBigInt(BigInt num) {
    final input = num.toUnsigned(128);
    final high = (input >> 64).toUnsigned(64);
    final low = (input & BigInt.parse('0xFFFFFFFFFFFFFFFF')).toUnsigned(64);
    return Uint128(high, low);
  }

  final BigInt high;
  final BigInt low;

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    if (other.runtimeType != runtimeType) return false;

    return other is Uint128 && high == other.high && low == other.low;
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

  BigInt toBigInt() => (high.toUnsigned(64) << 64) + low.toUnsigned(64);
}
