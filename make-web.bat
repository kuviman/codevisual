@echo off
setlocal enabledelayedexpansion

call emsdk\emsdk_env.bat
set PATH=C:\Users\vkudasov\.cargo\bin;%NODEJS_HOME%;C:\Users\vkudasov\AppData\Roaming\npm;%PATH%
set EMMAKEN_CFLAGS=-s TOTAL_MEMORY=268435456 -s FETCH=1 -s FETCH_DEBUG=1
cargo build --release --target=asmjs-unknown-emscripten --example playground
if !errorlevel! neq 0 exit /b !errorlevel!

set target_dir=target\web\asmjs\playground
mkdir %target_dir%

for /d %%a in (
    "target\asmjs-unknown-emscripten\release\build\codevisual_core_html-*"
) do copy /Y %%~fa\out\lib.html %target_dir%\codevisual.html
if !errorlevel! neq 0 exit /b !errorlevel!

for /d %%a in (
    "target\asmjs-unknown-emscripten\release\build\codevisual_core_css-*"
) do copy /Y %%~fa\out\lib.css %target_dir%\codevisual.css
if !errorlevel! neq 0 exit /b !errorlevel!

for /d %%a in (
    "target\asmjs-unknown-emscripten\release\build\codevisual_core_js-*"
) do copy /Y %%~fa\out\lib.js %target_dir%\codevisual.js
if !errorlevel! neq 0 exit /b !errorlevel!

xcopy /Y /q src\core\web\favicon.ico %target_dir%
if !errorlevel! neq 0 exit /b !errorlevel!
mkdir %target_dir%\lib
xcopy /E /Y /q src\core\web\lib %target_dir%\lib
if !errorlevel! neq 0 exit /b !errorlevel!
xcopy /E /Y /q examples\playground\static\* %target_dir%
if !errorlevel! neq 0 exit /b !errorlevel!
copy /Y target\asmjs-unknown-emscripten\release\examples\playground.js %target_dir%\code.js
if !errorlevel! neq 0 exit /b !errorlevel!