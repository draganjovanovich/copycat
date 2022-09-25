# Install script for directory: /Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/lib/libcfltk.a")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/lib" TYPE STATIC_LIBRARY FILES "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/build/libcfltk.a")
  if(EXISTS "$ENV{DESTDIR}/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/lib/libcfltk.a" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/lib/libcfltk.a")
    execute_process(COMMAND "/usr/bin/ranlib" "$ENV{DESTDIR}/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/lib/libcfltk.a")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_box.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_browser.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_button.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_dialog.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_draw.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_enums.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_group.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_image.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_input.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_lock.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_macros.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_menu.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_misc.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_printer.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_surface.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_table.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_text.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_tree.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_utils.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_valuator.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_widget.h;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_widget.hpp;/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk/cfl_window.h")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/include/cfltk" TYPE FILE FILES
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_box.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_browser.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_button.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_dialog.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_draw.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_enums.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_group.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_image.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_input.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_lock.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_macros.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_menu.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_misc.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_printer.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_surface.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_table.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_text.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_tree.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_utils.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_valuator.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_widget.h"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_widget.hpp"
    "/Users/macbookair/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_window.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/share/cmake/cfltk/cfltkConfig.cmake")
    file(DIFFERENT EXPORT_FILE_CHANGED FILES
         "$ENV{DESTDIR}/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/share/cmake/cfltk/cfltkConfig.cmake"
         "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/build/CMakeFiles/Export/_Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/share/cmake/cfltk/cfltkConfig.cmake")
    if(EXPORT_FILE_CHANGED)
      file(GLOB OLD_CONFIG_FILES "$ENV{DESTDIR}/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/share/cmake/cfltk/cfltkConfig-*.cmake")
      if(OLD_CONFIG_FILES)
        message(STATUS "Old export file \"$ENV{DESTDIR}/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/share/cmake/cfltk/cfltkConfig.cmake\" will be replaced.  Removing files [${OLD_CONFIG_FILES}].")
        file(REMOVE ${OLD_CONFIG_FILES})
      endif()
    endif()
  endif()
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/share/cmake/cfltk/cfltkConfig.cmake")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/share/cmake/cfltk" TYPE FILE FILES "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/build/CMakeFiles/Export/_Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/share/cmake/cfltk/cfltkConfig.cmake")
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/share/cmake/cfltk/cfltkConfig-release.cmake")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
      message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
      message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    file(INSTALL DESTINATION "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/share/cmake/cfltk" TYPE FILE FILES "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/build/CMakeFiles/Export/_Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/share/cmake/cfltk/cfltkConfig-release.cmake")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/share/cmake/cfltk/cfltkConfigVersion.cmake")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/share/cmake/cfltk" TYPE FILE FILES "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/build/cfltkConfigVersion.cmake")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/build/fltk/cmake_install.cmake")

endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
