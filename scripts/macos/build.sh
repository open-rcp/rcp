#!/bin/bash

echo "===================================="
echo "RCP Project - macOS Build Script"
echo "===================================="
echo

# Default values
BUILD_TYPE="debug"
BUILD_TARGET="all"
RUN_AFTER_BUILD=false
RUN_COMPONENT="rcpdaemon"
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
        --rcpcore)
            BUILD_TARGET="rcpcore"
            shift
            ;;
        --rcpcli)
            BUILD_TARGET="rcpcli"
            shift
            ;;
        --rcpdaemon)
            BUILD_TARGET="rcpdaemon"
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
        --run-daemon)
            RUN_AFTER_BUILD=true
            RUN_COMPONENT="daemon"
            shift
            ;;
        --run-server)
            RUN_AFTER_BUILD=true
            RUN_COMPONENT="daemon"
            echo "Warning: --run-server option is deprecated, use --run-daemon instead"
            shift
            ;;
        --run-service)
            RUN_AFTER_BUILD=true
            RUN_COMPONENT="daemon"
            echo "Warning: --run-service option is deprecated, use --run-daemon instead"
            shift
            ;;
        --run-client)
            RUN_AFTER_BUILD=true
            RUN_COMPONENT="client"
            shift
            ;;
        --run-ws-bridge)
            RUN_AFTER_BUILD=true
            RUN_COMPONENT="ws-bridge"
            shift
            ;;
        *)
            echo "Unknown option: $key"
            echo "Usage: $0 [--release|--debug] [--daemon|--client|--ws-bridge|--all] [--run|--run-daemon|--run-client|--run-ws-bridge]"
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

# Check if required environment variables are set
if [ -z "$OPENSSL_DIR" ]; then
    # Try to set them automatically if Homebrew is available
    if command -v brew &> /dev/null; then
        echo "Setting OpenSSL environment variables..."
        OPENSSL_PATH=$(brew --prefix openssl@3)
        export OPENSSL_DIR=$OPENSSL_PATH
        export OPENSSL_INCLUDE_DIR=$OPENSSL_PATH/include
        export OPENSSL_LIB_DIR=$OPENSSL_PATH/lib
    else
        echo "Warning: OPENSSL_DIR is not set. Build may fail."
        echo "Run the setup.sh script first or set these manually."
    fi
fi

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
    cargo build $BUILD_OPTS
    if [ $? -ne 0 ]; then
        echo "Error building project"
        exit 1
    fi
else
    if [ "$BUILD_TARGET" == "daemon" ]; then
        echo "Building RCP daemon in $BUILD_TYPE mode..."
        cargo build $BUILD_OPTS -p rcpdaemon
    elif [ "$BUILD_TARGET" == "client" ]; then
        echo "Building client component in $BUILD_TYPE mode..."
        cargo build $BUILD_OPTS -p rcpcli
    elif [ "$BUILD_TARGET" == "ws-bridge" ]; then
        echo "Building WebSocket bridge component in $BUILD_TYPE mode..."
        cargo build $BUILD_OPTS -p rcp-ws-bridge
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
        if [ "$RUN_COMPONENT" == "daemon" ]; then
            "./target/release/rcpdaemon"
        else
            "./target/release/rcp-$RUN_COMPONENT"
        fi
    else
        if [ "$RUN_COMPONENT" == "daemon" ]; then
            "./target/debug/rcpdaemon"
        else
            "./target/debug/rcp-$RUN_COMPONENT"
        fi
    fi
fi