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

    writeln!(out, "// Generated using ast_generator. DO NOT EDIT").unwrap();
    writeln!(out, "use crate::token::Token;").unwrap();
    writeln!(out).unwrap();

    // Expr enum — one variant per grammar rule, matched directly with `match`
    // instead of going through a Visitor trait + accept().
    writeln!(out, "pub enum Expr {{").unwrap();
    for (name, fields) in &types {
        // Literal is a tuple variant (it just wraps a value, no field name
        // adds clarity); every other rule uses named fields so callers can
        // pattern-match on field names directly.
        if *name == "Literal" {
            let (_, field_type) = fields[0];
            writeln!(out, "    {name}({}),", map_type(field_type)).unwrap();
        } else {
            writeln!(out, "    {name} {{").unwrap();
            for (field_name, field_type) in fields {
                writeln!(out, "        {field_name}: {},", map_type(field_type)).unwrap();
            }
            writeln!(out, "    }},").unwrap();
        }
    }
    writeln!(out, "}}").unwrap();

    let output_path = format!("{output_dir}/expr.rs");
    fs::write(&output_path, &out).expect("Failed to write expr.rs");
    println!("Written: {output_path}");
}

fn map_type<'a>(t: &'a str) -> &'a str {
    match t {
        "Expr"  => "Box<Expr>",
        "Token" => "Token",
        other   => other,
    }
}
