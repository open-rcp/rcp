use rcp_cli::utils;

fn main() {
    // Test different duration values
    let seconds = utils::format_duration(30);
    let minutes = utils::format_duration(150);
    let hours = utils::format_duration(3600);
    let days = utils::format_duration(86400 * 2);

    println!("30 seconds: {}", seconds);
    println!("150 seconds: {}", minutes);
    println!("3600 seconds: {}", hours);
    println!("172800 seconds: {}", days);
}
