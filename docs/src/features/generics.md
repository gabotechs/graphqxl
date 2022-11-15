# Generics

Generics can be used to reuse `types` or `inputs` that have some
small subset of the fields slightly different from each other:

[Open in sandbox](https://graphqxl-explorer.vercel.app/?code=dHlwZSBHZW5lcmljPFQ%2bIHsKICAgIGZvbzogVAp9Cgp0eXBlIEZvb1N0cmluZyA9IEdlbmVyaWM8U3RyaW5nIT4KCnR5cGUgRm9vSW50ID0gR2VuZXJpYzxJbnQhPg==)
<table style="width: 100%">
    <thead>
        <tr>
            <td align="center">Source GraphQXL</td>
            <td align="center">Compiled GraphQL</td>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td style="width: 50%">

```graphql
type Generic<T> {
    foo: T
}

type FooString = Generic<String!>

type FooInt = Generic<Int!>
```
</td>
            <td>

```graphql
type FooString {
    foo: String!
}

type FooInt {
    foo: Int!
}
```
</td>
        </tr>
    </tbody>
</table>

Notice how the generic type definition is omitted from the generated GraphQL. If
a `type` or an `input` is declared with generic type parameters, it will not be
present in the generated GraphQL.

It can even be combined with [inheritance](./inheritance.md):

[Open in sandbox](https://graphqxl-explorer.vercel.app/?code=dHlwZSBCb29rIHsKICAgIHRpdGxlOiBTdHJpbmchCn0KCnR5cGUgTGlzdDxUPiB7CiAgICBmaXJzdDogVAogICAgbGFzdDogVAogICAgY29udGVudDogW1QhXSEKfQoKdHlwZSBMaXN0T2ZCb29rc1dpdGhMZW5ndGggewogICAgLi4uTGlzdDxCb29rPgogICAgbGVuZ3RoOiBJbnQhCn0)
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
type Book {
    title: String!
}

type List<T> {
    first: T
    last: T
    content: [T!]!
}

type ListOfBooksWithLength {
    ...List<Book>
    length: Int!
}
```
</td>
            <td>

```graphql
type Book {
    title: String!
}

type ListOfBooksWithLength {
    first: Book
    last: Book
    content: [Book!]!
    length: Int!
}




```

</td>
        </tr>
    </tbody>
</table>
