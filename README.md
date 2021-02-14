# workout

Workout Project written in Flutter and Rust.

## Devlog

Flutter app created via:

```sh
flutter create --platforms android,ios workout
```

Rust crates created via:

```sh
cargo new --lib native/workout
cargo new --lib native/workout-ffi
```

Flutter plugin package created via:

```sh
flutter create  --platforms ios,android,macos --template=plugin packages/workout_ffi
```

Ensure to add the `ffi` lib as a dependency of the plugin and then run:

```sh
(cd packages/workout_ffi && flutter pub get)
```

Modify ios setup to make things work, see [this commit](https://github.com/thlorenz/workout-frs/commit/f2e062ac1d9d0299fae46d28181207e220b218d2):

- ios/Classes/SwiftWorkoutFfiPlugin.swift
- ios/workout_ffi.podspec

_In some cases it helps to run `flutter clean` if the `add` method (or whichever you are
exposing) is not found_.
