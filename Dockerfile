FROM rust:latest AS builder

RUN update-ca-certificates

# Create appuser
ENV USER=bill-file-analyzer-service
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /bill-file-analyzer-service

COPY ./ .

ENV SQLX_OFFLINE true
RUN cargo build --release

######################
FROM ubuntu:latest as bill-file-analyzer-service

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /bill-file-analyzer-service

# Copy our build
COPY --from=builder /bill-file-analyzer-service/target/release/bill-file-analyzer-service ./
COPY --from=builder /bill-file-analyzer-service/configuration ./configuration

# Use an unprivileged user.
USER bill-file-analyzer-service:bill-file-analyzer-service

EXPOSE 8000
ENV APP_ENVIRONMENT production

CMD ["/bill-file-analyzer-service/bill-file-analyzer-service"]
