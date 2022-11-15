@echo off

cd ..

echo Testing common...
cd common || exit /b
cargo test || exit /b
cd ..

echo:
echo Testing server...
cd server || exit /b
cargo test || exit /b
cd ..

echo:
echo Testing client...
cd client || exit /b
wasm-pack test --headless --chrome || exit /b
cd ..

cd windows || exit /b
