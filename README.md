# rustyle

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

## How's it works

It's written in proc macro. The css codes checked and wrote at compile time. macro calls replaced to randomly generated class names.
