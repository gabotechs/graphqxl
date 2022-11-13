# Generics

Generics can be used to reuse `types` or `inputs` that have some
small subset of the fields slightly different from each other:

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
