use oxide::App;

fn main() {
    println!("Hello, world!");
    let app = App::new("Oxide Playground", 800, 600);
    app.run().expect("Event loop error.");
}
