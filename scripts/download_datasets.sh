#!/usr/bin/env bash
# Download the public datasets kenyon's reproduction uses into $KENYON_DATA
# (default ~/datasets). No credentials. Total ~120 MB. Checksums are pinned
# in scripts/datasets.sha256 and verified after download.
set -euo pipefail
DATA="${KENYON_DATA:-$HOME/datasets}"
HERE="$(cd "$(dirname "$0")" && pwd)"
mkdir -p "$DATA"/{mnist,fashion-mnist,kmnist,siftsmall,door}

fetch() { # url dest
  if [ ! -s "$2" ]; then
    echo "fetch $1"
    curl -fsSL --retry 3 --max-time 600 -o "$2.part" "$1" && mv "$2.part" "$2"
  fi
}

IDX="train-images-idx3-ubyte train-labels-idx1-ubyte t10k-images-idx3-ubyte t10k-labels-idx1-ubyte"
for f in $IDX; do
  fetch "https://ossci-datasets.s3.amazonaws.com/mnist/$f.gz" "$DATA/mnist/$f.gz"
  fetch "http://fashion-mnist.s3-website.eu-central-1.amazonaws.com/$f.gz" "$DATA/fashion-mnist/$f.gz"
  fetch "https://codh.rois.ac.jp/kmnist/dataset/kmnist/$f.gz" "$DATA/kmnist/$f.gz"
done

# ANN_SIFT10K (Jegou et al. 2011), the "siftsmall" corpus.
fetch "ftp://ftp.irisa.fr/local/texmex/corpus/siftsmall.tar.gz" "$DATA/siftsmall/siftsmall.tar.gz"
if [ ! -s "$DATA/siftsmall/siftsmall/siftsmall_base.fvecs" ]; then
  tar -xzf "$DATA/siftsmall/siftsmall.tar.gz" -C "$DATA/siftsmall"
fi

# DoOR 2.0 (Muench & Galizia 2016), CC BY-SA 4.0, pinned commit. Only the
# Hallem & Carlson 2006 receptors are used (column Hallem.2006.EN).
DOOR_SHA=db323a496577c4b4a72b5c2fcd1859e07521ffb5
for r in Or2a Or7a Or9a Or10a Or19a Or22a Or23a Or33b Or35a Or43a Or43b Or47a Or47b \
         Or49b Or59b Or65a Or67a Or67c Or82a Or85a Or85b Or85f Or88a Or98a; do
  fetch "https://raw.githubusercontent.com/ropensci/DoOR.data/$DOOR_SHA/data/$r.csv" "$DATA/door/$r.csv"
done

cd "$DATA"
if [ "${KENYON_WRITE_SUMS:-0}" = 1 ]; then
  find mnist fashion-mnist kmnist siftsmall door -type f \( -name '*.gz' -o -name '*.fvecs' -o -name '*.ivecs' -o -name '*.csv' \) \
    | sort | xargs sha256sum > "$HERE/datasets.sha256"
  echo "wrote $HERE/datasets.sha256"
else
  sha256sum -c --quiet "$HERE/datasets.sha256"
  echo "datasets OK in $DATA"
fi
