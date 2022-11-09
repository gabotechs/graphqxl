# Generics

Generics can be used to reuse `types` or `inputs` that have some
small subset of the fields slightly different from each other:
```graphql
type Generic<T> {
    foo: T
}

type ConcreteString = Generic<String!>
type ConcreteInt = Generic<Int!>
```
will compile to
```graphql
type ConcreteString {
    foo: String!
}

type ConcreteInt {
    foo: Int!
}
```

It can even be combined with [inheritance](./inheritance.md):
```graphql
type Book {
    title: String!
}

type List<T> {
    first: T
    last: T
    content: [T!]!
}

type _ListOfBooks = List<Book>

type ListOfBooksWithLength {
    ..._ListOfBooks
    length: Int!
}
```
will compile to
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
