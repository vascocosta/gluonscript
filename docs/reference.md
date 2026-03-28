---
layout: default
title: Function Reference
nav_order: 2
has_children: true
---

# Built-in Functions Reference
Explore the standard library functions available in GluonScript.

{: .note }
> All functions below are available in the `core` namespace and do not require explicit imports.

## String Functions

### `fn len(list: List): Int`
Returns the length of a List.

### `fn slice(list List, start: Int, end: Int): List`
Returns a slice of a List, between index start and end.