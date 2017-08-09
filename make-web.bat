set EMMAKEN_CFLAGS="-s TOTAL_MEMORY=268435456"
cargo build --release --target=asmjs-unknown-emscripten --example playground

set target_dir=target\web\asmjs\playground
mkdir %target_dir%

for /d %%a in (
    "target\asmjs-unknown-emscripten\release\build\codevisual_core_html-*"
) do copy /Y %%~fa\out\lib.html %target_dir%\codevisual.html

for /d %%a in (
    "target\asmjs-unknown-emscripten\release\build\codevisual_core_css-*"
) do copy /Y %%~fa\out\lib.css %target_dir%\codevisual.css

for /d %%a in (
    "target\asmjs-unknown-emscripten\release\build\codevisual_core_js-*"
) do copy /Y %%~fa\out\lib.js %target_dir%\codevisual.js

xcopy /Y src\core\web\favicon.ico %target_dir%
mkdir %target_dir%\lib
xcopy /E /Y src\core\web\lib %target_dir%\lib
xcopy /E /Y examples\playground\static\* %target_dir%
xcopy /Y target\asmjs-unknown-emscripten\release\examples\playground.js %target_dir%\code.js