FROM ethcore/rust:nightly 
EXPOSE 8080
COPY Cargo.toml /build/
COPY src/main.rs /build/src/
COPY static/index.html /build/static/
COPY static/main.css /build/static/
CMD cargo build --release
CMD cargo run
