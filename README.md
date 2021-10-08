<!-- markdownlint-disable MD033 -->
<h1 align="center">RsUnit</h1>

<p align="center">⚠️⚠️ Important: This project is work in progress. <br> Right now it's just a test if a structure and framework like this is feasible. ⚠️⚠️</p>

RsUnit is a unit testing framework for Rust. It's a wrapper around the native `cargo test` interface. RsUnit mimics the structure and behavior of [ExUnit](https://hexdocs.pm/ex_unit/1.12/ExUnit.html).

## How to use

## Structure

A test suite is always wrapped in a `rs_unit` block. Inside this `rs_unit` block can be multiple `describe` blocks. A `describe` block is used to group related tests. The essence of **RsUnit** is the `describe` block. This block consists of three parts; a `setup`, `test` and `teardown` part. The `setup` and `teardown` are actually optional. The only required part is the `test` part.

### Setup

The `setup` block contains logic that is run before the tests. Here we differentiate between `setup` and `setup_all`. The `setup` block is run with every test. The `setup_all` block is executed a single time, before every test. Database or drive setups are use cases for the `setup` block.

### Test

Each `describe` block contain multiple `test` blocks. There is no limit but keep in mind to keep your tests as short as possible.

### Teardown

The `teardown` block is run after the tests. Similar to the `setup` blocks, you have the choice between `teardown` and `teardown_all`. A typical use case is for example the removal of the database or folders that were created in the `setup` block.

## Example

```rust
pub fn add_number(number_one: i32, number_two: i32) -> i32 {
    number_one + number_two
}

#[cfg(test)]
mod tests {
    use rs_unit::rs_unit;
    use super::*;

    rs_unit! {
        describe "add_number/2" {
            test "success: Add two numbers" {
                let result = add_number(1, 1);

                assert_eq!(result, 2);
            }
        }
    }
}
```

## License

MIT License - Copyright (c) 2021 Marius Wilms
