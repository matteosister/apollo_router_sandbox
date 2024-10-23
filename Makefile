server_1:
		RUST_LOG=info cargo watch -x 'run --bin server1'

server_2:
		RUST_LOG=info cargo watch -x 'run --bin server2'

server_3:
		cd server3; mix phx.server

run:
		npx wgc router compose -i compose.yaml > router.json
		./router
