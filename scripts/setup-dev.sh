#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ASSETS="$ROOT/assets"
FC_VERSION="${FC_VERSION:-1.8.0}"
ARCH="$(uname -m)"

mkdir -p "$ASSETS" "$ROOT/var/quick-os/agents" "$ROOT/var/quick-os/snapshots"

echo "==> quick-os dev setup"
echo "root: $ROOT"

if [[ ! -e /dev/kvm ]]; then
  echo "WARN: /dev/kvm missing — Firecracker needs KVM (bare metal or nested virt + privileged container)"
else
  echo "ok: /dev/kvm"
fi

FC_URL="https://github.com/firecracker-microvm/firecracker/releases/download/v${FC_VERSION}/firecracker-v${FC_VERSION}-${ARCH}.tgz"
FC_TGZ="$ASSETS/firecracker.tgz"

if [[ ! -x /usr/local/bin/firecracker ]]; then
  echo "==> downloading firecracker v${FC_VERSION} (${ARCH})"
  curl -fsSL "$FC_URL" -o "$FC_TGZ"
  tar -xzf "$FC_TGZ" -C "$ASSETS"
  install -m 755 "$ASSETS/release-v${FC_VERSION}-$(uname -m)/firecracker-v${FC_VERSION}-${ARCH}" /usr/local/bin/firecracker
  echo "installed: /usr/local/bin/firecracker"
else
  echo "ok: /usr/local/bin/firecracker"
fi

if [[ ! -f "$ASSETS/vmlinux" ]]; then
  echo "==> downloading guest kernel (Alpine virt)"
  curl -fsSL "https://dl-cdn.alpinelinux.org/alpine/v3.20/releases/${ARCH}/netboot/vmlinuz-virt" -o "$ASSETS/vmlinux"
  echo "ok: assets/vmlinux"
else
  echo "ok: assets/vmlinux"
fi

if [[ ! -f "$ASSETS/rootfs.ext4" ]]; then
  echo "==> building minimal alpine rootfs (~256M)"
  ROOTFS="$ASSETS/rootfs.ext4"
  truncate -s 256M "$ROOTFS"
  mkfs.ext4 -F "$ROOTFS" >/dev/null
  MNT="$(mktemp -d)"
  sudo mount "$ROOTFS" "$MNT"
  sudo apk add --root "$MNT" --initdb --no-cache --arch "$ARCH" alpine-base openssh >/dev/null
  echo "quick-os" | sudo tee "$MNT/etc/hostname" >/dev/null
  sudo umount "$MNT"
  rmdir "$MNT"
  echo "ok: assets/rootfs.ext4"
else
  echo "ok: assets/rootfs.ext4"
fi

echo
echo "Setup complete. Next:"
echo "  cargo build"
echo "  cargo run -p quick-os -- check-env"
echo "  cargo run -p quick-os -- snapshot-create --id base"
echo "  cargo run -p quick-os -- serve"
