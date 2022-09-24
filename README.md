# GraphQXL

[![Coverage Status](https://coveralls.io/repos/github/gabotechs/graphqxl/badge.svg?branch=main)](https://coveralls.io/github/gabotechs/graphqxl?branch=main)

> **WIP**: This project is a work in progress

GraphQXL is an extension of the GraphQL language with some additional features
that are useful for creating scalable server side schemas.

- Object inheritance.
```graphql
type MyType {
    field: String!
    ...OtherType
}
```
- Import statements.
```graphql
import "other_file"

type MyType {
    field: OtherFilesType!
}
```
- Generics (TODO).
```graphql
type MyGenericType<T> {
    field: T
}

type MyType {
    stringField: MyGenericType<String!>
}

```

## Install

Mac M1
```shell
wget https://github.com/gabotechs/graphqxl/releases/latest/download/graphqxl-aarch64-apple-darwin.tar.gz
tar -xvf graphqxl-aarch64-apple-darwin.tar.gz
```
Mac Intel
```shell
wget https://github.com/gabotechs/graphqxl/releases/latest/download/graphqxl-x86_64-apple-darwin.tar.gz
tar -xvf graphqxl-x86_64-apple-darwin.tar.gz
```
Linux x86_64
```shell
wget https://github.com/gabotechs/graphqxl/releases/latest/download/graphqxl-x86_64-unknown-linux-gnu.tar.gz
tar -xvf graphqxl-x86_64-unknown-linux-gnu.tar.gz
```
Linux aarch64
```shell
wget https://github.com/gabotechs/graphqxl/releases/latest/download/graphqxl-aarch64-unknown-linux-gnu.tar.gz
tar -xvf graphqxl-aarch64-unknown-linux-gnu.tar.gz
```

## Usage

```shell
./graphqxl foo.graphqxl
```

this will output `foo.graphql` as a result

## License

> TODO