# AST Generator Tool Design

## Overview

Convert the `rlox` repo into a Cargo workspace and add an `ast_generator` binary crate that generates `expr.rs` for the interpreter — mirroring the `GenerateAst` tool from Crafting Interpreters Chapter 5, adapted for idiomatic Rust.

## Workspace Layout

```
rlox/
├── Cargo.toml              # workspace root: members = ["rlox", "ast_generator"]
├── rlox/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── scanner.rs
│       ├── token.rs
│       ├── token_type.rs
│       └── expr.rs         ← generated output
└── ast_generator/
    ├── Cargo.toml
    └── src/
        └── main.rs         ← generator binary
```

The existing `rlox/` source files move into the `rlox/` subcrate unchanged.

## Tool Design (`ast_generator`)

### Input

Hardcoded in `main.rs` as a list of `(&str, Vec<(&str, &str)>)` — (type name, fields):

```rust
let types = vec![
    ("Binary",   vec![("left", "Expr"), ("operator", "Token"), ("right", "Expr")]),
    ("Grouping", vec![("expression", "Expr")]),
    ("Literal",  vec![("value", "Option<crate::token::Literal>")]),
    ("Unary",    vec![("operator", "Token"), ("right", "Expr")]),
];
```

### Invocation

```
cargo run -p ast_generator -- rlox/src
```

Takes the output directory as a single CLI argument, writes `<dir>/expr.rs`.

### Output: `expr.rs`

The generated file contains four sections:

1. **Structs** — one pub struct per variant, with pub fields matching the definition list. `Token` fields use `crate::token::Token`. Recursive `Expr` fields are `Box<Expr>`.

2. **`Expr` enum** — one variant per type wrapping `Box<TypeName>`:
   ```rust
   pub enum Expr {
       Binary(Box<Binary>),
       Grouping(Box<Grouping>),
       Literal(Box<Literal>),
       Unary(Box<Unary>),
   }
   ```

3. **`Visitor<T>` trait** — one method per variant:
   ```rust
   pub trait Visitor<T> {
       fn visit_binary(&self, expr: &Binary) -> T;
       fn visit_grouping(&self, expr: &Grouping) -> T;
       fn visit_literal(&self, expr: &Literal) -> T;
       fn visit_unary(&self, expr: &Unary) -> T;
   }
   ```

4. **`accept` impl on `Expr`**:
   ```rust
   impl Expr {
       pub fn accept<T>(&self, visitor: &impl Visitor<T>) -> T {
           match self {
               Expr::Binary(e)   => visitor.visit_binary(e),
               Expr::Grouping(e) => visitor.visit_grouping(e),
               Expr::Literal(e)  => visitor.visit_literal(e),
               Expr::Unary(e)    => visitor.visit_unary(e),
           }
       }
   }
   ```

### Generation Strategy

Plain string formatting — the tool iterates the type list and uses `write!` / `writeln!` into a `String`, then writes the file via `std::fs::write`. No external dependencies.

## Error Handling

- Missing or invalid CLI argument: print usage and exit with code 1.
- File write failure: propagate with `expect`.

## Testing

- Run `cargo run -p ast_generator -- rlox/src` and verify `expr.rs` is written.
- Run `cargo build -p rlox` to confirm the generated file compiles cleanly.
- No unit tests in the generator itself — correctness is validated by the interpreter compiling.
