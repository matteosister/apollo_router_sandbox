superschema:
		rover supergraph compose --config supergraph.yaml > schema.graphql

run: superschema
		./dist/router --supergraph schema.graphql --config configuration.yaml