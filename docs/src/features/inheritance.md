# Inheritance

There will be times that a lot of `types` or `inputs` have some fields in common. In GraphQXL you
can inherit fields between `types` and `inputs` using spread operators.

## Spread operator
`types` and `inputs` can inherit fields from other `types` and `inputs` using
spread operators, for example:

[Open in sandbox](https://graphqxl-explorer.vercel.app/?code=dHlwZSBDb21tb24gewogICAgIlR5cGUncyBJRCIKICAgIGlkOiBJRCEKICAgICJUeXBlJ3MgTmFtZSIKICAgIG5hbWU6IFN0cmluZyEKICAgICJUeXBlJ3MgZGVzY3JpcHRpb24iCiAgICBkZXNjcmlwdGlvbjogU3RyaW5nIQp9Cgp0eXBlIFByb2R1Y3QgewogICAgLi4uQ29tbW9uCiAgICAiUHJvZHVjdCdzIHByaWNlIgogICAgcHJpY2U6IEZsb2F0IQp9)
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

[Open in sandbox](https://graphqxl-explorer.vercel.app/?code=dHlwZSBfQ29tbW9uIHsKICAgICJUeXBlJ3MgSUQiCiAgICBpZDogSUQhCiAgICAiVHlwZSdzIE5hbWUiCiAgICBuYW1lOiBTdHJpbmchCiAgICAiVHlwZSdzIGRlc2NyaXB0aW9uIgogICAgZGVzY3JpcHRpb246IFN0cmluZyEKfQoKdHlwZSBQcm9kdWN0IHsKICAgIC4uLl9Db21tb24KICAgICJQcm9kdWN0J3MgcHJpY2UiCiAgICBwcmljZTogRmxvYXQhCn0=)
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

[Open in sandbox](https://graphqxl-explorer.vercel.app/?code=aW50ZXJmYWNlIFBlcnNvbiB7CiAgICBwYXJlbnQ6IFN0cmluZyEKICAgIGNoaWxkczogW1N0cmluZyFdIQp9Cgp0eXBlIERhZCBpbXBsZW1lbnRzIFBlcnNvbiB7CiAgICAuLi5QZXJzb24KICAgIGpvYl90aXRsZTogU3RyaW5nIQp9Cgp0eXBlIEtpZCBpbXBsZW1lbnRzIFBlcnNvbiB7CiAgICAuLi5QZXJzb24KICAgIHNjaG9vbF9uYW1lOiBTdHJpbmchCn0=)
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
