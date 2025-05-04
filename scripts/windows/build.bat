@echo off
echo RCP Project - Windows Build Script
echo ================================

:: Parse command line arguments
set BUILD_TYPE=debug
set BUILD_TARGET=server

if "%1"=="--release" set BUILD_TYPE=release
if "%1"=="-r" set BUILD_TYPE=release
if "%2"=="server" set BUILD_TARGET=server
if "%2"=="client" set BUILD_TARGET=client
if "%2"=="ws-bridge" set BUILD_TARGET=ws-bridge
if "%2"=="all" set BUILD_TARGET=all

echo Build configuration:
echo - Build type: %BUILD_TYPE%
echo - Build target: %BUILD_TARGET%
echo.

:: Build targets
if "%BUILD_TYPE%"=="release" (
    set BUILD_OPTS=--release
) else (
    set BUILD_OPTS=
)

if "%BUILD_TARGET%"=="all" (
    echo Building all components in %BUILD_TYPE% mode...
    cargo build %BUILD_OPTS%
) else (
    if "%BUILD_TARGET%"=="server" (
        echo Building server component in %BUILD_TYPE% mode...
        cargo build %BUILD_OPTS% -p rcp-server
    ) else if "%BUILD_TARGET%"=="client" (
        echo Building client component in %BUILD_TYPE% mode...
        cargo build %BUILD_OPTS% -p rcp-client
    ) else if "%BUILD_TARGET%"=="ws-bridge" (
        echo Building WebSocket bridge in %BUILD_TYPE% mode...
        cargo build %BUILD_OPTS% -p rcp-ws-bridge
    )
)

:: Check build status
if %ERRORLEVEL% neq 0 (
    echo Build failed. Please check the errors above.
    exit /b %ERRORLEVEL%
)

echo Build completed successfully!