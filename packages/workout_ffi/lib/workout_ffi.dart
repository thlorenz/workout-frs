import 'dart:ffi';
import 'dart:typed_data';

import 'package:workout_ffi/ffi.dart' as ffi;

String stringFromVec(ffi.Vec_uint8_t vec) {
  final Uint8List list = vec.ptr.asTypedList(vec.len);
  return new String.fromCharCodes(list);
}

class AppState {
  @Int64()
  final int count;
  final String msg;
  final String err;

  AppState._(this.count, this.msg, this.err);

  factory AppState._from(ffi.AppState_t x) {
    return new AppState._(
      x.count,
      stringFromVec(x.msg),
      stringFromVec(x.err),
    );
  }
}

class Workout {
  AppState addStruct(int a, int b) {
    final x = ffi.add_struct(a, b);
    return AppState._from(x);
  }
}
