<table>
  <tr>
    <td valign="middle">
      <img src="https://gluonscript.org/logo.png" width="64">
    </td>
    <td valign="middle">
      <h1 style="margin: 0;">GluonScript</h1>
    </td>
  </tr>
</table>

A dynamically typed language with the bare minimum features to be enjoyable and simple to use.

```python
io = import("std/io")
strings = import("std/strings")

fn main() {
    list = ["Hi", "World!", "Goodbye"]

    text = list
        |> slice(0, len(list) - 1)
        |> strings.join(" ")
        |> strings.replace("Hi", "Hello")
        |> strings.upper()

    io.println(text)
}
```

```
HELLO WORLD!
```

## Philosophy

GluonScript was born out of my interest in learning how to implement an interpreted programming language in Rust. However it soon evolved from a toy language into a language that I enjoy using for real scripting purposes as the syntax and main concepts were inspired by some of the languages I enjoy the most. The language draws from Rust, Python, Go, JavaScript and Gleam, as you may notice on its syntax, types and ideas.

The core guiding principle is minimalism, but without making it boringly simple. I will keep the language simple enough to be able to keep it in my head or for a new learner to learn it in a weekend, nevertheless I do like some features that despite not being strictly necessary, make using the language more enjoyable. For instance, I consider first-class functions, lambdas, closures and immutable data structures crucial. That said, I also like a good balance between imperative and functional style, supporting both.

## Features

* Minimalist, ergonomic and consistent syntax
* Immutable Python-like lists
* Immutable JavaScript-like records
* First-class functions, lambdas and closures
* Function pipe operator (`|>`)
* Easy collection iteration
* Lightweight error propagation with `?=`
* Rich standard library
* Balanced imperative and functional style
* Friendly to both new and experienced programmers
* Implemented in Rust without a garbage collector

## Examples

### Example 1

```python
io = import("std/io")

fn generic_operation(a, b, operation) {
    operation(a, b)
}

sum = fn(x , y) { x + y }
sub = fn(x , y) { x - y }
mul = fn(x , y) { x * y }
div = fn(x , y) { x / y }

operations = [sum, sub, mul, div]

for operation in operations {
    result = generic_operation(4, 2, operation)
    io.println(result)
}
```

```
Output:
6
2
8
2
```

### Example 2

```python
io = import("std/io")

fn even_odd(numbers) {
    even = []
    odd = []
    all = []

    i = 0
    while i < len(numbers) {
        if numbers[i] % 2 == 0 {
            even = append(even, numbers[i])
        } else {
            odd = append(odd, numbers[i])
        }

        i += 1
    }

    all = append(all, even)
    all = append(all, odd)

    return all
}

numbers = [1, 2, 3, 4, 5, 6]

io.println(even_odd(numbers))
```

```
Output:
[[2, 4, 6], [1, 3, 5]]
```

### Example 3

```python
env = import("std/env")
http = import("std/http")
io = import("std/io")
strings = import("std/strings")

fn get_weather(location) {
    # Get may fail so it returns { "error": Bool, "value": Any }.
    # The last expression is returned even without the return keyword.
    # A function returns an expression that evaluates to a value and is returned.
    http.get("https://wttr.in/" + location + "?format=3")
}

fn main() {
    args = env.args()

    if len(args) < 3 {
        io.println("Usage: weather.gs <location>")
        return 1
    }

    # The function pipe |> operator makes data manipulation easy to reason.
    location =
        args # List of all arguments passed to the program.
        |> slice(2, len(args)) # Slice the list to exclude unwanted elements.
        |> strings.join(" ") # Join elements of args to form a location string.

    result = get_weather(location)

    # By convention functions that might fail return { "error": Bool, "value": Any }.
    # Checking this record for an error is a common pattern in gluonscript.
    # If error is true, value shows its message as a string.
    # Otherwise if error is false, value shows whatever value the function returns.
    # This is similar in spirit to what languages like Go or Rust do.
    if result.error {
        io.println("Could not fetch weather: " + result.value)
    } else {
        io.print(result.value)
    }
}
```

```
Output:
lisbon: 🌦   +14°C
```

### Example 4

```python
http = import("std/http")
io = import("std/io")
json = import("std/json")

fn fetch(url) {
    # The ?= operator (error propagation) is similar to Rust's ? operator.
    # It makes it easy to handle { "error": Bool, "value": Any } return types.
    # If the value on the right is an error, it propagates the error to the caller.
    # Otherwise it unwraps value and assigns it to the variable on the left.

    raw_json ?= http.get(url) # Get raw json from server or propagate error.
    parsed_json ?= json.parse(raw_json) # Parse JSON into a record or propagate error.
    
    # Functions using the ?= operator should return { "error": Bool, "value": Any }.
    # This record is similar to Rust's Result type, but simpler and without generics.
    { error: false, value: parsed_json }
}

fn main() {
    content = fetch("https://catfact.ninja/fact")

    if content.error {
        io.println("There was an error: " + content.value)
        return 1
    }

    io.println(content.value.fact)
}
```

```
A cat can jump 5 times as high as it is tall.
```