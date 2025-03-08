// ignore_for_file: unused_import
library generated_types;

import 'dart:typed_data';
import 'package:meta/meta.dart';
import 'package:tuple/tuple.dart';
import '../serde/serde.dart';
import '../bincode/bincode.dart';

import 'dart:async';
import 'package:rinf/rinf.dart';

export '../serde/serde.dart';

export 'sample_fractal_os.dart'
    if (dart.library.js_interop) 'sample_fractal_web.dart';
export 'sample_number_input_os.dart'
    if (dart.library.js_interop) 'sample_number_input_web.dart';
export 'sample_number_output_os.dart'
    if (dart.library.js_interop) 'sample_number_output_web.dart';
export 'sample_schema_os.dart'
    if (dart.library.js_interop) 'sample_schema_web.dart';

part 'trait_helpers.dart';
part 'sample_fractal.dart';
part 'sample_number_input.dart';
part 'sample_number_output.dart';
part 'sample_schema.dart';
part 'signal_handlers.dart';
