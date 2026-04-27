---
layout: default
title: Reference
nav_order: 3
has_children: true
---

## GluonScript Standard Library

Explore the standard library functions.

---

## Module `core`

[fn append(list: List, element: Any): List](#fn-appendlist-list-element-any-list)<br>
[fn exit(code: Int): None](#fn-exitint-none)<br>
[fn import(file: String): Record](#fn-importfile-string-record)<br>
[fn len(list: List): Int](#fn-lenlist-list-int)<br>
[fn slice(list: List, start: Int, end: Int): List](#fn-slicelist-list-start-int-end-int-list)
[fn update(collection: List | Record, element: Int | String, value: Any): List | Record]()

---

## Module `conv`

[fn float(string: String): Record { error: Bool, value: Float }](#fn-floatstring-string-record--error-bool-value-float-)<br>
[fn int(string: String): Record { error: Bool, value: Int }](#fn-intstring-string-record--error-bool-value-int-)<br>
[fn string(any: Any): String](#fn-stringany-any-string)

---

## Module `env`

[fn args(): List](#fn-args-list)<br>
[fn consts(): Record](#fn-consts-record)<br>
[fn vars(): Record](#fn-vars-record)

---

## Module `fs`

[fn create_dir(path: String): Record { error: Bool, value: None }](#fn-create_dirpath-string-record--error-bool-value-none-)<br>
[fn read_file(path: String): Record { error: Bool, value: String }](#fn-read_filepath-string-record--error-bool-value-string-)<br>
[fn write_file(path: String, contents: String): Record { error: Bool, value: None }](#fn-write_filepath-string-contents-string-record--error-bool-value-none-)

---

## Module `http`

[fn get(url: String): Record { error: Bool, value: String }](#fn-geturl-string-record--error-bool-value-string-)

---

## Module `io`

[fn input(): String](#fn-input-string)<br>
[fn print(any: Any...): None](#fn-printany-any-none)<br>
[fn println(any: Any...): None](#fn-printlnany-any-none)

---

## Module `iter`

[fn filter(list: List, predicate: fn(e: Any): bool): List](#fn-filterlist-list-predicate-fne-any-bool-list)<br>
[fn find(list: List, predicate: fn(e: Any): bool): Any](#fn-findlist-list-predicate-fne-any-bool-any)<br>
[fn map(list: List, f: fn(e: Any): Any): List](#fn-maplist-list-f-fne-any-any-list)<br>
[fn take(list: List, n: Int): List](#fn-takelist-list-n-int-list)

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

## `fn exit(code: Int): None`

Terminates the current process immediately and passes the specified exit code to the operating system.

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

## `fn update(collection: List | Record, element: Int | String, value: Any): List | Record`

Returns a new collection containing the same elements of the collection passed as first argument, which can be either
a list or a record, but with the element at the list index or record field specified by element replaced by value.

---

## Module: `conv`

## `fn float(string: String): Record { error: Bool, value: Float }`

Converts the given string into a floating-point number and returns a result record. The `error` field indicates whether the conversion failed, and the `value` field contains either the floating-point number or an error message.

---

## `fn int(string: String): Record { error: Bool, value: Int }`

Converts the given string into an integer number and returns a result record. The `error` field indicates whether the conversion failed, and the `value` field contains either the integer number or an error message.

---

## `fn string(any: Any): String`

Returns the string representation of the given value.

---

## Module: `env`

## `fn args(): List`

Returns a list containing all command-line arguments passed to the script, in the order they were provided.

---

## `fn consts(): Record`

Returns a record containing all environment constants of the current process as key/value pairs.

---

## `fn vars(): Record`

Returns a record containing all environment variables of the current process as key/value pairs.

---

## Module: `fs`

## `fn create_dir(path: String): Record { error: Bool, value: None }`

Creates the directory specified in path in the current directory and returns a result record. The `error` field indicates whether the directory creation operation failed, and the `value` field contains either none or an error message.

---

## `fn read_file(path: String): Record { error: Bool, value: String }`

Reads the file located at the specified path and returns a result record. The `error` field indicates whether the read operation failed, and the `value` field contains either the file contents as a string or an error message.

---

## `fn write_file(path: String, contents: String): Record { error: Bool, value: None }`

Writes the specified contents to the file located at the specified path and returns a result record. The `error` field indicates whether the read operation failed, and the `value` field contains either none or an error message.

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

## Module: `iter`

## `fn filter(list: List, predicate: fn(e: Any): bool): List`

Returns a new list containing only the elements of the input list for which the predicate function returns `true`. The original list is not modified.

---

## `fn find(list: List, predicate: fn(e: Any): bool): Any`

Returns the first element in the list for which the predicate function returns `true`. If no such element is found, `None` is returned.

---

## `fn map(list: List, f: fn(e: Any): Any): List`

Returns a new list containing the results of applying the given function to each element of the input list. The original list is not modified.

---

## `fn take(list: List, n: Int): List`

Returns a new list containing the first n elements of the input list. If n exceeds the length of the list, all elements are returned. The original list is not modified.

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
