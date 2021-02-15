import 'ffi.dart' as ffi;

class Workout {
  int add(int a, int b) {
    return ffi.add(a, b);
  }

  ffi.AppState addStruct(int a, int b) {
    return ffi.add_struct(a, b);
  }
}
