if(NOT EXISTS "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/build/fltk/install_manifest.txt")
   message(FATAL_ERROR
      "Cannot find install manifest: \"/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/build/fltk/install_manifest.txt\"")
endif(NOT EXISTS "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/build/fltk/install_manifest.txt")

file(READ "/Users/macbookair/Desktop/Projects/copycat/copycat/popup/popup/target/release/build/fltk-sys-c3a6832f0b6e5225/out/build/fltk/install_manifest.txt" files)
string(REGEX REPLACE "\n" ";" files "${files}")

foreach(file ${files})
message(STATUS "Uninstalling \"$ENV{DESTDIR}${file}\"")
   exec_program("/opt/homebrew/Cellar/cmake/3.23.3/bin/cmake"
      ARGS "-E remove -f \"$ENV{DESTDIR}${file}\""
      OUTPUT_VARIABLE rm_out
      RETURN_VALUE rm_retval
   )
   if(NOT "${rm_retval}" STREQUAL 0)
      message(FATAL_ERROR "Problem when removing \"$ENV{DESTDIR}${file}\"")
   endif(NOT "${rm_retval}" STREQUAL 0)
endforeach(file)
