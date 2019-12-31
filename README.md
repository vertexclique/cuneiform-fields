<h1 align="center">
    <img src="https://github.com/vertexclique/cuneiform-fields/raw/master/img/cuneiform-logo.png" width="200" height="200"/>
</h1>
<div align="center">
 <strong>
   Field level cache optimizations for Rust (no_std)
 </strong>
<hr>

[![Build Status](https://github.com/vertexclique/cuneiform-fields/workflows/CI/badge.svg)](https://github.com/vertexclique/cuneiform-fields/actions)
[![Latest Version](https://img.shields.io/crates/v/cuneiform-fields.svg)](https://crates.io/crates/cuneiform-fields)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/cuneiform-fields/)
</div>

This crate provides cache line size fitting optimizations to fields in structs.

This crate aligns fields with `#[repr(align(COHERENCE_LINE_SIZE))]` to decrease the time between prefetch signals for data. 
`COHERENCE_LINE_SIZE` can be detected or decided based on the architecture by `cuneiform` itself.

```toml
[dependencies]
cuneiform_fields = "0.1"
```

## Examples

#### Hermetic aligned fields
Align by hermetic cache line size detection mentioned in [cuneiform readme](https://github.com/vertexclique/cuneiform#----):
```rust
use cuneiform_fields::prelude::*;

pub struct Hermetic {
    data: HermeticPadding<u8>,
    data_2: u16,
}
```
In the example above `data` will be aligned by hermetic alignment but field `data_2` isn't going to be alignment optimized.

#### Architecture aligned fields
Align by cache line size detected by current Rust compiler architecture.
If architecture isn't detected in known architectures it will fall back to default alignment:
```rust
use cuneiform_fields::prelude::*;

pub struct ArchSpecific {
    data: ArchPadding<u8>,
    data_2: u16,
}
```
In the example above `data` will be aligned by architecture alignment but field `data_2` isn't going to be alignment optimized.

**NOTE:** Alignment values are not randomly chosen or incorporated directly.
Values are considered and incorporated inside with the mindset of preventing false sharing
or creating less warp points in exclusive caching.

For design choices, architecture and board systems and more information. Please visit [Cuneiform GitHub](https://github.com/vertexclique/cuneiform).