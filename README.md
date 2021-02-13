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
flutter create  --platforms ios,android --template=plugin packages/workout_ffi
```
