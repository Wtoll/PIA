# PIA (Packed Integer Array)

PIA is a simple library for the Rust programming language that adds packed integer arrays for mass storage of oddly sized variables.

While a couple packed integer array libraries already existed in the Rust ecosystem, none seemed to be completely featureful. PIA noteably utilizes const generics in order to allow packed integer arrays of any size or resolution to be created with the added benefits of being housed entirely on the stack, and being able to leverage Rust's compile time guarantees. On top of that, PIA underneath all of the method implementations, is basically just a glorified array, so much so that the entire struct is simply a `[repr(transparent)]` array of `u8`s. All of this means that PIA is designed to be about as bare-metal as a packed integer array implementation can get.

To get started simply construct a new instance of a [`PackedIntegerArray`] with the desired amount of items and bits per item.
```rust
// Constructs a new packed integer array with 5 bits per item and 4 items
let packed_array = pia::PackedIntegerArray::<5, 4>::new();
```

After that, use the array just like any other array. Items can be set using `PackedIntegerArray::set()`, items can be queried using `PackedIntegerArray::get()`, and items can be reset back to 0 using `PackedIntegerArray::clear()`.

For further documentation make sure to see [docs.rs](https://docs.rs/pia/).