# Schemius

[![CI](https://github.com/cowuake/schemius/actions/workflows/continuous-integration.yml/badge.svg)](https://github.com/cowuake/schemius/actions/workflows/continuous-integration.yaml)
[![Web](https://github.com/cowuake/schemius/actions/workflows/publish-web.yml/badge.svg)](https://github.com/cowuake/schemius/actions/workflows/publish-web.yml)
[![Coverage](https://coveralls.io/repos/github/cowuake/schemius/badge.svg)](https://coveralls.io/github/cowuake/schemius)

## What is Schemius?

Schemius is an interpreter for the Scheme programming language written in Rust, aiming at being fully compliant with the [R7RS-small](https://small.r7rs.org/) standard and easily accessible for everyone. Currently in an early development stage.

## Goals

- **Real R7RS-small compliance**
  - **The usual state of things**: several Scheme interpreters are declared to be R7RS-compliant, but it's easy to find some example in the official specification which will give a different outcome when executed on some of these interpreters
  - **The objective**: test new features against the R7RS-small specifications whenever feasible in the early steps of their implementations; implement all the features strictly required from the the standard and as many as possible of those not strictly required
  - **How the issue is being addressed**: the examples reported on the R7RS-small document are used to write integration tests for the interpreter, so that departures from the standard are captured early (won't pass continuous integration checks)

- **Accessibility**
  - **The usual state of things**: most Scheme interpreters only expose their REPLs as CLI applications to be run in a terminal on a local machine, thus discouraging the casual user's interest for Scheme, especially on mobile devices
  - **The objective**: keep the interpreter available online as an easily accessible web page providing all the essential features of the terminal application, and some additional comforts
  - **How the issue is being addressed**: Schemius' core is accessed through a minimalistic API, which compiles to WebAssembly and is made available to the [Schemius Web Client](https://cowuake.github.io/schemius/), easily reachable for everyone

## Try out Schemius

### From the command line (native)

You will have to install the Rust toolchain on your machine, what you can achieve following [the instructions here](https://www.rust-lang.org/tools/install).

Once Rust is correctly set up on the machine, you can either choose to directly run Schemius or to install it on your machine. Any previous installation should be simply overwritten in the process.

#### Run without installing

From the repository's root, spawn a terminal and run:

```bash
cargo run --release
```

if you want to run the REPL (Read-Eval-Print Loop interface), or

```bash
cargo run --release -- --help
```

if you want to explore all the available options.

#### Install and run

You can open a terminal and run:

```bash
cargo install --path ./schemius-native
```

which will install the Schemius terminal application and make it available from the command line as `schemius`. You can run the command without arguments for spawning the REPL, or see all the available options with `schemius --help`.

### From the command line (Docker)

If you do not want to install Rust but already have Docker installed on your machine, you can build an image spawning a terminal, running

```bash
docker build -t schemius .
```

and then execute the repl in a container with

```bash
docker run -it schemius
```

or explore all the available options with

```bash
docker run -it schemius --help
```

### From the web

The Schemius Web Client is hosted on [GitHub Pages](https://pages.github.com/) and can be found at [this address](https://cowuake.github.io/schemius/). Support for mobile devices and touchscreen input is being improved along the way.

> From the web client on mobile devices, you could find difficult to rely on keybinds involving modifier keys such as `Ctrl` and `Shift`. For now, you can take advantage of the following special commands (only available online):
>
> - `(switch-font)`: Cycle between available fonts
> - `(switch-theme)`: Cycle between available color thems
> - `(clc)` or `(clear-screen)`: Clear the screen
>

## Start out with the Scheme programming language

An essential primer on the Scheme language will be included in the future. For now, you can refer to the [Scheme documentation](https://docs.scheme.org/). If you find the amount of material intimiting or you struggle finding the entry point of an effective and straightforward introduction, reach for [this Scheme Primer] instead.

You will soon discover you cannot run all the Scheme code out there on Schemius because of its early development stage. Do not hesitate contacting me or opening an issue if you think some feature should be implemented soon rather the later, or if you evaluate some expression and obtain an unexpected outcome.

## License

Licensed under either of

- Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
