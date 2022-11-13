# Inheritance

There will be times that a lot of `types` or `inputs` have some fields in common. In GraphQXL you
can inherit fields between `types` and `inputs` using spread operators.

## Spread operator
`types` and `inputs` can inherit fields from other `types` and `inputs` using
spread operators, for example:
<table style="width: 100%">
    <thead>
        <tr>
            <td align="center">Source GraphQXL</td>
            <td align="center">Compiled GraphQL</td>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td >

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
</td>
            <td>

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
</td>
        </tr>
    </tbody>
</table>

## Private fields

It is very common that you do not want to expose the `Common` type in the public API,
so you can make it private by prefixing the name with a `_` character (or the 
prefix that you provide in the CLI argument):
<table style="width: 100%">
    <thead>
        <tr>
            <td align="center">Source GraphQXL</td>
            <td align="center">Compiled GraphQL</td>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>

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
</td>
            <td>

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
</td>
        </tr>
    </tbody>
</table>

## Inheriting interfaces

A common pattern is to declare a GraphQL `interface` and to implement it in a `type`, but
you need to rewrite all the fields in the `type` that belong to the `interface` as they
are not implicit. You can use the spread operator also with `interfaces`:

<table style="width: 100%">
    <thead>
        <tr>
            <td align="center">Source GraphQXL</td>
            <td align="center">Compiled GraphQL</td>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>

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
</td>
            <td>

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
</td>
        </tr>
    </tbody>
</table>
