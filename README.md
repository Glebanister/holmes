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

```java
>> ?pain
false
>> !rain on the street -> pain in the soul
>> ?pain
false
>> !rain -> rain on the street
>> !pain in the soul -> pain
>> ?pain
false
>> !rain
>> ?pain
true
>>
```
