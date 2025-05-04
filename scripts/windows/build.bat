@echo off
echo ====================================
echo RCP Project - Windows Build Script
echo ====================================
echo.

setlocal enabledelayedexpansion

:: Parse command line arguments
set BUILD_TYPE=debug
set BUILD_TARGET=all
set RUN_AFTER_BUILD=0
set RUN_COMPONENT=server

:parse_args
if "%~1"=="" goto :end_parse_args
if /i "%~1"=="--release" set BUILD_TYPE=release
if /i "%~1"=="--debug" set BUILD_TYPE=debug
if /i "%~1"=="--server" set BUILD_TARGET=server
if /i "%~1"=="--client" set BUILD_TARGET=client
if /i "%~1"=="--ws-bridge" set BUILD_TARGET=ws-bridge
if /i "%~1"=="--all" set BUILD_TARGET=all
if /i "%~1"=="--run" set RUN_AFTER_BUILD=1
if /i "%~1"=="--run-server" (
    set RUN_AFTER_BUILD=1
    set RUN_COMPONENT=server
)
if /i "%~1"=="--run-client" (
    set RUN_AFTER_BUILD=1
    set RUN_COMPONENT=client
)
if /i "%~1"=="--run-ws-bridge" (
    set RUN_AFTER_BUILD=1
    set RUN_COMPONENT=ws-bridge
)
shift
goto :parse_args
:end_parse_args

echo Build configuration:
echo - Build type: %BUILD_TYPE%
echo - Build target: %BUILD_TARGET%
if %RUN_AFTER_BUILD%==1 (
    echo - Will run %RUN_COMPONENT% after build
)
echo.

:: Set environment variables for OpenSSL if they're not set
if "%OPENSSL_DIR%"=="" (
    echo Setting OpenSSL environment variables...
    set OPENSSL_DIR=C:\Program Files\OpenSSL-Win64
    set OPENSSL_INCLUDE_DIR=C:\Program Files\OpenSSL-Win64\include
    set OPENSSL_LIB_DIR=C:\Program Files\OpenSSL-Win64\lib
)

:: Create build directory
echo Creating build directory...
if not exist "target\%BUILD_TYPE%" mkdir "target\%BUILD_TYPE%"

echo Building RCP components...

:: Build the selected components
if "%BUILD_TARGET%"=="all" (
    echo Building all components in %BUILD_TYPE% mode...
    cargo build %BUILD_OPTS%
    if !errorlevel! neq 0 (
        echo Error building project
        exit /b !errorlevel!
    )
) else (
    if "%BUILD_TYPE%"=="release" (
        set BUILD_OPTS=--release
    ) else (
        set BUILD_OPTS=
    )
    
    if "%BUILD_TARGET%"=="server" (
        echo Building server component in %BUILD_TYPE% mode...
        cargo build %BUILD_OPTS% -p rcp-server
    ) else if "%BUILD_TARGET%"=="client" (
        echo Building client component in %BUILD_TYPE% mode...
        cargo build %BUILD_OPTS% -p rcp-client
    ) else if "%BUILD_TARGET%"=="ws-bridge" (
        echo Building WebSocket bridge component in %BUILD_TYPE% mode...
        cargo build %BUILD_OPTS% -p rcp-ws-bridge
    )
    
    if !errorlevel! neq 0 (
        echo Error building %BUILD_TARGET%
        exit /b !errorlevel!
    )
)

echo.
echo Build completed successfully!

:: Run component if requested
if %RUN_AFTER_BUILD%==1 (
    echo Running %RUN_COMPONENT%...
    if "%BUILD_TYPE%"=="release" (
        target\release\rcp-%RUN_COMPONENT%.exe
    ) else (
        target\debug\rcp-%RUN_COMPONENT%.exe
    )
)

endlocal