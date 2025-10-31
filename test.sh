#!/bin/sh

set -eu

MIMALLOC_VERSION=$1
USE_SECURE=${2:-OFF}

if [[$USE_SECURE != "secure" || $USE_SECURE != "SECURE"]]; then
  USE_SECURE=OFF
  LIBMIMALLOC_A=libmimalloc.a
else
  LIBMIMALLOC_A=libmimalloc-secure.a
fi


apk add --no-cache \
  patch \
  cmake \
  ninja-is-really-ninja

curl -f -L --retry 5 https://github.com/microsoft/mimalloc/archive/refs/tags/v$MIMALLOC_VERSION.tar.gz | tar xz

cd mimalloc-$MIMALLOC_VERSION

# patch -p1 < ../mimalloc.diff

CFLAGS=${CFLAGS/-O3} cmake \
  -Bout \
  -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_C_COMPILER=clang \
  -DCMAKE_INSTALL_PREFIX=/usr \
  -DMI_SECURE=$USE_SECURE \
  -DMI_NO_OPT_ARCH=ON \
  -DMI_INSTALL_TOPLEVEL=ON \
  -DMI_BUILD_OBJECT=ON \
  -DMI_BUILD_TESTS=OFF \
  -DMI_LIBC_MUSL=ON \
  -DMI_SKIP_COLLECT_ON_EXIT=ON \
  -G Ninja \
  .

cmake --build out --target install -- -v

# for libc_path in $(find /usr -name libc.a); do
#   {
#     echo "CREATE libc.a"
#     echo "ADDLIB $libc_path"
#     echo "DELETE aligned_alloc.lo calloc.lo donate.lo free.lo libc_calloc.lo lite_malloc.lo malloc.lo malloc_usable_size.lo memalign.lo posix_memalign.lo realloc.lo reallocarray.lo valloc.lo"
#     echo "ADDLIB out/$LIBMIMALLOC_A"
#     echo "SAVE"
#   } | ar -M
#   mv libc.a $libc_path
# done