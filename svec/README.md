# `svec`

[![](http://meritbadge.herokuapp.com/svec)](https://crates.io/crates/svec)
[![](https://docs.rs/svec/badge.svg)](https://docs.rs/svec)

Svec lets you create beautiful Dart-like lists in Rust that are both readable and concise.

If you're making a list of things in Rust, you're probably using `vec`.

```rust
// a list
let row = vec![
	Elem::IconButton("hamburger"),
	Elem::Space,
	Elem::IconButton("info"),
	Elem::IconButton("profile")
];
```

`svec` lets you do all the things you can do with `vec`, but it also adds "collection if" and "collection for".

```rust
// a list with svec
let row = svec![
	Elem::IconButton("hamburger"),
	Elem::Space,
	Elem::IconButton("info"),
	Elem::IconButton("profile"),
	if isLiteVersion { Elem::IconButton("store") }
];
```

Here's a "collection for".

```rust
// a list with vec + svec
let row = vec![
	Elem::IconButton("hamburger"),
	Elem::Space,
	Elem::IconButton("info"),
	Elem::IconButton("profile"),
	Elem::MenuBar(svec![
		for friend in friends.take(3) { Elem::MenuItem(friend) },
		Elem::MenuItem("All friends"),
		Elem::MenuItem("All people"),
	])
];
```

Using `svec` in your project is super easy.

1. Add `svec = 0.1.0` to your `Cargo.toml`.
2. Add `use svec::*`.