# Inheritance

## Spread operator
`types` and `inputs` can inherit fields from other `types` and `inputs` using
spread operators, for example:
```graphql
type Common {
    "Type's ID"
    id: ID!
    "Type's Name"
    name: String!
    "Type's description"
    description: String!
}

type Product {
    ...Common
    "Product's price"
    price: Float!
}
```
will compile to 
```graphql
type Common {
    "Type's ID"
    id: ID!
    "Type's Name"
    name: String!
    "Type's description"
    description: String!
}

type Product {
    "Type's ID"
    id: ID!
    "Type's Name"
    name: String!
    "Type's description"
    description: String!
    "Product's price"
    price: Float!
}
```

## Private fields

It is very common that you do not want to expose the `Common` type in the public API,
so you can make it private by prefixing the type with a `_` character (or the 
prefix override that you provide in the CLI argument):
```graphql
type _Common {
    "Type's ID"
    id: ID!
    "Type's Name"
    name: String!
    "Type's description"
    description: String!
}

type Product {
    ..._Common
    "Product's price"
    price: Float!
}
```
will compile to
```graphql
type Product {
    "Type's ID"
    id: ID!
    "Type's Name"
    name: String!
    "Type's description"
    description: String!
    "Product's price"
    price: Float!
}
```

## Inheriting interfaces

A common pattern is to declare a GraphQL `interface` and to implement it in a `type`, but
you need to rewrite all the fields in the `type` that belong to the `interface` as they
are not implicit. You can use the spread operator also with `interfaces`:

```graphql
interface Person {
    parent: String!
    childs: [String!]!
}

type Dad implements Person {
    ...Person
    job_title: String!
}

type Kid implements Person {
    ...Person
    school_name: String!
}
```
will compile to
```graphql
interface Person {
    parent: String!
    childs: [String!]!
}

type Dad implements Person {
    parent: String!
    childs: [String!]!
    job_title: String!
}

type Kid implements Person {
    parent: String!
    childs: [String!]!
    school_name: String!
}
```
