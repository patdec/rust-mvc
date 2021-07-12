#!/bin/bash

podman pod create --name rust-mvc -p 5432:5432
podman unshare chown 1000:1000 -R /home/patrice/Working/docker/db/rust_mvc
podman run -d --restart=always --pod=rust-mvc -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres \
  -e postgres_DB=rust_mvc -v /home/patrice/Working/docker/db/rust_mvc:/var/lib/postgresql/data:Z \
  --name=rust-mvc-db postgres:12.5