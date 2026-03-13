```bash
podman build -t patdec/aarch64:last cross
cross build --release --target=aarch64-unknown-linux-gnu
    