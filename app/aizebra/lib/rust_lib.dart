import 'dart:ffi' as ffi;
import 'dart:io' show Platform;
import 'package:ffi/ffi.dart';

ffi.DynamicLibrary _openRustLib() {
  if (Platform.isWindows) return ffi.DynamicLibrary.open('aIzebra.dll');
  if (Platform.isAndroid) return ffi.DynamicLibrary.open('libaizebra.so');
  if (Platform.isMacOS)   return ffi.DynamicLibrary.open('librust_lib.dylib');
  if (Platform.isIOS)     return ffi.DynamicLibrary.process();
  throw UnsupportedError('Unsupported platform');
}

typedef GetLastErrorFunc     = ffi.Pointer<Utf8> Function();
typedef GetLastErrorFuncDart = ffi.Pointer<Utf8> Function();

typedef FreeLastErrorFunc     = ffi.Void Function(ffi.Pointer<Utf8>);
typedef FreeLastErrorFuncDart = void Function(ffi.Pointer<Utf8>);

typedef ParseFunc = ffi.Int32 Function(ffi.Pointer<Utf8>);
typedef ParseFuncDart = int Function(ffi.Pointer<Utf8>);

typedef EvaluateFunc = ffi.Float Function(ffi.Int32, ffi.Float);
typedef EvaluateFuncDart = double Function(int, double);

class Parser {
  ffi.DynamicLibrary dylib;
  String? lastError;

  static final Parser _singleton = Parser._internal();

  factory Parser() {
    return _singleton;
  }

  Parser._internal() : dylib =_openRustLib();

  bool parse(String s){
      final parseFunc = dylib.lookupFunction<ParseFunc, ParseFuncDart>('parse');
      final res = parseFunc(s.toNativeUtf8());
      if (res < 0) {
        lastError = _getLastError();
        return false;
      }
      lastError = null;
      return true;
  }

  double? evaluate(int index, double x){
    final evaluateFunc = dylib.lookupFunction<EvaluateFunc, EvaluateFuncDart>('evaluate');
    final res = evaluateFunc(index, x);

    final error = _getLastError();
    if(error != null && error.isNotEmpty){
      lastError = error;
      return null;
    }
    else{
      lastError = null;
      return res;
    }
  }

  String? _getLastError(){
    final getLastError = dylib.lookupFunction<GetLastErrorFunc, GetLastErrorFuncDart>('get_last_error');
    final freeLastError = dylib.lookupFunction<FreeLastErrorFunc, FreeLastErrorFuncDart>('free_last_error');

    final ptr = getLastError();
    if (ptr == ffi.nullptr) return null;

    try {
      final msg = ptr.toDartString();
      return msg.isEmpty ? null : msg;
    } finally {
      // Always free the memory from Rust
      freeLastError(ptr);
    }
  }
}
