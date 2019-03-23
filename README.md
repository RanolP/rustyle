# rustyle

[![Travis (.com)](https://img.shields.io/travis/com/RanolP/rustyle.svg)](https://travis-ci.com/RanolP/rustyle)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/RanolP/rustyle.svg)
![Crates.io](https://img.shields.io/crates/v/rustyle.svg)
![Crates.io](https://img.shields.io/crates/d/rustyle.svg)
![Crates.io](https://img.shields.io/crates/l/rustyle.svg)

A new way to represent the CSS stylesheet in Rust

## Basic Information

Read like \[rough style\]. It seems like styled-components, emotion, glamor, and other CSS-in-JS libraries. It's basically inspired by their concepts. But more friendly with rust.

## Syntax

Write CSS-in-Rust like this! (We call it rusty css syntax)

```rust
let CLASS = css! {
  background-color: gray;

  &:hover {
    background-color: lighten!(15%, gray);
  }

  &:not(:hover) > p {
    display: none;
  }
}
```

## Special features which not exists on CSS

### Ruleset Metadata

Ruleset Metadata syntax is inspired by the metadata syntax in Rust.
It appends the metadata of ruleset which controls the behavior of code generator.
For the cleaner code, We decided it should not appear after other rule appeared.

Syntax:

```rust
let Ruleset = rustyle! {
  #![ruleset_metadata]
  #![ruleset_metadata(with_param)]
}
```

rustyle embedded those ruleset attributes:

- `#![inject_global]` (todo)
- `#![no_optimize]` (todo)
- `#![no_collapse]` (todo)

### Rule Metadata

Rule Metadata syntax is inspired by the attribute syntax in Rust.
It appends the metadata of rule which gives hint to the code generator.
Affect on the rule on the next line.

Syntax:

```rust
let Ruleset = rustyle! {
  #[rule_metadata]
  the: rule;

  #[rule_metadata]
  the rule {
    #[rule_metadata(with_param)]
    the: rule;
  }
}
```

rustyle embedded those ruleset attributes:

- `#[allow(vendor_prefix)]` (todo)
- `#[no_optimize]` (todo)

### Function

Function seems like Sass, Less, etc's. But more likey with macro on Rust.
Function evaluated at the compile time. And our custom function ends with exclamation(!) character.
You can declare custom function if we can :p.

Syntax:

```rust
let Ruleset = rustyle! {
  expression_position: function!();

  statement_position!();
}
```

rustyle embedded those functions:

- `adjust_hue!(color, angle)` (todo)
- `lighten!(color, amount)` (todo)
- `darken!(color, amount)` (todo)
- `saturate!(color, amount)` (todo)
- `desaturate!(color, amount)` (todo)
- `transparentize!(color, amount)` (todo)
- `grayscale!(color)` (todo)
- `complement!(color)` (todo)
- `invert!(color)` (todo)

## How's it works

It's written in proc macro. The css codes checked and wrote at compile time. macro calls replaced to randomly generated class names.
