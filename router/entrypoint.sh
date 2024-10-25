#!/bin/bash
# Docker entrypoint script.

if [ -n "$1" ]; then
  sh -c "$@"
else
  exec npx wgc router compose -i federation.yaml > router.json
fi
