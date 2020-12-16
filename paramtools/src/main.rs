use serde_json::{Result, Value};

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "title": "My parameter",
            "description": "tell me more",
            "type": "int",
            "value": 2
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("{} {} {} {}", v["title"], v["description"], v["type"], v["value"]);

    Ok(())
}

fn main() {
    match untyped_example() {
        Err(e) => println!("error {}", e),
        _ => ()
    }
}