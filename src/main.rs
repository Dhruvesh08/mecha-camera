use anyhow::Error;
use gstreamer::prelude::*;
use std::io::Write;

fn main() -> Result<(), Error> {
    // Initialize GStreamer
    gstreamer::init()?;

    // Build the pipeline
    let pipeline = gstreamer::parse_launch("v4l2src ! videoconvert ! x264enc ! mp4mux ! filesink location=captured_video.mp4")?;

    // Start recording
    let _ = pipeline.set_state(gstreamer::State::Playing)?;

    // Wait for the pipeline to be ready
    let bus = pipeline.bus().expect("Pipeline has no bus");
    let msg = bus.timed_pop_filtered(gstreamer::CLOCK_TIME_NONE, &[gstreamer::MessageType::EOS, gstreamer::MessageType::Error]);
    match msg {
        Some(msg) => {
            match msg.view() {
                gstreamer::MessageView::Error(err) => {
                    eprintln!(
                        "Error from {:?}: {} ({:?})",
                        err.src().map(|s| s.path_string()),
                        err.error(),
                        err.debug()
                    );
                }
                gstreamer::MessageView::Eos(..) => {
                    println!("Recording finished");
                }
                _ => unreachable!(),
            }
        }
        None => {
            eprintln!("Failed to receive message from the bus");
        }
    }

    // Stop the pipeline
    let _ = pipeline.set_state(gstreamer::State::Null)?;

    Ok(())
}
