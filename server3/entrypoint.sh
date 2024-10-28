#!/bin/bash
# Docker entrypoint script.

if [ -n "$1" ]; then
  sh -c "$@"
else
  exec mix phx.server
fi
