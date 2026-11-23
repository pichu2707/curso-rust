mod profile;

use profile::Profile;

fn main() {
    // Lee las entradas
    let mut input1 = String::new();
    std::io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");
    let username1 = input1.trim().to_string();

    let mut input2 = String::new();
    std::io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");
    let nickname1 = input2.trim().to_string();

    let mut input3 = String::new();
    std::io::stdin()
        .read_line(&mut input3)
        .expect("Failed to read line");
    let username2 = input3.trim().to_string();

    let profile1 = Profile::new(username1, Some(nickname1));

    let profile2 = Profile::new(username2, None);

    println!("Display name: {}", profile1.display_name());
    println!("Formatted nickname: {:?}", profile1.formatted_nickname());
    println!("Display name: {}", profile2.display_name());
    println!("Formatted nickname: {:?}", profile2.formatted_nickname());
}
