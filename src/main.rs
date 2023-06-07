use gstreamer::prelude::*;
use gstreamer::Element;
use gstreamer::ElementFactory;
use gstreamer::Message;
use gstreamer::MessageView;
use gstreamer::Pipeline;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize GStreamer
    gstreamer::init()?;

    // Create a pipeline
    let pipeline = Pipeline::new(None);

    // Create a v4l2src element
    let src = ElementFactory::make("v4l2src").build().unwrap();

    // Create a jpegenc element
    let enc = ElementFactory::make("jpegenc").build().unwrap();

    // Create a filesink element
    let sink = ElementFactory::make("filesink").build().unwrap();



    // Get an instance of the sink element
    let sink = sink.clone().dynamic_cast::<Element>().unwrap();

    // Set the output file name
    sink.set_property("location", &"test_using_gst.jpg");

    // Add elements to the pipeline
    pipeline.add_many(&[&src, &enc, &sink]).unwrap();

    // Link the elements together
    src.link(&enc).unwrap();
    enc.link(&sink).unwrap();

    // Start the pipeline
    pipeline.set_state(gstreamer::State::Playing)?;

    // Wait for EOS or error message
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter() {
        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!("Error from {:?}:", err);
                break;
            }
            _ => (),
        }
    }

    // Stop the pipeline
    pipeline.set_state(gstreamer::State::Null)?;

    Ok(())
}
