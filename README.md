https://aya-rs.dev/book/
https://tokio.rs/tokio/tutorial/setup https://github.com/tokio-rs/tokio


# Créez l’image Docker
docker build -t ebpf-build-env .

# Lancez le conteneur
docker run --rm -it -v "$(pwd):/workspace" ebpf-build-env

# Dans le conteneur : générer vmlinux.h
bpftool btf dump file /sys/kernel/btf/vmlinux format c > /workspace/vmlinux.h

# Dans le conteneur : compiler xdp.c en xdp.o
clang -O2 -g -target bpf -c /workspace/xdp.c -o /workspace/xdp.o
