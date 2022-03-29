FROM rust:1-alpine3.14

EXPOSE 8000

RUN apk add --no-cache gcc musl-dev linux-headers
RUN apk add git
RUN git clone https://github.com/maxfer1221/dummy-server

WORKDIR /dummy-server

CMD ["cargo", "run"]
