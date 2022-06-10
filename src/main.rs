#[path = "utils/ingestor.rs"] mod ingestor;
use crate::ingestor::BasicPipeline;

fn ingestor_cycle() {
    println!("[*] Creating elements, packing elements together in a pipeline...");
    let test_pipeline = ingestor::VideoTestPipeline::new();
    test_pipeline.build();
    println!("[+] Starting!");
    test_pipeline.start();
}

fn main() {
    ingestor_cycle();   
}
