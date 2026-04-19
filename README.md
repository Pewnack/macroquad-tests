# Macroquad Test Suite

A collection of small test programs for the [macroquad](https://github.com/not-fl3/macroquad) game framework.

## Running Tests

Each test is a standalone binary executable.

### Available Tests

| Binary | Description |
|--------|-------------|
| `mouse` | Shows current mouse coordinates centered in the window on a black background. |
| `keyboard` | Displays currently pressed keys centered in the window; modifier keys (Shift, Ctrl, Alt, Super) are shown above in yellow, regular keys are shown in white. |

Run the mouse test with:

```bash
cargo run --bin mouse
```

Run the keyboard test with:

```bash
cargo run --bin keyboard
```

## Building All Tests

To build all test programs at once:

```bash
cargo build --bins
```

## Testing Against Different Macroquad Versions

Edit `Cargo.toml` to change the macroquad dependency. Options:

- **Main branch:** use a git dependency
- **Specific version from crates.io:** `version = "0.4"`
- **Local path:** `path = "/path/to/macroquad"`

After changing the dependency, run:

```bash
cargo clean
cargo build --bins
```

## Project Structure

```
macroquad-tests/
├── Cargo.toml
├── README.md
├── .gitignore
└── src/
    └── bin/
        ├── keyboard.rs
        └── mouse.rs
```

## Adding New Tests

1. Create a new `.rs` file in `src/bin/`, e.g. `src/bin/my_test.rs`
2. Add a `[[bin]]` entry in `Cargo.toml` pointing to the new file
3. Run with `cargo run --bin my_test`
