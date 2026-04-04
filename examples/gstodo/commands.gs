conv = import("conv")
fs = import("fs")
io = import("io")
json = import("json")
strings = import("strings")

error = import("error.gs")

fn parse_cmd(cmd) {
	components = strings.split(cmd, " ")

	for cmd_name in ["help", "list", "add", "del", "toggle"] {
		if strings.lower(components[0]) == cmd_name {
			return {
				error: false,
				value: {
					name: cmd_name,
					args: slice(components, 1, len(components))
				}
			}
		}
	}

	return {
		error: true,
		value: "Unknown command.\n"
	}
}

fn handle_list(parsed_cmd, config) {
	tasks =
		config.db_path
		|> fs.read_file() |> error.handle_error()
		|> json.parse() |> error.handle_error()

	output = ""

	for task in tasks {
		if task.finished {
			status = "✅"
		} else {
			status = "❌"
		}

		output += conv.string(task.id) + ". " + task.description + " " + status + "\n"
	}

	return output
}

fn handle_add(parsed_cmd, config) {
	if len(parsed_cmd.args) == 0 {
		return "Please provide a task description."
	}

	tasks =
		config.db_path
		|> fs.read_file() |> error.handle_error()
		|> json.parse() |> error.handle_error()

	tasks = append(tasks, {
		id: len(tasks) + 1,
		description: strings.join(parsed_cmd.args, " "),
		finished: false
	})

	fs.write_file(config.db_path, conv.string(tasks)) |> error.handle_error()

	return "Task successfully added."
}

fn handle_del(parsed_cmd, config) {
	if len(parsed_cmd.args) == 0 {
		return "Please provide a task id."
	}

	tasks =
		config.db_path
		|> fs.read_file() |> error.handle_error()
		|> json.parse() |> error.handle_error()

	new_tasks = []
	i = 1

	for task in tasks {
		if task.id != conv.int(parsed_cmd.args[0]) {
			new_tasks = append(new_tasks, {
				id: i,
				description: task.description,
				finished: task.finished
			})

			i += 1
		}
	}

	if len(tasks) > len(new_tasks) {
		fs.write_file(config.db_path, conv.string(new_tasks)) |> error.handle_error()

		return "Task successfully deleted."
	} else {
		return "Task not found."
	}

}

fn handle_toggle(parsed_cmd, config) {
	if len(parsed_cmd.args) == 0 {
		return "Please provide a task id."
	}

	parsed_id = conv.int(parsed_cmd.args[0])

	if parsed_id.error {
		return "Invalid task id.\n"
	}

	tasks =
		config.db_path
		|> fs.read_file() |> error.handle_error()
		|> json.parse() |> error.handle_error()

	for task in tasks {
		if task.id == parsed_id.value {
			if task.finished {
				new_status = false
			} else {
				new_status = true
			}

			new_task = {
				id: len(tasks),
				description: task.description,
				finished: new_status
			}
		}
	}

	new_tasks = []
	i = 1

	for task in tasks {
		if task.id != parsed_id.value {
			new_tasks = append(new_tasks, {
				id: i,
				description: task.description,
				finished: task.finished
			})

			i += 1
		}
	}

	new_tasks = append(new_tasks, new_task)

	fs.write_file(config.db_path, conv.string(new_tasks)) |> error.handle_error()

	return "Task status was successfully changed."
}

fn handle_help(parsed_cmd) {
	if len(parsed_cmd.args) > 0 {
		if strings.lower(parsed_cmd.args[0]) == "add" {
			return "Add a new task to the list.\n"
		}

		if strings.lower(parsed_cmd.args[0]) == "help" {
			return "Show this help message.\n"
		}		

		if strings.lower(parsed_cmd.args[0]) == "add" {
			return "Show all currently unfinished tasks.\n"
		}

		if strings.lower(parsed_cmd.args[0]) == "del" {
			return "Delete a task from the list.\n"
		}

		if strings.lower(parsed_cmd.args[0]) == "toggle" {
			return "Toggle the status of a task.\n"
		}

		if strings.lower(parsed_cmd.args[0]) == "quit" || strings.lower(parsed_cmd.args[0]) == "exit" {
			return "Exit the application.\n"
		}

		return "Unknown command.\n"
	}

	return
		"List of available commands:\n\n" +
		"add\n" +
		"help\n" +
		"list\n" +
		"del\n" +
		"toggle\n"
}

fn handle_cmd(cmd, config) {
	parsed_cmd = parse_cmd(cmd)

	if parsed_cmd.error {
		return parsed_cmd.value
	}

	cmd_name = parsed_cmd.value.name

	if strings.lower(cmd_name) == "add" { return handle_add(parsed_cmd.value, config) }
	if strings.lower(cmd_name) == "help" { return handle_help(parsed_cmd.value) }
	if strings.lower(cmd_name) == "list" { return handle_list(parsed_cmd.value, config) }
	if strings.lower(cmd_name) == "del" { return handle_del(parsed_cmd.value, config) }
	if strings.lower(cmd_name) == "toggle" { return handle_toggle(parsed_cmd.value, config) }
}
