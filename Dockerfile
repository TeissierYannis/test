# Utilisez une image basée sur Debian ou Ubuntu
FROM ubuntu:20.04

# Mettez à jour les paquets et installez les outils nécessaires
RUN apt-get update && \
    apt-get install -y software-properties-common && \
    add-apt-repository -y ppa:ubuntu-toolchain-r/test && \
    apt-get update && \
    apt-get install -y clang llvm libelf-dev gcc make curl

# Installer bpftool séparément en le téléchargeant depuis le dépôt des sources du noyau Linux
RUN apt-get install -y linux-headers-$(uname -r) && \
    cd /tmp && \
    curl -LO https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-5.10.tar.xz && \
    tar -xf linux-5.10.tar.xz && \
    cd linux-5.10 && \
    make defconfig && \
    make -C tools/bpf/bpftool && \
    cp tools/bpf/bpftool/bpftool /usr/local/bin/ && \
    rm -rf /tmp/linux-5.10

# Créez un répertoire de travail
WORKDIR /workspace

# Commande par défaut pour rester dans le conteneur après le démarrage
CMD ["/bin/bash"]
