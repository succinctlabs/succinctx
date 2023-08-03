# `succinctup`

Update or revert to a specific Succinct branch with ease.

## Installing

```sh
curl -L https://cli.succinct.xyz | bash
```

## Usage

To install the **nightly** version:

```sh
succinctup
```

To install a specific **version** (in this case the `nightly` version):

```sh
succinctup --version nightly
```

To install a specific **branch** (in this case the `release/0.1.0` branch's latest commit):

```sh
succinctup --branch release/0.1.0
```

To install a **fork's main branch** (in this case `puma314/succinct`'s main branch):

```sh
succinctup --repo puma314/succinct
```

To install a **specific branch in a fork** (in this case the `patch-10` branch's latest commit in `puma314/succinct`):

```sh
succinctup --repo puma314/succinct --branch patch-10
```

To install from a **specific Pull Request**:

```sh
succinctup --pr 1071
```

To install from a **specific commit**:

```sh
succinctup -C 94bfdb2
```

To install a local directory or repository (e.g. one located at `~/git/succinct`, assuming you're in the home directory)

##### Note: --branch, --repo, and --version flags are ignored during local installations.

```sh
succinctup --path ./git/succinct
```

---

**Tip**: All flags have a single character shorthand equivalent! You can use `-v` instead of `--version`, etc.

---

## Acknowledgements

`succinctup` based on the [Foundry](https://github.com/foundry-rs/foundry) project's installation and usage. [Foundry](https://github.com/foundry-rs/foundry) is a Rust Ethereum
client focused on performance and reliability. Big shoutout to the folks behind `foundryup` for
providing and inspiring great CLI experiences.