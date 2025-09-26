use std::io;

fn main() {
    let mut tasks:Vec<String> = Vec::new();

    loop {
        println!("1. Add task(+)\n2. View tasks(v)\n3. Remove task(r)\n4. Exit(e)");

        // Take input from user
        println!("Enter your choice: ");
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Error: Failed to take user input");

        let user_input: u16 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match user_input {
            1 => add_task(&mut tasks),
            2 => view_task(&tasks),
            3 => remove_tasks(&mut tasks),
            4 => { println!("Exiting...");
                break; 
            }
            _ => println!("Please Enter valid input!"),

        }

    }
}


fn add_task(tasks: &mut Vec<String>) {
    println!("Enter task: ");
    let mut task = String::new();
    io::stdin().read_line(&mut task).expect("Error reading input");
    tasks.push(task.trim().to_string());
    println!("Task added successfully!");
}

fn view_task(tasks: &Vec<String>) {
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        for (i, task) in tasks.iter().enumerate() {
            println!("{}: {}", i + 1, task);
        }
    }
}

fn remove_tasks(tasks: &mut Vec<String>) {
    tasks.clear();
    println!("All tasks have been removed.");
}
