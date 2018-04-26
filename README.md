# turbine

This library is an opinionated utility for fetching configuration and secrets from
AWS SSM Parameter Store and AWS Secrets Manager. Its primary use is to act as a wrapper
around a command to execute. The executed command is injected with the configuration as
environment variables.

// TODO pathing

## Download

Downloading from `cargo`:

```
cargo install turbine-rs
```

## Usage

Describe keys for a service:

```
$ turbine staging service-name-test describe
 KEY | VERSION | LAST_MODIFIED_USER  | LAST_MODIFIED_DATE
-----+---------+---------------------+---------------------
 one |       1 | vienna@turbine.com  | 2018-04-24 19:36:02
 two |       1 | lochy@turbine.com   | 2018-04-24 19:36:16
```

Print key/value pairs for a service:

```
$ turbine staging service-name-test stdout
 KEY | VALUE
-----+----------
 one | valueone
 two | valuetwo

```

Execute a command with the configuration injected as environment variables:

```
$ turbine staging service-name-test exec env
ONE=valueone
TWO=valuetwo
```

See `turbine --help` for a full listing of available commands.

# License

This project is licensed under Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
