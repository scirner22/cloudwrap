# Cloudwrap

<a href="https://travis-ci.org/Blackfynn/cloudwrap" title="Travis Build Status"><img src="https://travis-ci.org/Blackfynn/cloudwrap.svg?branch=master" alt="travis-badge"></img></a>

NOTE: The AWS Secrets Manager interface is currently disabled.

This library is an opinionated utility for fetching configuration and secrets from
AWS SSM Parameter Store and AWS Secrets Manager. Its primary use is to act as a wrapper
around a command to execute. The executed command is injected with the configuration as
environment variables.

Key/value pairs are fetched by using resource paths. The path must be specified in the form of
`/{environment}/{service_name}/key`. This utility always expects a path of three components,
namely a key that is nested under an environment and service name. Multiple `service_name`s can
be provided and the configurations are merged together. See `cloudwrap --help`.

Values are converted from kebab case to upper case with underscores.

This utility is only associated with fetching of the underlying AWS services. Another mechanism
of your choice may be used to set the configuration in SSM Parameter Store and AWS Secrets
Manager.

## Download

NOTE: Due to a dependency that isn't yet released (rusoto), this project cannot be pulled from cargo.

## Usage

Describe keys for a service:

```
$ cloudwrap staging service-name-test describe
   KEY   | VERSION |  LAST_MODIFIED_USER   | LAST_MODIFIED_DATE
---------+---------+-----------------------+---------------------
 one-key |       1 | vienna@cloudwrap.com  | 2018-04-24 19:36:02
 two     |       1 | lachy@cloudwrap.com   | 2018-04-24 19:36:16
```

Print key/value pairs for a service:

```
$ cloudwrap staging service-name-test stdout
   KEY   | VALUE
---------+----------
 one-key | valueone
 two     | valuetwo

```

Execute a command with the configuration injected as environment variables:

```
$ cloudwrap staging service-name-test exec env
ONE_KEY=valueone
TWO=valuetwo
...
```

See `cloudwrap --help` for a full listing of available commands.

# Permissions

### AWS IAM

The following is a minimal example policy needed in order to use cloudwrap to wrap
programs in AWS that make use of IAM permissions. The `kms:Decrypt` permission is only needed
if your configuration parameters contain secure strings. Likewise, the kms key/alias used will have
to be changed if you didn't use the ssm default.

#### Command

```
cloudwrap dev auth-service exec java -jar {jar-name}.jar
```

#### Resource

```
resource "aws_iam_role_policy" "parameters" {
  name = "dev-auth-service-parameter-policy"
  role = "${var.role_id}"

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "ssm:GetParameter",
        "ssm:GetParameters",
        "ssm:GetParametersByPath"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:ssm:${var.aws_region}:${var.aws_account_id}:parameter/dev/auth-service/*"
      ]
    },
    {
      "Action": [
        "kms:Decrypt"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:kms:${var.aws_region}:${var.aws_account_id}:key/alias/aws/ssm"
    }
  ]
}
EOF
}
```

# License

This project is licensed under Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
