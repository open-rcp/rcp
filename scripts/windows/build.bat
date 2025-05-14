@echo off
echo RCP Project - Windows Build Script
echo ================================

:: Parse command line arguments
set BUILD_TYPE=debug
set BUILD_TARGET=rcpd
set API_FEATURE=false

if "%1"=="--release" set BUILD_TYPE=release
if "%1"=="-r" set BUILD_TYPE=release
if "%2"=="rcpp" set BUILD_TARGET=rcpp
if "%2"=="rcpc" set BUILD_TARGET=rcpc
if "%2"=="rcpd" set BUILD_TARGET=rcpd
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
        echo Enabling API feature for rcpd...
        cargo build %BUILD_OPTS% --features "rcpd/api"
    ) else (
        cargo build %BUILD_OPTS%
    )
) else (
    if "%BUILD_TARGET%"=="rcpd" (
        echo Building RCP daemon in %BUILD_TYPE% mode...
        if "%API_FEATURE%"=="true" (
            echo Enabling API feature for rcpd...
            cargo build %BUILD_OPTS% -p rcpd --features "api"
        ) else (
            cargo build %BUILD_OPTS% -p rcpd
        )
    ) else if "%BUILD_TARGET%"=="rcpc" (
        echo Building RCP client in %BUILD_TYPE% mode...
        cargo build %BUILD_OPTS% -p rcpc
    ) else if "%BUILD_TARGET%"=="rcpp" (
        echo Building RCP protocol in %BUILD_TYPE% mode...
        cargo build %BUILD_OPTS% -p rcpp
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
        if "%BUILD_TARGET%"=="rcpd" (
            .\target\release\rcpd.exe
        ) else if "%BUILD_TARGET%"=="rcpc" (
            .\target\release\rcpc.exe
        ) else if "%BUILD_TARGET%"=="examples" (
            echo Please specify which example to run from the target\release directory
            dir /B ".\target\release\examples" 2>nul || echo No examples built
        )
    ) else (
        if "%BUILD_TARGET%"=="rcpd" (
            .\target\debug\rcpd.exe
        ) else if "%BUILD_TARGET%"=="rcpc" (
            .\target\debug\rcpc.exe
        ) else if "%BUILD_TARGET%"=="examples" (
            echo Please specify which example to run from the target\debug directory
            dir /B ".\target\debug\examples" 2>nul || echo No examples built
        )
    )
)