# Known Issues

## Literal Value

### Hex Color Literal

Hex literal cannot be compiled because the literal conflict with the integer literal of Rust.

#### Example

- `#0e0e0e` (conflict with exponentiation syntax; 1.234E+567)
- `#0b0b0b` (conflict with binary syntax; 0b00001010)

#### Solution

You can bypass that by using our hex-string-color literal like this: `#"0e0e0e"`, `#"0b0b0b"`
