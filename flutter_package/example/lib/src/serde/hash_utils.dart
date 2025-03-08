// Copyright (c) Facebook, Inc. and its affiliates
// SPDX-License-Identifier: MIT OR Apache-2.0

part of 'serde.dart';

const maxInt = 4294967296;

bool listEquals<T>(List<T>? a, List<T>? b) {
  if (a == null) return b == null;
  if (b == null || a.length != b.length) return false;
  if (identical(a, b)) return true;
  for (int index = 0; index < a.length; index += 1) {
    if (!_elementEquals(a[index], b[index])) return false;
  }
  return true;
}

bool mapEquals<T, U>(Map<T, U>? a, Map<T, U>? b) {
  if (a == null) return b == null;
  if (b == null || a.length != b.length) return false;
  if (identical(a, b)) return true;
  for (final T key in a.keys) {
    if (!a.containsKey(key) || !_elementEquals(a[key], b[key])) {
      return false;
    }
  }
  return true;
}

bool _elementEquals<T>(T? a, T? b) {
  if (a is List && b is List) {
    return listEquals(a, b);
  } else if (a is Map && b is Map) {
    return mapEquals(a, b);
  }
  return a == b;
}
