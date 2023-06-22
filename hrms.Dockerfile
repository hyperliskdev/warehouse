FROM rust as build

WORKDIR /hrms

COPY . .

RUN cargo build --bin hrms --release


FROM rust:slim
COPY --from=build /hrms/target/release/hrms .

CMD ["./hrms"]