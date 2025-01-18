cmake_minimum_required(VERSION 3.31.1)
project("example_rust_library")

include(FetchContent)

FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.5.1
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH example_rust_library/Cargo.toml)

add_custom_target(example_android_rust_library DEPENDS cargo-build_example_rust_library)
