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