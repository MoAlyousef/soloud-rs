cmake_minimum_required(VERSION 3.0)
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

file(GLOB_RECURSE AUDIOSOURCE ${CMAKE_CURRENT_LIST_DIR}/soloud/src/audiosource/*.c*)
file(GLOB_RECURSE CORE ${CMAKE_CURRENT_LIST_DIR}/soloud/src/core/*.c*)
file(GLOB_RECURSE FILTER ${CMAKE_CURRENT_LIST_DIR}/soloud/src/filter/*.c*)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_INSTALL_PREFIX}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_INSTALL_PREFIX}/lib)

set(BACKEND_SRCS ${CMAKE_CURRENT_LIST_DIR}/soloud/src/backend/miniaudio/soloud_miniaudio.cpp)

set(SOLOUD_SRCS 
    ${CMAKE_CURRENT_LIST_DIR}/soloud_new.cpp
    ${BACKEND_SRCS}
    ${CMAKE_CURRENT_LIST_DIR}/soloud/src/c_api/soloud_c.cpp
    ${AUDIOSOURCE}
    ${CORE}
    ${FILTER}
)

add_library(soloud STATIC ${SOLOUD_SRCS})
target_include_directories(soloud PUBLIC ${CMAKE_CURRENT_LIST_DIR}/soloud/include)
target_compile_definitions(soloud PUBLIC "WITH_MINIAUDIO=1" "WITH_TOOLS=0")

install(TARGETS soloud
    DESTINATION ${CMAKE_INSTALL_PREFIX}/lib
    )
