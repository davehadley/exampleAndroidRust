# An Example Android Application With Rust Integration

This example package demonstrates how to build and call a Rust library from an Android application.

The Rust crate is stored in `app/src/rust/main/example_rust_library`. 
If you need to add additional crates you will need to modify `app/src/rust/main/CMakeLists.txt`
and add any additional CMake targets that need to be built and included in the Android APK/bundle
to the list of targets in `build.gradle.kts`.

You will need to implement JNI bindings to call Rust code from Kotlin/Java.
On the Rust side, you can use the [JNI](https://docs.rs/jni/latest/jni/) crate.
On the Kotlin/Java side you can follow the [Android NDK documentation](https://developer.android.com/ndk/guides). 

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.