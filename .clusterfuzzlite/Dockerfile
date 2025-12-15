# SPDX-License-Identifier: AGPL-3.0-or-later
# ClusterFuzzLite build environment for czech-file-knife
FROM gcr.io/oss-fuzz-base/base-builder-rust

COPY . $SRC/czech-file-knife
WORKDIR $SRC/czech-file-knife

COPY .clusterfuzzlite/build.sh $SRC/
