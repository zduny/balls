# balls

Watch others move their balls in real time.

## Prerequisites

To build the app [wasm-pack](https://rustwasm.github.io/wasm-pack) needs to be installed.

## Development

Use shell scripts to format code, lint, build, test, run or clean:

```bash
./format.sh
./clippy.sh
./build.sh
./test.sh
./run.sh
./build_and_run.sh
./clean.sh
```

Native client isn't included in `run.sh` (and `build_and_run.sh`) script,
to run native client (most likely in another terminal window/tab) type:

```bash
./app_client
```

### Windows 

Above scripts are available in Batch file form in `windows` directory.

**NOTE**: They have to be run from said `windows` directory, don't move them to project root directory before running them.

## See also

[rust-webapp-template](https://github.com/zduny/rust-webapp-template)
