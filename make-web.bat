@echo off

set config=release
if not "%1" == "" (
    set config=%1
)

set cargoTarget=asmjs-unknown-emscripten
set cargoArgs=
if "%config%" == "release" (
    set cargoArgs=--release
)

call emsdk_env
set LIBCLANG_PATH=C:/Programs/Emscripten/clang/e1.37.9_64bit

cargo build %cargoArgs% --target=%cargoTarget% --example playground
copy target\%cargoTarget%\%config%\examples\playground.js playground\public\code.js