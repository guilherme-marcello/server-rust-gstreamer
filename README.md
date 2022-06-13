# server-rust-gstreamer-rtp

Example of usage of Gstreamer bindings for Rust



## Authors

- Guilherme Marcello \<guilemosmarcello@gmail.com\> - [@guilherme-marcello](https://github.com/guilherme-marcello)


## Run Locally
Install Rust toolchain manager (optional)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the project

```bash
  git clone https://github.com/guilherme-marcello/server-rust-gstreamer-rtp
```

Go to the project directory

```bash
  cd server-rust-gstreamer-rtp
```

Build project

```bash
  cargo build --release
```

Start the server

```bash
  ./target/release/server-rust-gstreamer-rtp
```


## Deployment with Docker

// TODO





## Documentation

new(), start() and build() are the default desired "signatures" for our pipeline usage inside a Gstreamer application.

* new() →  create a pipeline without any elements in the bin

* build() → create all the necessary Gst elements, links them together and adds them to the bin (Using ENV variables).

* start() → change the pipeline state to PLAYING.

So a trait was created for this purpose, named "BasicPipeline".
``` rust
// ./src/utils/ingestor.rs
pub trait BasicPipeline {
    fn new() -> Self;
    fn start(&self);
    fn build(&self);
}
```
The main advantage of this approach for pipeline design is that we can call the same methods on all of the defined pipelines. 

This way, whenever we want to add a new pipeline in our project, for example, we create a struct named 'NewPipeline' and do the implementation of the BasicPipeline trait on the 'NewPipeline' struct:
``` rust
// ./src/utils/ingestor.rs
pub struct NewPipeline {
    pipeline: Pipeline
}

impl BasicPipeline for NewPipeline {
    fn new() -> Self {
      ...
    }

    fn build(&self) {
      ...
    }

    fn start(&self) {
      ...
    }
}
```





## Feedback

If you have any feedback, please reach out to me at guilemosmarcello@gmail.com
## License

MIT
