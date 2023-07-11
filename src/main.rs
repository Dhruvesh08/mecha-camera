use std::process::Command;

fn capture_image(output_file: &str) {
    // Build the command
    let command = Command::new("gst-launch-1.0")
        .args(&[
            "-v",
            "v4l2src",
            &format!("device=/dev/video0"),
            "num-buffers=1",
            "!",
            "jpegenc",
            "!",
            "filesink",
            &format!("location={}", output_file),
        ])
        .output()
        .expect("Failed to execute command");

    if command.status.success() {
        println!("Image captured and saved to '{}'", output_file);
    } else {
        println!("Failed to capture the image");
    }
}

fn main() {
    // // Parse command-line arguments
    // let args: Vec<String> = std::env::args().collect();
    // if args.len() != 2 {
    //     eprintln!("Usage: {} <output_file>", args[0]);
    //     std::process::exit(1);
    // }
    // let output_file = &args[1];
    let output_file = "test.jpg";

    // Capture the image
    capture_image(output_file);
}
