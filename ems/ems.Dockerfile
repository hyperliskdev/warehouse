FROM rust

COPY ems /bin/ems

CMD ["/bin/ems"]