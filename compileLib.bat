@echo off
REM ===================================================================
REM Copy two folders and one file from a .bat, using relative paths
REM ===================================================================

REM Get the directory where this .bat file is located
set "SCRIPT_DIR=%~dp0"

cargo ndk -t x86_64 -o ./bin build --release

cargo ndk -t arm64-v8a -o ./bin build --release

cargo build --release --target-dir ./bin/windows --lib

REM Define relative source folders and file
set "SRC_FOLDER1=%SCRIPT_DIR%\bin\x86_64"
set "SRC_FOLDER2=%SCRIPT_DIR%\bin\arm64-v8a"
set "SRC_FILE=%SCRIPT_DIR%\bin\windows\release\aizebra.dll"

REM Define relative destination folder (relative to this script)
set "DEST_BASE=%SCRIPT_DIR%\app\aizebra\android\app\src\main\jniLibs"

REM Define destination for single file (can be absolute or relative)
set "DEST_FILE_PATH=%SCRIPT_DIR%\app\aizebra\build\windows\x64\runner"

REM -------------------------------------------------------------------
echo Copying folders...
xcopy "%SRC_FOLDER1%" "%DEST_BASE%\x86_64" /E /I /Y
xcopy "%SRC_FOLDER2%" "%DEST_BASE%\arm64-v8a" /E /I /Y

echo Copying file...
xcopy "%SRC_FILE%" "%DEST_FILE_PATH%\Debug" /Y
xcopy "%SRC_FILE%" "%DEST_FILE_PATH%\Release" /Y

echo.
echo === Copy complete ===