cmake_minimum_required(VERSION 3.15)
project(soloud)

set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(MSVC)
	string(REGEX REPLACE "/GR" "/GR-" CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS}")
    string(REGEX REPLACE "/EHsc" "/EHs-c-" CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS}")
else()
	set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fno-rtti -fno-exceptions")
endif()

add_compile_options("$<$<C_COMPILER_ID:MSVC>:/utf-8>")
add_compile_options("$<$<CXX_COMPILER_ID:MSVC>:/utf-8>")

option(WITH_MINIAUDIO "Use the miniaudio backend" OFF)
option(WITH_SDL2_STATIC "Use the SDL2-static backend" OFF)
option(WITH_SDL2 "Use the SDL2 backend" OFF)
option(WITH_ALSA "Use the Alsa backend" OFF)
option(WITH_PORTAUDIO "Use the Portaudio backend" OFF)
option(WITH_COREAUDIO "Use the Coreaudio backend" OFF)
option(WITH_XAUDIO2 "Use the Xaudio2 backend" OFF)
option(WITH_OSS "Use the Oss backend" OFF)
option(WITH_OPENSLES "Use the OpenSLES backend" OFF)
option(WITH_NULL "Use the null driver backend" OFF)
option(WITH_NOSOUND "Use the nosound backend" OFF)
option(WITH_JACK "Use the Jack backend" OFF)
option(WITH_OPENAL "Use the Openal backend" OFF)
option(WITH_WINMM "Use the Winmm backend" OFF)
option(WITH_WASAPI "Use the Wasapi backend" OFF)

file(GLOB_RECURSE AUDIOSOURCE ${CMAKE_CURRENT_LIST_DIR}/soloud/src/audiosource/*.c*)
file(GLOB_RECURSE CORE ${CMAKE_CURRENT_LIST_DIR}/soloud/src/core/*.c*)
file(GLOB_RECURSE FILTER ${CMAKE_CURRENT_LIST_DIR}/soloud/src/filter/*.c*)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_INSTALL_PREFIX}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_INSTALL_PREFIX}/lib)
set(BACKEND_DIR ${CMAKE_CURRENT_LIST_DIR}/soloud/src/backend)

if (WITH_MINIAUDIO)
    set(BACKEND_SRCS ${BACKEND_DIR}/miniaudio/soloud_miniaudio.cpp)
    add_compile_definitions(WITH_MINIAUDIO)
elseif (WITH_NULL)
    set(BACKEND_SRCS ${BACKEND_DIR}/null/soloud_null.cpp)
    add_compile_definitions(WITH_NULL)
elseif (WITH_NOSOUND)
    set(BACKEND_SRCS ${BACKEND_DIR}/nosound/soloud_nosound.cpp)
    add_compile_definitions(WITH_NOSOUND)
elseif (WITH_OPENAL)
    set(BACKEND_SRCS ${BACKEND_DIR}/openal/soloud_openal.cpp ${BACKEND_DIR}/openal/soloud_openal_dll.c)
    add_compile_definitions(WITH_OPENAL)
elseif (WITH_OPENSLES)
    set(BACKEND_SRCS ${BACKEND_DIR}/opensles/soloud_opensles.cpp)
    add_compile_definitions(WITH_OPENSLES)
elseif (WITH_OSS)
    set(BACKEND_SRCS ${BACKEND_DIR}/oss/soloud_oss.cpp)
    add_compile_definitions(WITH_OSS)
elseif (WITH_ALSA)
    set(BACKEND_SRCS ${BACKEND_DIR}/alsa/soloud_alsa.cpp)
    add_compile_definitions(WITH_ALSA)
elseif (WITH_SDL2_STATIC)
    set(BACKEND_SRCS ${BACKEND_DIR}/sdl2_static/soloud_sdl2_static.cpp)
    add_compile_definitions(WITH_SDL2_STATIC)
elseif (WITH_SDL2)
    set(BACKEND_SRCS ${BACKEND_DIR}/sdl/soloud_sdl2.cpp ${BACKEND_DIR}/sdl/soloud_sdl2_dll.c)
    add_compile_definitions(WITH_SDL2)
elseif (WITH_PORTAUDIO)
    set(BACKEND_SRCS ${BACKEND_DIR}/portaudio/soloud_portaudio.cpp ${BACKEND_DIR}/portaudio/soloud_portaudio_dll.c)
    add_compile_definitions(WITH_PORTAUDIO)
elseif (WITH_COREAUDIO)
    set(BACKEND_SRCS ${BACKEND_DIR}/coreaudio/soloud_coreaudio.cpp)
    add_compile_definitions(WITH_COREAUDIO)
elseif (WITH_JACK)
    set(BACKEND_SRCS ${BACKEND_DIR}/jack/soloud_jack.cpp)
    add_compile_definitions(WITH_JACK)
elseif (WITH_XAUDIO2)
    set(BACKEND_SRCS ${BACKEND_DIR}/xaudio2/soloud_xaudio2.cpp)
    add_compile_definitions(WITH_XAUDIO2)
elseif (WITH_WINMM)
    set(BACKEND_SRCS ${BACKEND_DIR}/winmm/soloud_winmm.cpp)
    add_compile_definitions(WITH_WINMM)
elseif (WITH_WASAPI)
    set(BACKEND_SRCS ${BACKEND_DIR}/wasapi/soloud_wasapi.cpp)
    add_compile_definitions(WITH_WASAPI)
else()
    message(FATAL_ERROR "Unsupported backend!")
endif()

set(SOLOUD_SRCS 
    ${CMAKE_CURRENT_LIST_DIR}/soloud_new.cpp
    ${CMAKE_CURRENT_LIST_DIR}/soloud_derives.cpp
    ${BACKEND_SRCS}
    ${CMAKE_CURRENT_LIST_DIR}/soloud/src/c_api/soloud_c.cpp
    ${AUDIOSOURCE}
    ${CORE}
    ${FILTER}
)

add_library(soloud STATIC ${SOLOUD_SRCS})
target_include_directories(soloud PUBLIC ${CMAKE_CURRENT_LIST_DIR}/soloud/include)

install(TARGETS soloud
    DESTINATION ${CMAKE_INSTALL_PREFIX}/lib
    )

