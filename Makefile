server_1:
		RUST_LOG=info cargo watch -x 'run --bin server1'

server_2:
		RUST_LOG=info cargo watch -x 'run --bin server2'

server_3:
		cd server3; mix phx.server

superschema:
		rover supergraph compose --config supergraph.yaml > schema.graphql

run: superschema
		./dist/router --supergraph schema.graphql --config configuration.yaml
