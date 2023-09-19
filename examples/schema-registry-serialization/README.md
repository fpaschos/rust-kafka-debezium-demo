### schema-registry-serialization
Example of using schema registry in order to serialize and deserialize `claims-schema` messages.

A running confluent schema registry instance with claims-schema definitions registered is required.

Run with
```
docker compose up -d
cargo run -p claims-schema-register
cargo run -p schema-registry-serialization
```