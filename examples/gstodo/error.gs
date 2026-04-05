io = import("std/io")

fn handle_error(result) {
    if result.error {
        io.println(result.value)
        exit(1)
    } else {
        return result.value
    }
}
