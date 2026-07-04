# AST Generator Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Convert the `rlox` repo into a Cargo workspace and add an `ast_generator` binary crate that generates `rlox/src/expr.rs` with the visitor-pattern AST types.

**Architecture:** The root `Cargo.toml` becomes a workspace manifest with two members: `rlox` (the existing interpreter, moved to a subcrate) and `ast_generator` (a new binary that writes `expr.rs` via plain string formatting). Running the generator then re-building the `rlox` crate validates correctness.

**Tech Stack:** Rust, Cargo workspaces, `std::fs::write`, `std::fmt::Write`

---

## File Map

| Action | Path | Responsibility |
|--------|------|----------------|
| Modify | `Cargo.toml` | Workspace root — replaces `[package]` with `[workspace]` |
| Create | `rlox/Cargo.toml` | Package manifest for the interpreter subcrate |
| Create | `rlox/src/main.rs` | Copy of `src/main.rs` + `mod expr;` declaration |
| Copy   | `rlox/src/scanner.rs` | Unchanged from `src/scanner.rs` |
| Copy   | `rlox/src/token.rs` | Unchanged from `src/token.rs` |
| Copy   | `rlox/src/token_type.rs` | Unchanged from `src/token_type.rs` |
| Delete | `src/` (old root source dir) | No longer needed after move |
| Create | `ast_generator/Cargo.toml` | Package manifest for the generator binary |
| Create | `ast_generator/src/main.rs` | Generator binary — outputs `expr.rs` |
| Generated | `rlox/src/expr.rs` | Written by running `cargo run -p ast_generator -- rlox/src` |

---

### Task 1: Convert root `Cargo.toml` to a workspace manifest

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Replace root Cargo.toml with workspace manifest**

  Write this content to `Cargo.toml` (replaces all existing content):

  ```toml
  [workspace]
  members = ["rlox", "ast_generator"]
  resolver = "2"
  ```

- [ ] **Step 2: Verify syntax is valid**

  Run: `cargo metadata --no-deps 2>&1 | head -5`

  Expected: JSON output starting with `{"packages":` or similar — no `error` lines.

---

### Task 2: Create the `rlox` subcrate

**Files:**
- Create: `rlox/Cargo.toml`
- Create: `rlox/src/main.rs`
- Create: `rlox/src/scanner.rs`
- Create: `rlox/src/token.rs`
- Create: `rlox/src/token_type.rs`

- [ ] **Step 1: Create `rlox/Cargo.toml`**

  ```toml
  [package]
  name = "rlox"
  version = "0.1.0"
  edition = "2024"

  [dependencies]
  regex = "1.12.3"
  ```

- [ ] **Step 2: Copy source files into `rlox/src/`**

  Read each file from the old `src/` location and write it verbatim to `rlox/src/`:

  - `src/scanner.rs` → `rlox/src/scanner.rs` (no changes)
  - `src/token.rs` → `rlox/src/token.rs` (no changes)
  - `src/token_type.rs` → `rlox/src/token_type.rs` (no changes)

- [ ] **Step 3: Create `rlox/src/main.rs` with `mod expr` added**

  This is `src/main.rs` with `mod expr;` inserted after the existing module declarations:

  ```rust
  use std::{env, process::exit, fs, io, io::Write};

  mod token_type;
  mod token;
  mod scanner;
  mod expr;

  use scanner::Scanner;

  fn main() {
      let args: Vec<String> = env::args().collect();

      if args.len() > 2 {
          println!("Usage: rlox [script]");
          exit(1);
      } else if args.len() == 2 {
          run_file(args[1].to_string());
      } else {
          run_prompt();
      }
  }

  fn run_file(path: String) {
      let contents = fs::read_to_string(path).expect("Error reading the file");
      run(contents);
  }

  fn run_prompt() {
      loop {
          print!("> ");
          io::stdout().flush().expect("Error flushing stdout");
          let mut source = String::new();
          let bytes_read = io::stdin().read_line(&mut source).expect("Error reading the user input");
          if bytes_read == 0 {
              break;
          }
          run(source);
      }
  }

  fn run(source: String) {
      let mut scanner = Scanner::new(source);
      let tokens = scanner.scan_tokens();
      for token in &tokens {
          println!("{}", token);
      }
  }

  #[allow(dead_code)]
  fn error(line: i32, message: String) {
      report(line, "".to_string(), message);
  }

  #[allow(dead_code)]
  fn report(line: i32, location: String, message: String) {
      println!("[line {} ] Error {} :{}", line, location, message);
  }
  ```

