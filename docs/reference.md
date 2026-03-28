---
layout: default
title: Reference
nav_order: 3
has_children: true
---

# Built-in Functions Reference
Explore the standard library functions available in GluonScript.

## Module `core`

[fn append(list: List, element: Any): List](https://gluonscript.org/reference.html#fn-appendlist-list-element-any-list)

[fn import(file: String): Record](https://gluonscript.org/reference.html#fn-importfile-string-record)

[fn len(list: List): Int](https://gluonscript.org/reference.html#fn-lenlist-list-int)

[fn slice(list: List, start: Int, end: Int): List](https://gluonscript.org/reference.html#fn-slicelist-list-start-int-end-int-list)

## Module `conv`

## Module `env`

## Module `http`

## Module `io`

## Module `json`

## Module `strings`

---

## Module: `core`

### `fn append(list: List, element: Any): List`
Returns a new list with the element passed as argument appended to it.

### `fn import(file: String): Record`
Returns a record with all the functions exported by the module defined in file.

### `fn len(list: List): Int`
Returns the length of a list.

### `fn slice(list: List, start: Int, end: Int): List`
Returns a new list which is a slice of the list passed as argument, between index start and end.

---

## Module: `conv`

### `fn float(string: String): Float`
Returns a float by converting the string passed as argument.

### `fn int(string: String): Int`
Returns an int by converting the string passed as argument.

### `fn string(any: Any): String`
Returns a string by converting the type passed as argument into its string representation.

---

## Module: `env`

### `fn args(): List`
Returns a list with all the command line arguments passed into the script.

---

## Module: `http`

### `fn get(url: String): Record { error: Bool, value: String }`
Returns a result record with a boolean error and the server response from url as a string.

---

## Module: `io`

### `fn input(): String`
Returns a string with the content read from the standard input.

### `fn print(any: Any...): None`
Prints every value passed as argument to the standard output.

### `fn println(any: Any...): None`
Prints every value passed as argument to the standard output and appends a newline char.

---

## Module: `json`

### `fn parse(json: String): Record`
Returns a record with the parsed json string passed as argument.

---

## Module: `strings`

### `fn join(list: List, sep: String): String`
Returns a string formed by joining all the elements in the list passed as argument separated by sep.

### `fn lower(string: String): String`
Returns a new string with the string passed as argument converted to lower case.

### `fn upper(string: String): String`
Returns a new string with the string passed as argument converted to upper case.

### `fn split(string: String, sep: String): List`
Returns a list with the string passed as argument separated by sep into elements of the list.