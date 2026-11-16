mod notification;

use notification::Notification;

fn main() {
    // Leer entradas
    let mut alert_message = String::new();
    std::io::stdin()
        .read_line(&mut alert_message)
        .expect("Failed to read line");
    let alert_message = alert_message.trim().to_string();

    let mut reminder_title = String::new();
    std::io::stdin()
        .read_line(&mut reminder_title)
        .expect("Failed to read line");
    let reminder_title = reminder_title.trim().to_string();

    let mut minutes_input = String::new();
    std::io::stdin()
        .read_line(&mut minutes_input)
        .expect("Failed to read line");
    let reminder_minutes: u32 = minutes_input
        .trim()
        .parse()
        .expect("Failed to parse minutes");

    // TODO: Crea una notificación Alert usando el alert_message
    let alert = Notification::Alert(alert_message);
    // TODO: Create a Reminder notification using reminder_title and reminder_minutes
    let reminder = Notification::Reminder {
        title: reminder_title,
        minutes: reminder_minutes,
    };
    // TODO: Crea una notificación Dismiss
    let dismiss = Notification::Dismiss;
    // TODO: Imprime cada notificación usando el formato de depuración {:?}
    // Cada notificación debe imprimirse en su propia línea
    println!("{:?}", alert);
    println!("{:?}", reminder);
    println!("{:?}", dismiss);
}
