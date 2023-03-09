# eli
CLI wrapper application for Elhuyar Hiztegia

**WARNING! This application may break at any time since it uses a web scraper.**

As mentioned above, this application uses a web scraper to fetch terms from the
[Elhuyar translator](https://hiztegiak.elhuyar.eus/). It provides a flexible CLI interface.

## How to build
To build the application (`rustup` is required):
```bash
$ cargo build --release
```

If the build ends up in a compiler version error, a *nightly* toolchain should be set for the project:
```bash
$ rustup override set nightly
```
And then try again the above command.

The following command can be used in order to install it under the default `cargo` directory:
```bash
$ cargo install --path .
```

## Basic usage
The application provides an easy-to-use CLI interface:
```bash
$ eli --help
...
```

We can search multiple terms at once:
```bash
$ eli sentitu pentsatu ekin
...
```

Or we can specify the language in which we want to do the search:
```bash
$ eli --from es hola buenos dias
...
```

We can also search for word sets
```bash
$ eli -f es "buenos dias"
```
