# bf2s (Brainfuck-to-string)

## Usage
```rust
fn main() {
    let code = "
        -[------->+<]>-.-[->+++++<]>
        ++.+++++++..+++.[->+++++<]>+
        .------------.---[->+++<]>.-
        [--->+<]>---.+++.------.----
        ----.-[--->+<]>.
    ";
    let s = bf2s::bf_to_str(code);
    assert_eq!(s, "Hello, World!")
}
```