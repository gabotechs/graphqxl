# Imports

Import statements allow you to split your schema across different files without
relying on any opinionated merging tool or some language-specific merging script.
This is useful for keeping your schema's code base clean and for improving its
maintainability.

An `import` statement will resolve all the content of the imported
file in the current one, for example:


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
# common-stuff.graphqxl

type Identifier {
    id: ID!
}
```
```graphql
# product.graphqxl

import "common-stuff"

type Product {
    id: Identifier
    price: Float!
}
```
</td>
            <td>

```graphql
# product.graphql

type Identifier {
    id: ID!
}

type Product {
    id: Identifier
    price: Float!
}





```

</td>
        </tr>
    </tbody>
</table>


A good pattern for keeping your schema clean could look something like this:
```graphql
import "products"
import "users"
# import other entities

type Query {
    products: [Product!]!
    product(id: ID!): Product
    users: [User!]!
    user(id: ID!): User
    # queries for the other entities
}

type Mutation {
    createProduct(input: CreateProductInput!): Product!
    deleteProduct(id: ID!): Product!
    createUser(input: CreateUserInput!): User!
    deleteUser(id: ID!): User!
    # mutations for the other entities
}

schema {
    query: Query
    mutation: Mutation
}
```