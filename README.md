# gluonscript

A dynamically typed language with the bare minimum features to be enjoyable and simple to use.

## Features

* Minimalist but ergonomic and consistent syntax
* Lists (immutable Python-like lists)
* Records (immutable Javascript-like objects)
* First-class functions
* Lambdas
* Closures
* Easy collection iteration
* Functions that might fail return { "error": Bool, "value": Value }
* One single way to do things
* Rich built-in std library
* Balanced imperative/functional style
* Ideal for new programmers
* Ideal for experienced programmers
* Implemented in Rust taking advantage of its ownership model (no GC)

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
```
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

println(even_odd(numbers))
```
Output:
[List([Int(2), Int(4), Int(6)]), List([Int(1), Int(3), Int(5)])]
```

### Example 3

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