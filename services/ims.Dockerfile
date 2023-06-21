FROM rust as build

COPY Cargo.toml ./Cargo.toml


RUN cargo build --release

RUN rm ./target/release/deps/ims*
RUN cargo build --release


FROM rust:slim
COPY --from=build /ims/target/release/ims .

CMD ["./target/release/ims"]

# love you 3000 <3