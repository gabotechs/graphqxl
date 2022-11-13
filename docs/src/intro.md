<div style="display: flex; flex-direction: row; align-items: flex-start">
    <img height="40" width="200" style="object-fit: cover; margin-top: 20px" src="./assets/graphqxl-name.svg" alt="">
    <span style="font-size: 32px; margin-top: 16px; font-weight: normal">book</span>
</div>


**GraphQXL** is an extension of the [GraphQL](https://graphql.org/) language
with some additional features that help creating big and scalable server-side
schemas.

When following a schema-first approach, there are
some challenges that are left unaddressed compared to a code-first approach.

- With a **code-first** approach, you have all the tools the programming language
you are using provides, like functions for automatically generating repetitive 
types, inheritance for reusing code pieces, and so on.

- With a **schema-first** approach you need to write by hand all the repetitive entities
in your schema, and if you want to split your schema across different files for a better 
maintainability you are bound to language-specific tools for merging them.

GraphQXL provides additional syntax to the original GraphQL language for solving this
challenges and making defining server-side GraphQL schemas a nicer experience,
without being bound to language-specific tools. At the end of the day, GraphQXL is just
a binary executable that will compile your GraphQXL schemas into GraphQL, independently
of the programming language that you are using for the backend.