FROM rust:1-alpine3.14

EXPOSE 8000

RUN apk add --no-cache gcc musl-dev linux-headers

WORKDIR /code
COPY . .

CMD ["cargo", "run"]
