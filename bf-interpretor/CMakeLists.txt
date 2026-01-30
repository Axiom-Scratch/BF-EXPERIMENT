cmake_minimum_required(VERSION 3.16)
project(bf_interpretor LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(bf
    src/main.cpp
    src/bf_vm.cpp
    src/bf_brackets.cpp
)

target_include_directories(bf PRIVATE include)
