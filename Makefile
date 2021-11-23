server_quotes:
		RUST_LOG=info cargo watch -x 'run --bin server1'

server_brokers:
		RUST_LOG=info cargo watch -x 'run --bin server2'

server_claims:
		cd server3; mix phx.server

servers:
		RUST_LOG=info cargo watch -x 'run --bin server1' & \
		RUST_LOG=info cargo watch -x 'run --bin server2' & \
		cd server3; mix phx.server

superschema:
		rover supergraph compose --config supergraph.yaml > schema.graphql

run: superschema
		./dist/router --supergraph schema.graphql --config configuration.yaml

print_schema_1:
		mkdir -p schemas; rover subgraph introspect http://localhost:8080/graphql > schemas/schema_1.graphql

print_schema_2:
		mkdir -p schemas; rover subgraph introspect http://localhost:8081/graphql > schemas/schema_2.graphql

print_schema_3:
		mkdir -p schemas; rover subgraph introspect http://localhost:8082/graphql > schemas/schema_3.graphql

print_schemas: print_schema_1 print_schema_2 print_schema_3
