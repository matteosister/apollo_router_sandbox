server_1:
		RUST_LOG=info cargo watch -x 'run --bin server1'

server_2:
		RUST_LOG=info cargo watch -x 'run --bin server2'

server_3:
		cd server3; mix deps.get; mix phx.server

run:
		npx -y wgc router compose -i router/federation.yaml > router.json
		./router
