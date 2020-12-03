# Holmes - deduction tool

![Rust](https://github.com/Glebanister/holmes/workflows/Rust/badge.svg)

## Install

First, make sure `git version`, `cargo version` works fine on your machine.

```bash
git clone https://github.com/Glebanister/holmes.git
cd holmes
cargo build --release
```

## Use

Run `target/release/holmes` to start application. You will see hello message,
then you can extract a few commands

| Command   | Description                                   |
|-----------|-----------------------------------------------|
| `!<fact>` | Add `<fact>` to deductor space                |
| `?<fact>` | Ask if `<fact>` can be deduced deductor space |

`<fact>` can be one of:

* `<fact>: [A-Za-z]+` - just some text

* `<fact>: <fact> -> <fact>` - implication of two facts, which means, that if left fact exists, then right exists too. Operator `->` is right associative.

## Examples

### Startup

```bash
./target/release/holmes
Welcome to holmes!
>>
```

### Deduction

```plain
Welcome to holmes - deduction tool!
~> help

!<fact> : tell holmes that <fact> exists
?<fact> : ask holmes if <fact> exists
<fact> can be
- a string literal (it's raining)
- an implication of facts (it's raining -> take an umbrella)

~> !it's raining
~> !it's raining -> take an umbrella
~> ?take an umbrella
true
~> exit
```
