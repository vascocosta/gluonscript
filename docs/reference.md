---
layout: default
title: Function Reference
nav_order: 2
has_children: true
---

# Built-in Functions Reference
Explore the standard library functions available in GluonScript.

## Module: `core`

### `fn append(list: List, element: Any): List`
Returns a new list with the element passed as argument appended to it.

### `fn import(file: String): Record`
Returns a record with all the functions exported by the module defined in file.

### `fn len(list: List): Int`
Returns the length of a list.

### `fn slice(list List, start: Int, end: Int): List`
Returns a new list which is a slice of the list passed as argument, between index start and end.