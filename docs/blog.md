---
layout: default
title: "Blog"
nav_order: 4
has_children: true
---

# GluonScript 0.4.0 released

GluonScript 0.4.0 introduces several important improvements to the language and runtime. This release focuses on making error handling more ergonomic, expanding the standard functionality available to programs and improving interpreter performance and closure semantics internally.

## Error Propagation Operator `?=`

One of the biggest additions of this release is the new `?=` operator.

Functions that may fail in GluonScript commonly return a record in the following format:

```python
{ error: Bool, value: Any }
```

Checking these records manually quickly becomes repetitive. The new `?=` operator simplifies this pattern by automatically propagating errors.

Before:

```python
result = http.get("https://example.com")

if result.error {
    return result
}

content = result.value
```

Now:

```python
content ?= http.get("https://example.com")
```

If `error` is `false`, the value in `value` is unwrapped and assigned to the variable on the left.

If `error` is `true`, the current function immediately returns the full error record, making error propagation concise and easy to read.

This feature was heavily inspired by Rust's `?` operator while still fitting naturally into GluonScript's existing conventions and simpler "Result" record.

## New `math` Module

Version 0.4.0 also introduces a new built-in `math` module containing common mathematical functionality:

`fn abs(number: Int | Float): Int | Float`<br>
`fn ceil(number: Float): Float`<br>
`fn cos(number: Int | Float): Float`<br>
`fn clamp(value: Int | Float, min: Int | Float, max: Int | Float): Int | Float`<br>
`fn floor(number: Float): Float`<br>
`fn max(a: Int | Float, b: Int | Float): Int | Float`<br>
`fn min(a: Int | Float, b: Int | Float): Int | Float`<br>
`fn round(number: Float): Float`<br>
`fn sin(number: Int | Float): Float`

The goal of this module is to provide convenient access to frequently used mathematical operations without requiring external dependencies.

The module will continue expanding in future releases.

## Runtime Improvements with `Rc<RefCell<Env>>`

Internally, the interpreter previously relied heavily on cloning scope environments during function calls and closure creation. While simple, this approach became undesired as the language evolved and gained more advanced closure support.

In 0.4.0 the runtime now uses:

```rust
Rc<RefCell<Env>>
```

This change significantly improves performance and lowers memory usage by reducing unnecessary environment cloning. All that is cloned now are references to parent scopes, instead of cloning the data structure holding the variables and functions (HashMaps).

In addition to this, closures now capture their surrounding environment by reference, instead of copying a snapshot like previously. This means that if a variable in a parent scope changes its value, this will now be seen by the closure. You can think of it like a live view to its parent scopes.

However, contrary to what you might expect coming from mainstream languages, closures cannot mutate the captured variables. This is a hybrid approach in line with the immutability philosophy followed by GluonScript. This makes sure closures cannot have side effects outside their scope, preventing a whole class of bugs and unexpected results.

## Looking Ahead

GluonScript continues evolving toward a small but expressive scripting language with a strong focus on simplicity, immutable data structures and pragmatic functional programming ideas, while equally supporting imperative styles.

Future work will likely focus on:

- Expanding the standard library
- Refining module support
- Improving interpreter performance
- Additional language ergonomics
- Documentation improvements

Thank you for following the project.