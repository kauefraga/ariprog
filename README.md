# Ariprog

![GitHub top language](https://img.shields.io/github/languages/top/kauefraga/ariprog)
![Crates.io Version](https://img.shields.io/crates/v/ariprog)
![GitHub's license](https://img.shields.io/github/license/kauefraga/ariprog)
![GitHub last commit (branch)](https://img.shields.io/github/last-commit/kauefraga/ariprog/main)

## 🔥 Features

Here's a list of what **ariprog** is capable of.

- [x] Solve AP problems with `get_common_difference`, `get_first_term`, `get_nth_term` and `insert_arithmetic_means` functions.
- [ ] Store an AP (`new`) and solve problems with methods.

## 🛠 Usage

### Installation

First, create a new Rust project or open an existing one

```bash
cargo new testing-ariprog
cd testing-ariprog

# or
cd existing-project
```

Then, add [**ariprog**](https://crates.io/crates/ariprog) as a dependency

```bash
cargo add ariprog
```

As an alternative, you can add the following line in your `Cargo.toml` (dependencies section)

```bash
ariprog = "0.1.1"
```

### Getting started

```rust
use ariprog::{get_common_difference, get_nth_term};

fn main() {
    let common_diff = get_common_difference(6.0, 2.0); // expected 4.0

    println!(
        "The common difference in the AP [2.0, 6.0, 10.0, 14.0] is {}",
        common_diff
    );

    println!(
        "The seventeenth term of the AP [2.0, 6.0, 10.0, 14.0, ...] is {}",
        get_nth_term(2.0, common_diff, 17.0)
    ); // expected 66.0
}
```

### API

Coming soon...

## 💖 Contributing

Feel free to fork it, make a change and open a pull request. Same for issues, suggest an API change, an improvement, a feature or report a bug.

### How to contribute

1. Fork this repository
2. Clone your fork on your machine
3. Make your changes, commit and push these
4. Open a pull request (write a descriptive message about what you changed)

## 🧪 Testing

To test ariprog, with the project in your machine, run `cargo test`.

You should create tests in [`tests/unit_test.rs`](tests/unit_test.rs), however if you think that your tests should be in other module, do it and explain why in the PR.

## 📝 License

This project is licensed under the MIT License - See the [LICENSE](https://github.com/kauefraga/ariprog/blob/main/LICENSE) for more information.

---

Made with ❤ and 🦀 by Kauê Fraga Rodrigues.
