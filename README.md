<h1 align="center">
    Splitwise Exporter
</h1>

<p align="center">
    <i align="center">Exports Splitwise data to a csv</i>
</p>

<h4 align="center">
    <a href="https://github.com/mparusinski/splitwise_exporter/actions/workflows/build_nix.yml">
        <img src="https://img.shields.io/github/actions/workflow/status/mparusinski/splitwise_exporter/build_nix.yml?branch=main&label=pipeline&style=flat-square" alt="continuous integration" style="height: 20px;">
    </a>
    <a href="https://github.com/mparusinski/splitwise_exporter/graphs/contributors">
        <img src="https://img.shields.io/github/contributors-anon/mparusinski/splitwise_exporter?color=yellow&style=flat-square" alt="contributors" style="height: 20px;">
    </a>
</h4>

## Introduction

Splitwise Exporter is a simple project to export data from Splitwise from a (hardcoded for now) group to a csv file written in [Rust](https://doc.rust-lang.org/stable/reference/).

## Usage

The usage of the project is simple

```bash
export SPLITWISE_API_KEY=<your_api_key>
./splitwise_exporter output.csv
```

Here is information about requesting an API key https://dev.splitwise.com/

## Developing

This is a standard [Rust cargo binary project](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html).


```bash
cargo build
```

Using [nix flakes](https://nixos.wiki/wiki/Flakes)

```bash
nix develop
cargo build
```

## Contributing

This is a personal project, it is not open source for now and I am not accepting contributions.

## License

This project is not open source.
