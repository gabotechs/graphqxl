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

[Open in sandbox](https://graphqxl-explorer.vercel.app/?code=dHlwZSBfU29tZVR5cGUgewogICAgZm9vOiBTdHJpbmchCiAgICBiYXI6IEludCEKICAgIGJvb2w6IFtGbG9hdCFdCn0KCnR5cGUgT3B0aW9uYWxUeXBlID0gT3B0aW9uYWw8X1NvbWVUeXBlPg==)
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

[Open in sandbox](https://graphqxl-explorer.vercel.app/?code=dHlwZSBfU29tZVR5cGUgewogICAgZm9vOiBTdHJpbmcKICAgIGJhcjogSW50CiAgICBib29sOiBbRmxvYXRdIQp9Cgp0eXBlIFJlcXVpcmVkVHlwZSA9IFJlcXVpcmVkPF9Tb21lVHlwZT4=)
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
