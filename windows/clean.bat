@echo off

cd ..

cd common || exit /b
cargo clean || exit /b
cd ..

cd server || exit /b
cargo clean || exit /b
cd ..

cd client || exit /b
cargo clean || exit /b
del /S /Q pkg || exit /b
cd ..

cd www || exit /b
del client.js || exit /b
del client_bg.wasm || exit /b
cd ..

del app_server.exe || exit /b

cd windows || exit /b
