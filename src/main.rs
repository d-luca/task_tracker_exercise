enum Status {
    Todo,
    InProgress,
    Done,
}

struct Task {
    id: u32,
    description: String,
    status: Status,
    created_at: u64,
    updated_at: u64,
}

impl Task {
    fn new(id: u32, description: String, now: u64) -> Self {
        Task {
            id,
            status: Status::Todo,
            description,
            created_at: now,
            updated_at: now,
        }
    }
}

fn main() {
    println!("Hello, world!");
    // Load JSON file
    // Define next id as max id + 1

    // execute side effect of the declared user action
}
