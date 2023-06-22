FROM rust as build

WORKDIR /pms

COPY . .

RUN cargo build --bin pms --release


FROM rust:slim
COPY --from=build /pms/target/release/pms .

CMD ["./pms"]

# love you 3000 <3