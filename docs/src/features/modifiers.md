# Modifiers

Modifiers are like built-in generic types that will modify the provided type
based on some rules. Users cannot define their own modifiers, only use the
built-in ones.

These are the currently available modifiers:

## Optional

The `Optional` modifier takes a `type` or an `input` as a parameter and outputs a
similar object with all its fields marked as "nullable". If the field in the original
object was declared required with a `!` the output will contain that field without
the `!`, otherwise the field is left untouched.

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
type _SomeType {
    foo: String!
    bar: Int!
    bool: [Float!]
}

type OptionalType = Optional<_SomeType>
```
</td>
            <td>

```graphql
type OptionalType {
    foo: String
    bar: Int
    bool: [Float!]
}


```
</td>
        </tr>
    </tbody>
</table>

## Required

The `Required` modifier takes a `type` or an `input` as a parameter and outputs a
similar object with all its fields marked as "non-nullable". If a field in the original
object did not have a `!`, it will have it in the new object, otherwise it will be 
left untouched

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
type _SomeType {
    foo: String
    bar: Int
    bool: [Float]!
}

type RequiredType = Required<_SomeType>
```
</td>
            <td>

```graphql
type RequiredType {
    foo: String!
    bar: Int!
    bool: [Float]!
}


```
</td>
        </tr>
    </tbody>
</table>
