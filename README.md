# An Example Android Application With Rust Integration

This example package demonstrates how to build and call a Rust library from an Android application.

The Rust crate is stored in [app/src/main/rust/example_rust_library](app/src/main/rust/example_rust_library).
If you need to add additional crates you will need to modify [app/src/main/rust/CMakeLists.txt](app/src/main/rust/CMakeLists.txt).
and add any additional CMake targets that need to be built and included in the Android APK/bundle
to the list of targets in [build.gradle.kts](app/build.gradle.kts).

You will need to implement JNI bindings to call Rust code from Kotlin/Java.
On the Rust side, you can use the [JNI](https://docs.rs/jni/latest/jni/) crate.
On the Kotlin/Java side you can follow the [Android NDK documentation](https://developer.android.com/ndk/guides). 

You may need to install [CMake and the Android SDK](https://developer.android.com/studio/projects/install-ndk#install_ndk_and_cmake_automatically).
The Android Gradle Plugin seems to demand that the CMake version in [build.gradle.kts](app/build.gradle.kts) exactly matches the version
installed along side your Android SDK. You may need to update this file so that the version number matches the installed version.

You may need to [install additional Android targets](https://rust-lang.github.io/rustup/cross-compilation.html) that you wish to cross-compile to.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.