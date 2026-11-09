FROM golang:1.23-alpine AS builder
WORKDIR /src
COPY go.mod ./
COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -trimpath -ldflags="-s -w" -o /out/hello-go .

FROM scratch
COPY --from=builder /out/hello-go /hello-go
ENTRYPOINT ["/hello-go"]
