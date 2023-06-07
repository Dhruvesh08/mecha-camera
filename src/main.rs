use gstreamer::prelude::*;

use std::error::Error;

//improve this code as gst use gstreamer::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize GStreamer
    gstreamer::init().unwrap();

    let output_file = "test_gst_v1.jpg";

    // Create the elements
    let source = gstreamer::ElementFactory::make("v4l2src")
        .build()
        .expect("Could not create source element.");
    
    let enc = gstreamer::ElementFactory::make("jpegenc")
        .build()
        .expect("Could not create sink element");

    let sink = gstreamer::ElementFactory::make("filesink")
        .property("location", &output_file)
        .build()?;

    // Create the empty pipeline
    let pipeline = gstreamer::Pipeline::new(Some("capture-image"));

    // Build the pipeline
    pipeline.add_many(&[&source, &enc, &sink]).unwrap();
    source.link(&sink).expect("Elements could not be linked.");

    // Start playing
    pipeline
        .set_state(gstreamer::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    // Wait until error or EOS
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        use gstreamer::MessageView;

        match msg.view() {
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?}: {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                eprintln!("Debugging information: {:?}", err.debug());
                break;
            }
            MessageView::Eos(..) => break,
            _ => (),
        }
    }

    pipeline
        .set_state(gstreamer::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");

    Ok(())
}
