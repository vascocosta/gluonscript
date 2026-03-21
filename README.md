# gluonscript

A dynamically typed language with the bare minimum features to be enjoyable and simple to use.

## Features

* Immutable values
* Lists
* Records
* First-class functions
* Lambdas
* Closures
* Easy collection iteration
* One single way to do things
* Rich built-in std library
* Balanced imperative/functional style
* Ideal for new programmers
* Ideal for experienced programmers

## Examples

### Example 1

```Rust
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
    println(result)
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

```Rust
fn get_weather(location) {
    get("https://wttr.in/" + location + "?format=3")
}

fn main() {
    args = args()

    if len(args) != 3 {
        println("Usage: weather.gs <location>")
        return 1
    }

    result = get_weather(args[2])

    if result.error {
        println("Could not fetch weather: " + result.value)
    } else {
        print(result.value)
    }
}

main()
```

```
Output:
lisbon: 🌦   +14°C
```