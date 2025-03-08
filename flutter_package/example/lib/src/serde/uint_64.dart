// Copyright (c) Facebook, Inc. and its affiliates
// SPDX-License-Identifier: MIT OR Apache-2.0

part of 'serde.dart';

///
/// A Dart type to represent the Rust u64 type.
@immutable
class Uint64 {
  Uint64(this._high);

  factory Uint64.parse(String num, {int? radix}) {
    return Uint64.fromBigInt(BigInt.parse(num, radix: radix));
  }

  factory Uint64.fromBigInt(BigInt num) {
    return Uint64(num.toUnsigned(64));
  }

  final BigInt _high;

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    if (other.runtimeType != runtimeType) return false;

    return other is Uint64 && _high == other._high;
  }

  @override
  int get hashCode => _high.hashCode;

  @override
  String toString() {
    return toBigInt().toString();
  }

  BigInt toBigInt() => _high;

  // Warning: If the number does not fit, clamps to the max (or min) integer.
  int toInt() => _high.toInt();
}
