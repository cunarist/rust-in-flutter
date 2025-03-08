// Copyright (c) Facebook, Inc. and its affiliates
// SPDX-License-Identifier: MIT OR Apache-2.0

part of 'serde.dart';

@immutable
class Unit {
  const Unit();

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    if (other.runtimeType != runtimeType) return false;

    return other is Unit;
  }

  @override
  int get hashCode => 7;
}
