import 'dart:io';
import 'dart:ffi';
import 'dart:typed_data';
import 'package:ffi/ffi.dart';
import 'interface.dart';

String? dynamicLibPath;

void setDynamicLibPath(String path) {
  dynamicLibPath = path;
}

RustLibrary loadRustLibrary() {
  // Use provided dynamic library path if possible.
  // Otherewise, use the default path.
  final path = dynamicLibPath;
  DynamicLibrary lib;
  if (path != null) {
    lib = DynamicLibrary.open(path);
  } else if (Platform.isLinux) {
    lib = DynamicLibrary.open('libhub.so');
  } else if (Platform.isAndroid) {
    lib = DynamicLibrary.open('libhub.so');
  } else if (Platform.isWindows) {
    lib = DynamicLibrary.open('hub.dll');
  } else if (Platform.isIOS) {
    lib = DynamicLibrary.open('rinf.framework/rinf');
  } else if (Platform.isMacOS) {
    lib = DynamicLibrary.open('rinf.framework/rinf');
  } else {
    throw UnsupportedError('This operating system is not supported.');
  }

  if (useLocalSpaceSymbols) {
    return RustLibraryLocal(lib: lib);
  } else {
    return RustLibraryGlobal();
  }
}

/// The central interface for calling native function.
final rustLibrary = loadRustLibrary();

// Common type aliases.
// These exist for better readability of the code.

typedef PostCObjectInner = Int8 Function(Int64, Pointer<Dart_CObject>);
typedef PostCObjectPtr = Pointer<NativeFunction<PostCObjectInner>>;
typedef PrepareIsolateExtern = Void Function(PostCObjectPtr, Int64);
typedef PrepareIsolateWrap = void Function(PostCObjectPtr, int);
typedef SendDartSignalExtern = Void Function(
  Pointer<Uint8>,
  UintPtr,
  Pointer<Uint8>,
  UintPtr,
);
typedef SendDartSignalWrap = void Function(
  Pointer<Uint8>,
  int,
  Pointer<Uint8>,
  int,
);

/// Abstract class for unifying the interface
/// for calling native functions.
abstract class RustLibrary {
  void startRustLogic();
  void stopRustLogic();
  void prepareIsolate(PostCObjectPtr storePostObject, int port);
}

/// Class for global native library symbols loaded with `RTLD_GLOBAL`.
/// This is the efficient and ideal way to call native code.
/// `@Native` decorator with `isLeaf` parameter
/// that enables the `Uint8List.address` syntax
/// can only be used on globally loaded native symbols.
/// - https://github.com/dart-lang/sdk/issues/44589
class RustLibraryGlobal extends RustLibrary {
  // Direct access to global function symbols loaded in the process.
  // These are available only if the native library is
  // loaded into global space with `RTLD_GLOBAL` configuration.

  @Native<Void Function()>(
    isLeaf: true,
    symbol: 'rinf_start_rust_logic_extern',
  )
  external static void startRustLogicExtern();

  @Native<Void Function()>(
    isLeaf: true,
    symbol: 'rinf_stop_rust_logic_extern',
  )
  external static void stopRustLogicExtern();

  @Native<Void Function(PostCObjectPtr, Int64)>(
    isLeaf: true,
    symbol: 'rinf_prepare_isolate_extern',
  )
  external static void prepareIsolateExtern(
    PostCObjectPtr storePostObject,
    int port,
  );

  void startRustLogic() {
    startRustLogicExtern();
  }

  void stopRustLogic() {
    stopRustLogicExtern();
  }

  void prepareIsolate(PostCObjectPtr storePostObject, int port) {
    prepareIsolateExtern(storePostObject, port);
  }
}

/// Class for local native library symbols loaded with `RTLD_LOCAL`.
/// This is relatively inefficient because `malloc.allocate` is required.
/// It involves extra memory copy before sending the data to Rust.
class RustLibraryLocal extends RustLibrary {
  final DynamicLibrary lib;
  final Map<String, SendDartSignalWrap> sendDartSignalExterns = {};

  late void Function() startRustLogicExtern;
  late void Function() stopRustLogicExtern;
  late void Function(PostCObjectPtr, int) prepareIsolateExtern;

  RustLibraryLocal({required this.lib}) {
    this.startRustLogicExtern =
        lib.lookupFunction<Void Function(), void Function()>(
      'rinf_start_rust_logic_extern',
    );
    this.stopRustLogicExtern =
        lib.lookupFunction<Void Function(), void Function()>(
      'rinf_stop_rust_logic_extern',
    );
    this.prepareIsolateExtern =
        lib.lookupFunction<PrepareIsolateExtern, PrepareIsolateWrap>(
      'rinf_prepare_isolate_extern',
    );
  }

  void startRustLogic() {
    startRustLogicExtern();
  }

  void stopRustLogic() {
    stopRustLogicExtern();
  }

  void prepareIsolate(PostCObjectPtr storePostObject, int port) {
    prepareIsolateExtern(storePostObject, port);
  }

  void sendDartSignal(
    String endpointSymbol,
    Uint8List messageBytes,
    Uint8List binary,
  ) {
    final Pointer<Uint8> messageMemory = malloc.allocate(messageBytes.length);
    messageMemory.asTypedList(messageBytes.length).setAll(0, messageBytes);

    final Pointer<Uint8> binaryMemory = malloc.allocate(binary.length);
    binaryMemory.asTypedList(binary.length).setAll(0, binary);

    // Cache the dynamic library functions
    // to reduce symbol lookup overhead.
    var sendDartSignalExtern = sendDartSignalExterns[endpointSymbol];
    if (sendDartSignalExtern == null) {
      sendDartSignalExtern =
          lib.lookupFunction<SendDartSignalExtern, SendDartSignalWrap>(
        endpointSymbol,
      );
      sendDartSignalExterns[endpointSymbol] = sendDartSignalExtern;
    }

    sendDartSignalExtern(
      messageMemory,
      messageBytes.length,
      binaryMemory,
      binary.length,
    );

    malloc.free(messageMemory);
    malloc.free(binaryMemory);
  }
}
