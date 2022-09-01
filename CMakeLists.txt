cmake_minimum_required(VERSION 3.10)
set(CMAKE_CXX_STANDARD 20)


project(ramu_pim)
include(FetchContent)
FetchContent_Declare(
    fmt
    GIT_REPOSITORY https://github.com/fmtlib/fmt
)
FetchContent_MakeAvailable(fmt)


add_subdirectory(ramu_pim_rust)
add_subdirectory(ramulator)