import 'dart:ffi';
import 'dart:typed_data';

import 'package:workout_ffi/fluid.dart' as fluid;

String stringFromVec(fluid.Vec_u8 vec) {
  final Uint8List list = vec.ptr.asTypedList(vec.len);
  return new String.fromCharCodes(list);
}

class AppState {
  @Int64()
  final int count;
  final String msg;
  final String err;

  AppState._(this.count, this.msg, this.err);

  factory AppState._from(fluid.AppState x) {
    return new AppState._(
      x.count,
      stringFromVec(x.msg),
      stringFromVec(x.err),
    );
  }
}

class Workout {
  AppState addStruct(int a, int b) {
    final x = fluid.add_struct(a, b);
    return AppState._from(x);
  }
}
