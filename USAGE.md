#### Instructions/ Usage:
Export debezium version (execute once per open terminal session)
```bash
export DEBEZIUM_VERSION=2.3
```

Start up
```bash
docker compose up -d
```

Tear down
```bash 
docker compose down
```

#### Utility actions:
Check everything up and running
```bash
docker ps
```

List all available kafka topics
```bash
docker-compose exec kafka /kafka/bin/kafka-topics.sh --bootstrap-server kafka:9092 --list
```

#### Documentation/Resources
For more information see:
- [debezium/kafka](https://hub.docker.com/r/debezium/kafka)

Reference projects:
- [cschaible/rust-microservices-kafka](https://github.com/cschaible/rust-microservices-kafka/blob/master/README.md)
- [debezium-examples/distributed-caching](https://github.com/debezium/debezium-examples/blob/main/distributed-caching/README.md)

Rust libraries:
- [sqlx](https://github.com/launchbadge/sqlx/blob/main/README.md)