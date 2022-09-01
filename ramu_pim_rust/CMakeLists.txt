
cmake_minimum_required(VERSION 3.10)

project(ramu_pim_rust)
include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.2.1 # Optionally specify a commit hash, version tag or branch here
)

FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH ./Cargo.toml)

add_library(ramu_pim_rust_interface ${CMAKE_CURRENT_SOURCE_DIR}/ramu_pim_rust.cc)
target_include_directories(ramu_pim_rust_interface PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_link_libraries(ramu_pim_rust_interface PUBLIC ramu_pim_rust)

add_executable(ramu_pim_rust_test ${CMAKE_CURRENT_SOURCE_DIR}/ramu_pim_rust_test.cc)
target_link_libraries(ramu_pim_rust_test PUBLIC ramu_pim_rust ramu_pim_rust_interface)