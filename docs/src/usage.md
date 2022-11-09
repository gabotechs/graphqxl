# Usage

GraphQXL is just a compiler that will receive `.graphqxl` files as an input and will
compile them down to a common `.graphql` file.

For example, given this `foo.graphqxl`:
```graphql
# foo.graphqxl
type MyType {
    foo: String
}
```
You can do
```sh
graphqxl foo.graphqxl
```
And the file will get compiled to `foo.graphql`:
```graphql
# foo.graphql
type MyType {
    foo: String
}
```

## Features

So, the example above was not very useful, as the compilation result is exactly the same
as the input. Here is a list of more useful things you can do with GraphQXL:

- [Field inheritance](./features/inheritance.md)
- [Generic types and inputs](./features/generics.md)
- [Import statements](./features/imports.md)
- [String templates](./features/templates.md)
