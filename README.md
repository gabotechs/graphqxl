# GraphQXL

[![Coverage Status](https://coveralls.io/repos/github/gabotechs/graphqxl/badge.svg?branch=master)](https://coveralls.io/github/gabotechs/graphqxl?branch=master)

> **WIP**: This project is a work in progress and currently does not offer any of the
> described features

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
import other_file

type MyType {
    field: OtherFilesType!
}
```
- Generics.
```graphql
type MyGenericType<T> {
    field: T
}

type MyType {
    stringField: MyGenericType<String!>
}

```

## Install

> TODO

## Usage

> TODO

## License

> TOOD