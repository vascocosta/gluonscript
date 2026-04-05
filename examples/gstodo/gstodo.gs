#!/usr/bin/env gluonscript

env = import("std/env")
fs = import("std/fs")
io = import("std/io")
json = import("std/json")
strings = import("std/strings")

commands = import("commands")
error = import("error")

fn main() {
    config =
        env.vars().HOME
        |> append("/" + ".gstodo.json")
        |> fs.read_file() |> error.handle_error()
        |> json.parse() |> error.handle_error()

    while true {
        io.print("> ")
        cmd = io.input()

        if strings.lower(cmd) == "exit" || strings.lower(cmd) == "quit" {
            return 1
        }

        output = commands.handle_cmd(cmd, config)

        io.println(output)
    }
}
