# load-balancer-rs
A load balancing implementation similar to [Nucleon](https://github.com/NicolasLM/nucleon).

## Usage:

### Container Setup (Testing)

##### Startup
`docker-compose up -d --scale backend=3`

 Creates 5 containers:
```
1     load balancer (robin)    image: rust:1-alpine3.14
2     redis cache              image: redis:alpine
3-5   test backend server      image: rust:1-alpine3.14
```

##### Test
Head to `localhost:8000`

##### Cleanup
`docker-compose kill && docker-compose down`

### Balancer Only
```
Usage:
    load-balancer-rs [OPTIONS]

Dynamic HTTP load balancer

optional arguments:
  -h,--help             show this help message and exit
  -b,--bind BIND        Bind the load balancer to address:port (127.0.0.1:8000)
  -r,--redis REDIS      URL of Redis database (redis://localhost)
  -m,--method           Balancing method [robin, random] (robin)
```

##### Add/Remove IP to/from Server List

```
# Add IP
redis:6379> PUBLISH backend_add 127.0.0.1:8081

# Remove IP
redis:6379> PUBLISH backend_remove 127.0.0.1:8082
```
