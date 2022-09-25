# Generated by CMake

if("${CMAKE_MAJOR_VERSION}.${CMAKE_MINOR_VERSION}" LESS 2.6)
   message(FATAL_ERROR "CMake >= 2.6.0 required")
endif()
cmake_policy(PUSH)
cmake_policy(VERSION 2.6...3.21)
#----------------------------------------------------------------
# Generated CMake target import file.
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Protect against multiple inclusion, which would fail when already imported targets are added once more.
set(_targetsDefined)
set(_targetsNotDefined)
set(_expectedTargets)
foreach(_expectedTarget fluid fltk_z fltk_jpeg fltk_png fltk fltk_forms fltk_images)
  list(APPEND _expectedTargets ${_expectedTarget})
  if(NOT TARGET ${_expectedTarget})
    list(APPEND _targetsNotDefined ${_expectedTarget})
  endif()
  if(TARGET ${_expectedTarget})
    list(APPEND _targetsDefined ${_expectedTarget})
  endif()
endforeach()
if("${_targetsDefined}" STREQUAL "${_expectedTargets}")
  unset(_targetsDefined)
  unset(_targetsNotDefined)
  unset(_expectedTargets)
  set(CMAKE_IMPORT_FILE_VERSION)
  cmake_policy(POP)
  return()
endif()
if(NOT "${_targetsDefined}" STREQUAL "")
  message(FATAL_ERROR "Some (but not all) targets in this export set were already defined.\nTargets Defined: ${_targetsDefined}\nTargets not yet defined: ${_targetsNotDefined}\n")
endif()
unset(_targetsDefined)
unset(_targetsNotDefined)
unset(_expectedTargets)


# Create imported target fluid
add_executable(fluid IMPORTED)
set_property(TARGET fluid PROPERTY MACOSX_BUNDLE 1)

# Create imported target fltk_z
add_library(fltk_z STATIC IMPORTED)

# Create imported target fltk_jpeg
add_library(fltk_jpeg STATIC IMPORTED)

# Create imported target fltk_png
add_library(fltk_png STATIC IMPORTED)

set_target_properties(fltk_png PROPERTIES
  INTERFACE_LINK_LIBRARIES "fltk_z"
)

# Create imported target fltk
add_library(fltk STATIC IMPORTED)

set_target_properties(fltk PROPERTIES
  INTERFACE_LINK_LIBRARIES "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/usr/lib/libdl.tbd"
)

# Create imported target fltk_forms
add_library(fltk_forms STATIC IMPORTED)

set_target_properties(fltk_forms PROPERTIES
  INTERFACE_LINK_LIBRARIES "fltk"
)

# Create imported target fltk_images
add_library(fltk_images STATIC IMPORTED)

set_target_properties(fltk_images PROPERTIES
  INTERFACE_LINK_LIBRARIES "fltk;fltk_jpeg;fltk_z;fltk_png"
)

# Import target "fluid" for configuration "Release"
set_property(TARGET fluid APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fluid PROPERTIES
  IMPORTED_LOCATION_RELEASE "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/debug/build/fltk-sys-7f240e83d61d99b2/out/build/fltk/bin/fluid.app/Contents/MacOS/fluid"
  )

# Import target "fltk_z" for configuration "Release"
set_property(TARGET fltk_z APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fltk_z PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "C"
  IMPORTED_LOCATION_RELEASE "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/debug/build/fltk-sys-7f240e83d61d99b2/out/build/fltk/lib/libfltk_z.a"
  )

# Import target "fltk_jpeg" for configuration "Release"
set_property(TARGET fltk_jpeg APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fltk_jpeg PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "C"
  IMPORTED_LOCATION_RELEASE "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/debug/build/fltk-sys-7f240e83d61d99b2/out/build/fltk/lib/libfltk_jpeg.a"
  )

# Import target "fltk_png" for configuration "Release"
set_property(TARGET fltk_png APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fltk_png PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "C"
  IMPORTED_LOCATION_RELEASE "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/debug/build/fltk-sys-7f240e83d61d99b2/out/build/fltk/lib/libfltk_png.a"
  )

# Import target "fltk" for configuration "Release"
set_property(TARGET fltk APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fltk PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "C;CXX"
  IMPORTED_LOCATION_RELEASE "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/debug/build/fltk-sys-7f240e83d61d99b2/out/build/fltk/lib/libfltk.a"
  )

# Import target "fltk_forms" for configuration "Release"
set_property(TARGET fltk_forms APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fltk_forms PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/debug/build/fltk-sys-7f240e83d61d99b2/out/build/fltk/lib/libfltk_forms.a"
  )

# Import target "fltk_images" for configuration "Release"
set_property(TARGET fltk_images APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fltk_images PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/debug/build/fltk-sys-7f240e83d61d99b2/out/build/fltk/lib/libfltk_images.a"
  )

# This file does not depend on other imported targets which have
# been exported from the same project but in a separate export set.

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
cmake_policy(POP)
