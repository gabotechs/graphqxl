# Description templates

Let's say that you have this `type` that you want to reuse in other types:
```graphql
type _ToBeReused {
    "Field foo from ToBeReused"
    foo: String!
}

type Bar {
    ..._ToBeReused
}
```
The result of compiling the above would look like this:
```graphql
type Bar {
    "Field foo from ToBeReused"
    foo: String!
}
```
There, `foo`'s description says that the field `foo` belongs to the type `ToBeReused`,
but once that is compiled it is not true, it should say something like:
```graphql
type Bar {
    "Field foo from Bar"
    foo: String!
}
```
How to solve this?

You can use template variables in the description strings to refer to some 
contextual values:
```graphql
type _ToBeReused {
    "Field foo from ${{ block.name }}"
    foo: String!
}

type Bar {
    ..._ToBeReused
}

type Baz {
    ..._ToBeReused
}
```
Will compile to
```graphql
type Bar {
    "Field foo from Bar"
    foo: String!
}

type Baz {
    "Field foo from Baz"
    foo: String!
}
```

These are the available values for the templates:
- **block.name**: The parent's block name
- **block.type**: The parent's block type (`type` or `input`)
- **variables.YOUR_GENERIC_VARIABLE**: The value of the generic variable once instanced