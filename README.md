# Clay

[![Crates.io][crates_badge]][crates]
[![Docs.rs][docs_badge]][docs]
[![Travis CI][travis_badge]][travis]
[![License][license_badge]][license]

[crates_badge]: https://img.shields.io/crates/v/clay.svg
[docs_badge]: https://docs.rs/clay/badge.svg
[travis_badge]: https://api.travis-ci.org/clay-rs/clay.svg?branch=master
[license_badge]: https://img.shields.io/crates/l/clay.svg

[crates]: https://crates.io/crates/clay
[docs]: https://docs.rs/clay
[travis]: https://travis-ci.org/clay-rs/clay
[license]: #license

Flexible Monte-Carlo ray tracing engine written in Rust and OpenCL.

## Features

Clay ray tracing engine is:
+ **Fast** - because of the OpenCL, Clay is able to run its kernel code in heterogenous
  computing systems (e.g. GPUs), that makes it much faster than CPU-only analogs,
  and allows it to render images of sufficient quality even in real-time.
+ **Modular** - Clay is based on strict but flexible Rust trait system and type parametrization,
  that means you can assemble desired ray tracing pipeline from primitive building blocks.
+ **Extendable** - if desired functionality doesn't exist in Clay yet, you always can write
  it by yourself by implementing corresponding traits. Moreover, you can even write your own
  modules of OpenCL code to run on a GPU. (And make a pull request after that, if you want to.)

## About

This project is primarily aimed to be a convenient framework to experimenting with ray tracing,
testing new techniques, making proof of concepts and other research activity in this field.

The key principles of the project is modularity and extendability.
The performance is also one of the primary goals, as long as it doesn't significantly reduce flexibility.

You can find more information at the [Clay project website](https://clay-rs.github.io).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.


*"OpenCL and the OpenCL logo are trademarks of Apple Inc. used by permission by Khronos."*
