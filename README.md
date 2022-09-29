# GraphQXL

[![Coverage Status](https://coveralls.io/repos/github/gabotechs/graphqxl/badge.svg?branch=main)](https://coveralls.io/github/gabotechs/graphqxl?branch=main)

> **WIP**: This project is a work in progress

GraphQXL is an extension of the GraphQL language with some additional features
that are useful for creating scalable server side schemas.

### Object inheritance

#### Input:

```graphql
type OtherType {
    "Descriptions are also inherited"
    bar: Int!
}

type MyType {
    foo: String!
    ...OtherType
}
```

#### Output:

```graphql
type MyType {
    foo: String!
    "Descriptions are also inherited"
    bar: Int!
}
```

### Import statements

#### Input:

`my_file.graphqxl`

```graphql
import "other_file"

type MyType {
    foo: OtherType!
}
```

`other_file.graphqxl`

```graphql
type OtherType {
    bar: Int!
}
```

#### Output:

```graphql
type OtherType {
    bar: Int!
}

type MyType {
    foo: OtherType!
}
```

### Generics

#### Input:

```graphql
type MyGenericType<T> {
    foo: T
}

type MyStringType = MyGenericType<String!>

type MyIntType = MyGenericType<Int!>
```

#### Output:

```graphql
type MyStringType {
    foo: String!
}

type MyIntType {
    foo: Int!
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
