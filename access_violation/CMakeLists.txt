cmake_minimum_required(VERSION 3.22)
project(demo)

set(SOURCES main.c)
set(TARGET demo)

add_executable(${TARGET} WIN32 ${SOURCES})
