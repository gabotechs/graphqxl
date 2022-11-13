# Description templates

Let's say that you have a `type` that you want to reuse in other types:

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
type _ToBeReused {
    "Field foo from type 'ToBeReused'"
    foo: String!
}

input Bar {
    ..._ToBeReused
}
```
</td>
            <td>

```graphql
input Bar {
    "Field foo from type 'ToBeReused'"
    foo: String!
}




```

</td>
        </tr>
    </tbody>
</table>

There, `foo`'s description says that the field `foo` belongs to the type `ToBeReused`,
but once that is compiled it is not true, it should say something like:
```graphql
input Bar {
    "Field foo from input 'Bar'"
    foo: String!
}
```

## Description string interpolation

You can use template variables in the description strings to refer to some 
contextual values:


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
type _ToBeReused {
    "Field foo from ${{ block.type }} '${{ block.name }}'"
    foo: String!
}

input Bar {
    ..._ToBeReused
}

type Baz {
    ..._ToBeReused
}
```
</td>
            <td>

```graphql
input Bar {
    "Field foo from input 'Bar'"
    foo: String!
}

type Baz {
    "Field foo from type 'Baz'"
    foo: String!
}



```

</td>
        </tr>
    </tbody>
</table>

These are the available values for the templates:
- **block.name**: The parent's block name
- **block.type**: The parent's block type (`type` or `input`)
- **variables.YOUR_GENERIC_VARIABLE**: The value of the generic variable once instanced