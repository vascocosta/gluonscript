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
io = import("io")
strings = import("strings")

fn main() {
    list = ["Not", "Yet", "Yellow", "World", "But", "Almost"]

    text = list
        |> slice(2, len(list) - 2) # Slice the list to get rid of extra words.
        |> strings.join(" ") # Create a string by joining words from the list.
        |> strings.replace("Yellow", "Hello") # Fix the typo (Yellow > Hello).
        |> append("!") # Finally append !, although the + operator also works.

    io.println(text)
}
```

```
Hello World!
```

---

## Philosophy

GluonScript was born out of my interest in learning how to implement an interpreted programming language in Rust. However it soon evolved from a toy language into a language that I enjoy using for real scripting purposes as the syntax and main concepts were inspired by some of the languages I enjoy the most. The language draws from Rust, Python, Go, JavaScript and Gleam, as you may notice on its syntax, types and ideas.

The core guiding principle is minimalism, but without making it boringly simple. I will keep the language simple enough to be able to keep it in my head or for a new learner to learn it in a weekend, nevertheless I do like some features that despite not being strictly necessary, make using the language more enjoyable. For instance, I consider first-class functions, lambdas, closures and immutable data structures crucial. That said, I also like a good balance between imperative and functional style, supporting both.

---

## Features

* Minimalist but ergonomic and consistent syntax
* Lists (immutable Python-like lists)
* Records (immutable Javascript-like objects)
* First-class functions
* Lambdas
* Closures
* Function pipe operator
* Easy collection iteration
* Functions that might fail return { "error": Bool, "value": Value }
* One single way to do things
* Rich built-in std library
* Balanced imperative/functional style
* Ideal for new and experienced programmers
* Implemented in Rust taking advantage of its ownership model (no GC)

---

## Examples

### Example 1

```Python
io = import("io")

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

```Python
io = import("io")

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

```Python
conv = import("conv")
io = import("io")

# Define a function that "updates" a user (records are immutable).
fn birthday(user) {
    return {
        name: user.name,
        age: user.age + 1
    }
}

# Create a record.
user = {
    name: "Vasco",
    age: 44
}

io.println("User:")
io.println("Name: " + user.name)
io.println("Age: " + conv.string(user.age))

# Create a new updated record.
updated = birthday(user)

io.println()
io.println("After birthday:")
io.println("Name: " + updated.name)
io.println("Age: " + conv.string(updated.age))
```

```
User:
Name: Vasco
Age: 44

After birthday:
Name: Vasco
Age: 45
```

### Example 4

```Python
env = import("env")
http = import("http")
io = import("io")
strings = import("strings")

fn get_weather(location) {
    # Get may fail so it returns { "error": Bool, "value": Value }.
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

    # By convention functions that might fail return { "error": Bool, "value": Value }.
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