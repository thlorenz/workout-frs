#import "WorkoutFfiPlugin.h"
#if __has_include(<workout_ffi/workout_ffi-Swift.h>)
#import <workout_ffi/workout_ffi-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "workout_ffi-Swift.h"
#endif

@implementation WorkoutFfiPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftWorkoutFfiPlugin registerWithRegistrar:registrar];
}
@end
