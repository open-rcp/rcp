#!/bin/bash

echo "===================================="
echo "RCP Project - Linux Build Script"
echo "===================================="
echo

# Default values
BUILD_TYPE="debug"
BUILD_TARGET="all"
RUN_AFTER_BUILD=false
RUN_COMPONENT="rcpd"
API_FEATURE=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        --release)
            BUILD_TYPE="release"
            shift
            ;;
        --debug)
            BUILD_TYPE="debug"
            shift
            ;;
        --rcpp)
            BUILD_TARGET="rcpp"
            shift
            ;;
        --rcpc)
            BUILD_TARGET="rcpc"
            shift
            ;;
        --rcpd)
            BUILD_TARGET="rcpd"
            shift
            ;;
        --examples)
            BUILD_TARGET="examples"
            shift
            ;;
        --all)
            BUILD_TARGET="all"
            shift
            ;;
        --run)
            RUN_AFTER_BUILD=true
            shift
            ;;
        --run-rcpd)
            RUN_AFTER_BUILD=true
            RUN_COMPONENT="rcpd"
            shift
            ;;
        --run-rcpc)
            RUN_AFTER_BUILD=true
            RUN_COMPONENT="rcpc"
            shift
            ;;
        --run-examples)
            RUN_AFTER_BUILD=true
            RUN_COMPONENT="examples"
            shift
            ;;
        --api)
            API_FEATURE=true
            shift
            ;;
        *)
            echo "Unknown option: $key"
            echo "Usage: $0 [--release|--debug] [--rcpp|--rcpc|--rcpd|--examples|--all] [--run|--run-rcpd|--run-rcpc|--run-examples] [--api]"
            exit 1
            ;;
    esac
done

echo "Build configuration:"
echo "- Build type: $BUILD_TYPE"
echo "- Build target: $BUILD_TARGET"
if $RUN_AFTER_BUILD; then
    echo "- Will run $RUN_COMPONENT after build"
fi
echo

# Set build options based on build type
if [ "$BUILD_TYPE" == "release" ]; then
    BUILD_OPTS="--release"
else
    BUILD_OPTS=""
fi

# Create build directory
echo "Creating build directory..."
mkdir -p "target/$BUILD_TYPE"

echo "Building RCP components..."

# Build the selected components
if [ "$BUILD_TARGET" == "all" ]; then
    echo "Building all components in $BUILD_TYPE mode..."
    if $API_FEATURE; then
        echo "Enabling API feature for rcpd..."
        cargo build $BUILD_OPTS --features "rcpd/api"
    else
        cargo build $BUILD_OPTS
    fi
    if [ $? -ne 0 ]; then
        echo "Error building project"
        exit 1
    fi
else
    if [ "$BUILD_TARGET" == "rcpd" ]; then
        echo "Building RCP Daemon in $BUILD_TYPE mode..."
        if $API_FEATURE; then
            echo "Enabling API feature for rcpd..."
            cargo build $BUILD_OPTS -p rcpd --features "api"
        else
            cargo build $BUILD_OPTS -p rcpd
        fi
    elif [ "$BUILD_TARGET" == "rcpc" ]; then
        echo "Building RCP Client in $BUILD_TYPE mode..."
        cargo build $BUILD_OPTS -p rcpc
    elif [ "$BUILD_TARGET" == "rcpp" ]; then
        echo "Building RCP Protocol in $BUILD_TYPE mode..."
        cargo build $BUILD_OPTS -p rcpp
    elif [ "$BUILD_TARGET" == "examples" ]; then
        echo "Building examples in $BUILD_TYPE mode..."
        cargo build $BUILD_OPTS -p rcp-examples
    fi
    
    if [ $? -ne 0 ]; then
        echo "Error building $BUILD_TARGET"
        exit 1
    fi
fi

echo
echo "Build completed successfully!"

# Run component if requested
if $RUN_AFTER_BUILD; then
    echo "Running $RUN_COMPONENT..."
    if [ "$BUILD_TYPE" == "release" ]; then
        if [ "$RUN_COMPONENT" == "rcpd" ]; then
            "./target/release/rcpd"
        elif [ "$RUN_COMPONENT" == "rcpc" ]; then
            "./target/release/rcpc"
        elif [ "$RUN_COMPONENT" == "examples" ]; then
            echo "Please specify which example to run from the target/release directory"
            ls -la "./target/release/examples" 2>/dev/null || echo "No examples built"
        fi
    else
        if [ "$RUN_COMPONENT" == "rcpd" ]; then
            "./target/debug/rcpd"
        elif [ "$RUN_COMPONENT" == "rcpc" ]; then
            "./target/debug/rcpc"
        elif [ "$RUN_COMPONENT" == "examples" ]; then
            echo "Please specify which example to run from the target/debug directory"
            ls -la "./target/debug/examples" 2>/dev/null || echo "No examples built"
        fi
    fi
fi