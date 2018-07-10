# Cloudwrap

<a href="https://travis-ci.org/scirner22/cloudwrap" title="Travis Build Status"><img src="https://travis-ci.org/scirner22/cloudwrap.svg?branch=master" alt="travis-badge"></img></a>

NOTE: The AWS Secrets Manager interface is currently disabled.

This library is an opinionated utility for fetching configuration and secrets from
AWS SSM Parameter Store and AWS Secrets Manager. Its primary use is to act as a wrapper
around a command to execute. The executed command is injected with the configuration as
environment variables.

Key/value pairs are fetched by using resource paths. The path must be specified in the form of
`/{environment}/{service_name}/key`. This utility always expects a path of three components,
namely a key that is nested under an environment and service name. The resource path
`/{environment}/common/*` is also fetched and merged into the service resource path. Any keys
from the `common` resource path can be overwritten by the service resource path.

This utility is only associated with fetching of the underlying AWS services. Another mechanism
of your choice may be used to set the configuration in SSM Parameter Store and AWS Secrets
Manager.

## Download

NOTE: Due to a dependency that isn't yet released (rusoto), this project cannot be pulled from cargo.

## Usage

Describe keys for a service:

```
$ cloudwrap staging service-name-test describe
 KEY | VERSION |  LAST_MODIFIED_USER   | LAST_MODIFIED_DATE
-----+---------+-----------------------+---------------------
 one |       1 | vienna@cloudwrap.com  | 2018-04-24 19:36:02
 two |       1 | lachy@cloudwrap.com   | 2018-04-24 19:36:16
```

Print key/value pairs for a service:

```
$ cloudwrap staging service-name-test stdout
 KEY | VALUE
-----+----------
 one | valueone
 two | valuetwo

```

Execute a command with the configuration injected as environment variables:

```
$ cloudwrap staging service-name-test exec env
ONE=valueone
TWO=valuetwo
...
```

See `cloudwrap --help` for a full listing of available commands.

# License

This project is licensed under Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
