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
cargo build %cargoArgs% --target=%cargoTarget% --example playground
copy target\%cargoTarget%\%config%\examples\playground.js examples\public\code.js