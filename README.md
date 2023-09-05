# succinctx

The schelling point for ZK applications.

## Installing

Open your terminal and run the following command:

```sh
curl -L https://cli.succinct.xyz | bash
```

This installs `succinctup`. Running this will install or update the latest version of the binaries:

```sh
succinctup
```

## Creating a project

After installing, you can use the `succinct` CLI. For example, to initialize a circuit:

```sh
succinct init
```

Build the circuit with:

```sh
succinct build
```

Generate a proof for the circuit with:

```sh
succinct prove
```

## Release

To create a new SDK release:

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