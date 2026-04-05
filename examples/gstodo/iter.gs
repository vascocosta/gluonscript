fn filter(list, predicate) {
    result = []

    for element in list {
        if predicate(element) {
            result = append(result, element)
        }
    }

    result
}