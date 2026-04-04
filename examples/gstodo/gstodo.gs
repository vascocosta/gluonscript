#!/usr/bin/env gluonscript

env = import("env")
fs = import("fs")
io = import("io")
json = import("json")
strings = import("strings")

commands = import("commands.gs")
error = import("error.gs")

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
