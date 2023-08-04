# sdk

## Installing

Open your terminal and run the following command:

```sh
curl -L https://cli.succinct.xyz | bash
```

This installs `succinctup`. Running this will install or update the latest version of the binaries:

```sh
succinctup
```

Now you can use the `succinct` CLI. For example, to initalize a project:

```sh
succinct init
```

## Release

To create a new release:

```sh
./build/release.sh <X.Y.Z>
```

## Building ABIs and Bindings

To build the ABIs:

```sh
./build/abi.sh
```

Then to build the bindings:

```sh
./build/binding.sh
```

If you need to add a binding for a different contract, edit `build/binding.sh` and modify the `CONTRACTS` array.