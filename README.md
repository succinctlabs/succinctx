# sdk

## Release

To create a new release:

```sh
./build/release.sh <X.Y.Z>
```

## Building ABIs and Bindings

Then to build tha ABIs:

```sh
./build/abi.sh
```

To build the bindings:

```sh
./build/binding.sh
```

If you need to add a binding for a different contract, edit `build/binding.sh` and modify the `CONTRACTS` array.