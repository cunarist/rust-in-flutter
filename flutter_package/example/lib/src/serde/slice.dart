// Copyright (c) Facebook, Inc. and its affiliates
// SPDX-License-Identifier: MIT OR Apache-2.0

part of 'serde.dart';

@immutable
class Slice {
  const Slice(this.start, this.end);

  final int start;
  final int end;

  // Lexicographic comparison between the (unsigned!) bytes referenced by `slice1` and `slice2`
  // into `content`.
  static int compareBytes(Uint8List content, Slice slice1, Slice slice2) {
    final start1 = slice1.start;
    final end1 = slice1.end;
    final start2 = slice2.start;
    final end2 = slice2.end;
    final il = end1 - start1;

    for (var i = 0; i < il; i++) {
      final byte1 = content[start1 + i] & 0xFF;
      if (start2 + i >= end2) {
        return 1;
      }
      final byte2 = content[start2 + i] & 0xFF;
      if (byte1 > byte2) {
        return 1;
      }
      if (byte1 < byte2) {
        return -1;
      }
    }

    if (end2 - start2 > end1 - start1) {
      return -1;
    }

    return 0;
  }
}
