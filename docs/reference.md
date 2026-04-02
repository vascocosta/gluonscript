---
layout: default
title: Reference
nav_order: 3
has_children: true
---

## Built-in Functions Reference
Explore the standard library functions.

## Module `core`

[fn append(list: List, element: Any): List](#fn-appendlist-list-element-any-list)<br>
[fn import(file: String): Record](#fn-importfile-string-record)<br>
[fn len(list: List): Int](#fn-lenlist-list-int)<br>
[fn slice(list: List, start: Int, end: Int): List](#fn-slicelist-list-start-int-end-int-list)

---

## Module `conv`

[fn float(string: String): Float](#fn-floatstring-string-float)<br>
[fn int(string: String): Int](#fn-intstring-string-int)<br>
[fn string(any: Any): String](#fn-stringany-any-string)

---

## Module `env`

[fn args(): List](#fn-args-list)<br>
[fn vars(): Record](#fn-vars-record)

---

## Module `fs`

[fn read_file(path: String): String](#fn-read_filepath-string-record--error-bool-value-string-)

---

## Module `http`

[fn get(url: String): Record { error: Bool, value: String }](#fn-geturl-string-record--error-bool-value-string-)

---

## Module `io`

[fn input(): String](#fn-input-string)<br>
[fn print(any: Any...): None](#fn-printany-any-none)<br>
[fn println(any: Any...): None](#fn-printlnany-any-none)

---

## Module `json`

[fn parse(json: String): Record](#fn-parsejson-string-record)

---

## Module `strings`

[fn contains(string: String, substr: String): Bool](#fn-containsstring-string-substr-string-bool)<br>
[fn join(list: List, sep: String): String](#fn-joinlist-list-sep-string-string)<br>
[fn lower(string: String): String](#fn-lowerstring-string-string)<br>
[fn upper(string: String): String](#fn-upperstring-string-string)<br>
[fn split(string: String, sep: String): List](#fn-splitstring-string-sep-string-list)<br>
[fn replace(string: String, old: String, new: String): String](#fn-replacestring-string-old-string-new-string-string)

---

## Module: `core`

## `fn append(list: List, element: Any): List`

Returns a new list containing all elements of the input list, with the provided element appended at the end. The original list is not modified.

---

## `fn import(file: String): Record`

Loads and executes the module identified by the given file name or module name, returning a record containing all exported values defined in that module.

---

## `fn len(list: List): Int`

Returns the number of elements contained in the given list.

---

## `fn slice(list: List, start: Int, end: Int): List`

Returns a new list containing the elements of the input list between the specified start index (inclusive) and end index (exclusive). The original list is not modified.

---

## Module: `conv`

## `fn float(string: String): Float`

Converts the given string into a floating-point number and returns the result. The string must represent a valid numeric value.

---

## `fn int(string: String): Int`

Converts the given string into an integer and returns the result. The string must represent a valid integer value.

---

## `fn string(any: Any): String`

Returns the string representation of the given value.

---

## Module: `env`

## `fn args(): List`

Returns a list containing all command-line arguments passed to the script, in the order they were provided.

---

## `fn vars(): Record`

Returns a record containing all environment variables of the current process as key/value pairs.

---

## Module: `fs`

## `fn read_file(path: String): Record { error: Bool, value: String }`

Reads the file located at the specified path and returns a result record. The `error` field indicates whether the read operation failed, and the `value` field contains either the file contents as a string or an error message.

---

## Module: `http`

## `fn get(url: String): Record { error: Bool, value: String }`

Performs an HTTP GET request to the specified URL and returns a result record. The `error` field indicates whether the request failed, and the `value` field contains either the response body or an error message.

---

## Module: `io`

## `fn input(): String`

Reads a line of input from standard input and returns it as a string, excluding any trailing newline characters.

---

## `fn print(any: Any...): None`

Writes the string representation of each provided argument to standard output without appending a newline character.

---

## `fn println(any: Any...): None`

Writes the string representation of each provided argument to standard output, followed by a newline character.

---

## Module: `json`

## `fn parse(json: String): Record { error: Bool, value: Record }`

Parses the given JSON string and returns a result record. The `error` field indicates whether parsing failed, and the `value` field contains either the parsed JSON value as a record or an error message.

---

## Module: `strings`

## `fn contains(string: String, substr: String): Bool`

Returns `true` if the given string contains the specified substring, and `false` otherwise.

---

## `fn join(list: List, sep: String): String`

Returns a string created by concatenating the string representation of each element in the list, separated by the specified delimiter.

---

## `fn lower(string: String): String`

Returns a new string with all characters from the input string converted to lowercase.

---

## `fn upper(string: String): String`

Returns a new string with all characters from the input string converted to uppercase.

---

## `fn split(string: String, sep: String): List`

Returns a list of substrings obtained by splitting the input string using the specified separator.

---

## `fn replace(string: String, old: String, new: String): String`

Returns a new string where all occurrences of the specified substring are replaced with the provided replacement string. The original string is not modified.
