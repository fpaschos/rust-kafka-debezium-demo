#### Local installation / Usage:
Optional step for dev, because of local .env file
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

#### Initial setup/register schemas and connectors

After successful start up (or after any tear down)

Register schemas to schema registry
```bash
./scripts/publish-claims-schemas.sh

```

Register claims outbox postgres connector
```bash
./scripts/register-claims-outbox-connector.sh
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

Open kafka ui:
```bash
open http://localhost:58000
```

Open debezium ui:
```bash
open http://localhost:58001
```