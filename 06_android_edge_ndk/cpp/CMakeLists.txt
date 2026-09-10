cmake_minimum_required(VERSION 3.22.1)
project("vesper_ffi")

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(vesper_ffi SHARED
    vesper_jni.cpp
)

target_include_directories(vesper_ffi PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR}/../../02_l1_cpp23_cabi/include
)

target_link_libraries(vesper_ffi
    android
    log
)