@echo off
echo RCP Project - Windows Build Script
echo ================================

:: Parse command line arguments
set BUILD_TYPE=debug
set BUILD_TARGET=rcpdaemon
set API_FEATURE=false

if "%1"=="--release" set BUILD_TYPE=release
if "%1"=="-r" set BUILD_TYPE=release
if "%2"=="rcpcore" set BUILD_TARGET=rcpcore
if "%2"=="rcpcli" set BUILD_TARGET=rcpcli
if "%2"=="rcpdaemon" set BUILD_TARGET=rcpdaemon
if "%2"=="examples" set BUILD_TARGET=examples
if "%2"=="all" set BUILD_TARGET=all
if "%3"=="--api" set API_FEATURE=true

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
    if "%API_FEATURE%"=="true" (
        echo Enabling API feature for rcpdaemon...
        cargo build %BUILD_OPTS% --features "rcpdaemon/api"
    ) else (
        cargo build %BUILD_OPTS%
    )
) else (
    if "%BUILD_TARGET%"=="rcpdaemon" (
        echo Building RCP daemon in %BUILD_TYPE% mode...
        if "%API_FEATURE%"=="true" (
            echo Enabling API feature for rcpdaemon...
            cargo build %BUILD_OPTS% -p rcpdaemon --features "api"
        ) else (
            cargo build %BUILD_OPTS% -p rcpdaemon
        )
    ) else if "%BUILD_TARGET%"=="rcpcli" (
        echo Building RCP client in %BUILD_TYPE% mode...
        cargo build %BUILD_OPTS% -p rcpcli
    ) else if "%BUILD_TARGET%"=="rcpcore" (
        echo Building RCP protocol in %BUILD_TYPE% mode...
        cargo build %BUILD_OPTS% -p rcpcore
    ) else if "%BUILD_TARGET%"=="examples" (
        echo Building examples in %BUILD_TYPE% mode...
        cargo build %BUILD_OPTS% -p rcp-examples
    )
)

:: Check build status
if %ERRORLEVEL% neq 0 (
    echo Build failed. Please check the errors above.
    exit /b %ERRORLEVEL%
)

echo Build completed successfully!

:: Run component if requested
if "%4"=="--run" (
    echo Running %BUILD_TARGET%...
    if "%BUILD_TYPE%"=="release" (
        if "%BUILD_TARGET%"=="rcpdaemon" (
            .\target\release\rcpdaemon.exe
        ) else if "%BUILD_TARGET%"=="rcpcli" (
            .\target\release\rcpcli.exe
        ) else if "%BUILD_TARGET%"=="examples" (
            echo Please specify which example to run from the target\release directory
            dir /B ".\target\release\examples" 2>nul || echo No examples built
        )
    ) else (
        if "%BUILD_TARGET%"=="rcpdaemon" (
            .\target\debug\rcpdaemon.exe
        ) else if "%BUILD_TARGET%"=="rcpcli" (
            .\target\debug\rcpcli.exe
        ) else if "%BUILD_TARGET%"=="examples" (
            echo Please specify which example to run from the target\debug directory
            dir /B ".\target\debug\examples" 2>nul || echo No examples built
        )
    )
)