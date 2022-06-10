use gstreamer as gst;
use gst::{prelude::*, Pipeline};

pub trait BasicPipeline {
    fn new() -> Self;
    fn start(&self);
    fn build(&self);
}

pub struct VideoTestPipeline {
    pipeline: Pipeline
}

impl VideoTestPipeline {
    pub fn _build(&self, target_host: &str, target_port: &str) {
        let videosrc = gst::ElementFactory::make("videotestsrc", Some("source"))
        .expect("Could not create videotestsrc");

        let encoder = gst::ElementFactory::make("x264enc", Some("encoder"))
            .expect("Could not create x264enc");

        let pay = gst::ElementFactory::make("rtph264pay", Some("pay"))
            .expect("Could not create rtph264pay");

        let udpsink = gst::ElementFactory::make("udpsink", Some("udp"))
            .expect("Could not create udpsink");

        // set properties
        videosrc.set_property_from_str("is-live", "1");
        udpsink.set_property_from_str("host", &target_host);
        udpsink.set_property_from_str("port", &target_port);

        let gst_elements = &[&videosrc, &encoder, &pay, &udpsink];

        // add elements to bin
        self.pipeline.add_many(gst_elements).unwrap();

        // link elements
        gst::Element::link_many(gst_elements)
            .expect("Elements could not be linked");
    }
}

impl BasicPipeline for VideoTestPipeline {
    fn new() -> Self { 
        gst::init().unwrap();
        println!("Video pipeline test producer");    
        Self { 
            pipeline: gst::Pipeline::new(Some("pipeline"))
        } 
    }

    fn build(&self) {
        self._build("127.0.0.1", "50000");
    }

    fn start(&self) {
        // start playing pipeline
        self.pipeline.set_state(gst::State::Playing)
            .expect("Could not change state from NULL to Playing state");

        // handle EOS or error
        let bus = self.pipeline
        .bus().unwrap();

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
                    if state.src().map(|s| s == self.pipeline).unwrap_or(false) {
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
}