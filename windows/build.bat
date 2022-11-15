@echo off

cd ..

echo Building common...
cd common || exit /b
cargo build --release || exit /b
cd ..

echo:
echo Building server...
cd server || exit /b
cargo build --release || exit /b
cd ..

echo:
echo Building client...
cd client || exit /b
wasm-pack build --release --target web || exit /b
cd ..

copy "client\pkg\client.js" "www\client.js" || exit /b
copy "client\pkg\client_bg.wasm" "www\client_bg.wasm" || exit /b

copy "server\target\release\server.exe" ".\app_server.exe" || exit /b

cd windows || exit /b
