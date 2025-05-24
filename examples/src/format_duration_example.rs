#[cfg(feature = "cli")]
use rcpdaemon::cli::utils;

fn main() {
    // Test different duration values
    #[cfg(feature = "cli")]
    let seconds = utils::format_duration(30);
    #[cfg(feature = "cli")]
    let minutes = utils::format_duration(150);
    #[cfg(feature = "cli")]
    let hours = utils::format_duration(3600);
    #[cfg(feature = "cli")]
    let days = utils::format_duration(86400 * 2);

    #[cfg(feature = "cli")]
    {
        println!("30 seconds: {}", seconds);
        println!("150 seconds: {}", minutes);
        println!("3600 seconds: {}", hours);
        println!("172800 seconds: {}", days);
    }

    #[cfg(not(feature = "cli"))]
    {
        println!("CLI feature is not enabled. Enable it with --features cli");
    }
}
