#/bin/sh

export CXX="/usr/local/bin/g++-8"
export CXXFLAGS="${CXXFLAGS} -std=c++1z -fsanitize=address"

if [ $# -lt 1 ]; then
    echo "usage: ${0} <directory>"
    exit 1
fi

if [ ! -d $1 ]; then
    echo "directory '${1}' does not exist."
    exit 1
fi

DIR=$(cd $1 && pwd)
${CXX} ${CXXFLAGS} ${DIR}/main.cpp && time ./a.out < ${DIR}/sample.in
