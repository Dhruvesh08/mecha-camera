use anyhow::Error;
use gstreamer::prelude::*;

fn main() -> Result<(), Error> {
    // Initialize GStreamer
    gstreamer::init()?;

    // Create the elements
    let source = gstreamer::ElementFactory::make("v4l2src").build()?;

    let converter = gstreamer::ElementFactory::make("jpegenc").build()?;
    // let encoder = gstreamer::ElementFactory::make("vpuenc_h264").build()?;
    let filesink = gstreamer::ElementFactory::make("filesink")
        .property("location", "capture_Image.jpg")
        .build()?;
    // Create the empty pipeline
    let pipeline = gstreamer::Pipeline::new(Some("test-pipeline"));

    // Add elements to the pipeline
    pipeline.add_many(&[&source, &converter, &filesink])?;

    // Link the elements
    gstreamer::Element::link_many(&[&source, &converter, &filesink])?;

    // Start recording
    pipeline.set_state(gstreamer::State::Playing)?;

    // Wait for the pipeline to be ready
    let bus = pipeline.bus().expect("Pipeline has no bus");
    let msg = bus.timed_pop_filtered(
        gstreamer::ClockTime::NONE,
        &[gstreamer::MessageType::Eos, gstreamer::MessageType::Error],
    );
    match msg {
        Some(msg) => match msg.view() {
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
        },
        None => {
            eprintln!("Failed to receive message from the bus");
        }
    }

    // Stop the pipeline
    pipeline.set_state(gstreamer::State::Null)?;

    Ok(())
}
