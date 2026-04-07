fn filter(list, predicate) {
    result = []

    for element in list {
        if predicate(element) {
            result = append(result, element)
        }
    }

    result
}

fn find(list, predicate) {
    for element in list {
        if predicate(element) {
            return element
        }
    }
}

fn map(list, f) {
    result = []

    for element in list {
        result = append(result, f(element))
    }

    result
}

fn take(list, n) {
    slice(list, 0, n)
}