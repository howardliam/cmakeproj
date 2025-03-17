cmake_minimum_required(VERSION 3.30.0)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_BUILD_TYPE RelWithDebInfo)

project({{PROJECT_NAME}} VERSION 0.1.0 LANGUAGES CXX)

add_executable(${CMAKE_PROJECT_NAME}
    src/main.cpp
)

target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE src/)

if (MSVC)
    target_compile_options(${CMAKE_PROJECT_NAME} PRIVATE /W4)
else()
    target_compile_options(${CMAKE_PROJECT_NAME} PRIVATE -Wall -Wextra -pedantic -fdiagnostics-color=always)
endif()
