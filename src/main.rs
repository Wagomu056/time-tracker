use time_tracker_core::TimeTracer;

fn main() {
    println!("Task Tracker");
    let mut tracker = TimeTracer::new();

    loop {
        println!("Input task name: ");
        let mut task_name = String::new();
        std::io::stdin().read_line(&mut task_name).unwrap();

        // if input is 'clear', delete the save files
        if task_name.trim() == "clear" {
            tracker.delete_save_files().unwrap();
            println!("Save files deleted");
            continue;
        }

        println!("Running task: {}", task_name);
        let id = tracker.new_task(task_name.trim());
        tracker.start_task(id);

        println!("Press enter to stop the task...");
        std::io::stdin().read_line(&mut String::new()).unwrap();
        tracker.end_task(id);
    }
}
