@echo off

cd ..

echo Linting common...
cd common || exit /b
cargo clippy || exit /b
cd ..

echo:
echo Linting server...
cd server || exit /b
cargo clippy || exit /b
cd ..

echo:
echo Linting client...
cd client || exit /b
cargo clippy --target=wasm32-unknown-unknown || exit /b
cd ..

cd windows || exit /b
