cmake_minimum_required(VERSION 3.16)
project(zenohc_perf LANGUAGES C)

set(CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake" ${CMAKE_MODULE_PATH})
include(helpers)
set_default_build_type(Release)
configure_include_project(ZENOHC zenohc zenohc::lib ".." zenohc "https://github.com/eclipse-zenoh/zenoh-c" "main")
add_custom_target(perfs ALL)

file(GLOB files "${CMAKE_CURRENT_SOURCE_DIR}/src/*.c")

foreach(file ${files})
    get_filename_component(target ${file} NAME_WE)

    add_executable(${target} EXCLUDE_FROM_ALL ${file})
    add_dependencies(perfs ${target})

    add_dependencies(${target} zenohc::lib)
    target_link_libraries(${target} PRIVATE zenohc::lib)
    copy_dlls(${target})

    set_property(TARGET ${target} PROPERTY C_STANDARD 11)
endforeach()
