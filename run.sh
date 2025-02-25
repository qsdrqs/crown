set -euf

PROJ_DIR=$(dirname $0)

source $PROJ_DIR/find_entry.sh

PREPROCESS="$PROJ_DIR/preprocess.sh"
BENCHMARK="$PROJ_DIR/benchmark"

WORKSPACE=""
if [ $# -eq 0 ]; then
    if [ -d "$PROJ_DIR/results" ]; then
        echo "Please provide a workspace dir. Tried $PROJ_DIR/results but exists"
        exit 1
    fi
    WORKSPACE="$PROJ_DIR/results"
elif [ -d $1 ]; then
    echo "$1 exists"
    exit 1
else
    WORKSPACE=$1
fi

cp -r $BENCHMARK $WORKSPACE

"$PREPROCESS" $WORKSPACE

CROWN="$PROJ_DIR/target/release/crown"

RUSTC_PATH=$(rustc +nightly-2023-01-26 --print sysroot)/lib

if [[ "$OSTYPE" == "darwin"* ]]; then
    # add rustc lib to dyld path
    export DYLD_FALLBACK_LIBRARY_PATH=$RUSTC_PATH
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    export LD_LIBRARY_PATH=$RUSTC_PATH
else
    echo "platform $OSTYPE" not supported
    exit 1
fi

for f in $(find $WORKSPACE -name "Cargo.toml"); do
    BENCH_DIR="$(dirname $f)"
    BENCH_NAME="$(basename $BENCH_DIR)"
    ENTRY=$(find_entry $BENCH_DIR)
    echo "rewriting $BENCH_NAME"
    OPTIONS=""
    if [ $BENCH_NAME = "lil" ]; then
        OPTIONS="--type-reconstruction --no-attempt .*fnc_.*|do_exit|lil_find_var|lil_to_double"
    elif [ $BENCH_NAME = "libsamplerate" ]; then
        OPTIONS="--no-attempt .*_vari_process|.*_reset"
    elif [ $BENCH_NAME = "lodepng" ]; then
        OPTIONS="--no-attempt bpmnode_create|uivector_resize"
    elif [ $BENCH_NAME = "quadtree" ]; then
        OPTIONS="--force-box"
    elif [ $BENCH_NAME = "genann" ]; then
        OPTIONS="--raw-mutability"
    fi

    if [ -d "$BENCH_DIR/analysis_results" ]; then
        rm $BENCH_DIR/analysis_results/*
    else
        mkdir -p $BENCH_DIR/analysis_results
    fi
    $CROWN $ENTRY rewrite --results-path $BENCH_DIR/analysis_results $OPTIONS in-place || { echo "rewrite $f crashed" ; exit 1; }
    $CROWN $ENTRY summarise --results-path $BENCH_DIR/analysis_results
done
