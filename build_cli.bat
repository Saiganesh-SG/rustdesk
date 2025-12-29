@echo off
REM Build script for RustDesk CLI on Windows

echo Building RustDesk CLI...
echo ========================
echo.

REM Check if Cargo is installed
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: Cargo (Rust) is not installed
    echo Please install Rust from https://rustup.rs/
    exit /b 1
)

REM Initialize git submodules
echo Initializing git submodules...
git submodule update --init --recursive

REM Build the CLI binary
echo Building rustdeskcli binary...
cargo build --bin rustdeskcli --release

REM Check if build was successful
if exist "target\release\rustdeskcli.exe" (
    echo.
    echo [32mBuild successful![0m
    echo.
    echo The CLI binary is located at: target\release\rustdeskcli.exe
    echo.
    echo Usage example:
    echo   .\target\release\rustdeskcli.exe --remoteId ^<ID^> --idServer ^<SERVER^> --key ^<KEY^>
    echo.
    echo For more information, see CLI_README.md
) else (
    echo.
    echo [31mBuild failed![0m
    echo Please check the error messages above.
    exit /b 1
)