- [ ] **Step 4: Remove the old `src/` directory**

  ```bash
  rm -rf src/
  ```

- [ ] **Step 5: Verify `rlox` builds (will fail until expr.rs is generated — that is expected)**

  Run: `cargo build -p rlox 2>&1 | head -20`

  Expected: error about `expr.rs` being missing/empty — confirms the workspace resolves the `rlox` member correctly. If the error is instead about the workspace itself, stop and fix before proceeding.

---

### Task 3: Create the `ast_generator` crate

**Files:**
- Create: `ast_generator/Cargo.toml`
- Create: `ast_generator/src/main.rs`

- [ ] **Step 1: Create `ast_generator/Cargo.toml`**

  ```toml
  [package]
  name = "ast_generator"
  version = "0.1.0"
  edition = "2024"

  [dependencies]
  ```

- [ ] **Step 2: Create `ast_generator/src/main.rs`**

  ```rust
  use std::{env, fmt::Write, fs, process};

  fn main() {
      let args: Vec<String> = env::args().collect();
      if args.len() != 2 {
          eprintln!("Usage: ast_generator <output_dir>");
          process::exit(1);
      }
      let output_dir = &args[1];

      let types: Vec<(&str, Vec<(&str, &str)>)> = vec![
          ("Binary",   vec![("left", "Expr"), ("operator", "Token"), ("right", "Expr")]),
          ("Grouping", vec![("expression", "Expr")]),
          ("Literal",  vec![("value", "Option<crate::token::Literal>")]),
          ("Unary",    vec![("operator", "Token"), ("right", "Expr")]),
      ];

      let mut out = String::new();

      // Structs
      for (name, fields) in &types {
          writeln!(out, "pub struct {name} {{").unwrap();
          for (field_name, field_type) in fields {
              writeln!(out, "    pub {field_name}: {},", map_type(field_type)).unwrap();
          }
          writeln!(out, "}}").unwrap();
          writeln!(out).unwrap();
      }

      // Expr enum
      writeln!(out, "pub enum Expr {{").unwrap();
      for (name, _) in &types {
          writeln!(out, "    {name}(Box<{name}>),").unwrap();
      }
      writeln!(out, "}}").unwrap();
      writeln!(out).unwrap();

      // Visitor trait
      writeln!(out, "pub trait Visitor<T> {{").unwrap();
      for (name, _) in &types {
          let method = name.to_lowercase();
          writeln!(out, "    fn visit_{method}(&self, expr: &{name}) -> T;").unwrap();
      }
      writeln!(out, "}}").unwrap();
      writeln!(out).unwrap();

      // accept impl
      writeln!(out, "impl Expr {{").unwrap();
      writeln!(out, "    pub fn accept<T>(&self, visitor: &impl Visitor<T>) -> T {{").unwrap();
      writeln!(out, "        match self {{").unwrap();
      for (name, _) in &types {
          let method = name.to_lowercase();
          writeln!(out, "            Expr::{name}(e) => visitor.visit_{method}(e),").unwrap();
      }
      writeln!(out, "        }}").unwrap();
      writeln!(out, "    }}").unwrap();
      writeln!(out, "}}").unwrap();

      let output_path = format!("{output_dir}/expr.rs");
      fs::write(&output_path, &out).expect("Failed to write expr.rs");
      println!("Written: {output_path}");
  }

  fn map_type(t: &str) -> &str {
      match t {
          "Expr"  => "Box<Expr>",
          "Token" => "crate::token::Token",
          other   => other,
      }
  }
  ```

- [ ] **Step 3: Verify the generator crate compiles**

  Run: `cargo build -p ast_generator 2>&1`

  Expected: `Compiling ast_generator v0.1.0` then `Finished` with no errors.

---

### Task 4: Run the generator and verify the full build

**Files:**
- Generated: `rlox/src/expr.rs`

- [ ] **Step 1: Run the generator**

  Run from the workspace root:

  ```bash
  cargo run -p ast_generator -- rlox/src
  ```

  Expected output: `Written: rlox/src/expr.rs`

- [ ] **Step 2: Inspect the generated file**

  Run: `cat rlox/src/expr.rs`

  Expected: Four sections — four structs, `Expr` enum, `Visitor<T>` trait, `impl Expr` with `accept`. The file should start with `pub struct Binary {`.

- [ ] **Step 3: Build the rlox interpreter**

  Run: `cargo build -p rlox 2>&1`

  Expected: `Compiling rlox v0.1.0` then `Finished` with no errors.

- [ ] **Step 4: Build the full workspace**

  Run: `cargo build 2>&1`

  Expected: `Finished` with no errors.