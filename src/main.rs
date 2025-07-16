mod components;
mod git;
mod models;
mod utils;

use components::app;

fn main() {
    println!("Starting Claude Scheduler...");
    println!("Initializing Dioxus application...");
    println!("Open http://localhost:8080 in your browser to access the application.");

    dioxus::launch(app);

    println!("Dioxus application ended.");
}
