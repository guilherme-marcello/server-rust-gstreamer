use gstreamer as gst;
use gst::prelude::*;

fn main() {
    gst::init().unwrap();
    println!("TEKEVER <June 2022> Video pipeline test producer");

    let rtp_uri: &str = "127.0.0.1";

    // create empty pipeline
    let pipeline = gst::Pipeline::new(Some("pipeline"));

    let videosrc = gst::ElementFactory::make("videotestsrc", Some("source"))
        .expect("Could not create src");

    let encoder = gst::ElementFactory::make("x264enc", Some("encoder"))
        .expect("Could not create encoder");

    let pay = gst::ElementFactory::make("rtph264pay", Some("pay"))
        .expect("Could not create pay");

    let udpsink = gst::ElementFactory::make("udpsink", Some("udp"))
        .expect("Could not create udp sink");

    // set properties

    videosrc.set_property_from_str("is-live", "1");
    udpsink.set_property_from_str("host", rtp_uri);
    udpsink.set_property_from_str("port", "50000");

    let gst_elements = &[&videosrc, &encoder, &pay, &udpsink];

    // add elements to pipeline
    pipeline.add_many(gst_elements).unwrap();

    // link elements
    gst::Element::link_many(gst_elements)
        .expect("Elements could not be linked");

    // start playing pipeline
    pipeline.set_state(gst::State::Playing)
        .expect("Could not change state from NULL to Playing state");

    // handle EOS or error
    let bus = pipeline
    .bus().unwrap();
    //.expect("Pipeline without bus. Should not happen!");

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Error(err) => {
                println!("Error received from element {:?} {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                break;
            }
            MessageView::StateChanged(state) => {
                if state.src().map(|s| s == pipeline).unwrap_or(false) {
                    println!("Pipeline state changed from {:?} to {:?}!",
                        state.old(),
                        state.current()
                    );
                }
            }
            MessageView::Eos(_) => break,
            _ => (),
        }
    }



}

