@echo off
title Mycelium - Protocol Symbiosis
echo.
echo ========================================
echo    MYCELIUM v1.0.0 - PROTOCOL SYMBIOSIS
echo ========================================
echo.
echo Starting Mycelium desktop application...
echo.

REM Переходим в папку с релизом
cd /d "%~dp0"

REM Проверяем наличие Python
python --version >nul 2>&1
if errorlevel 1 (
    echo ERROR: Python not found!
    echo Please install Python from https://python.org
    echo.
    pause
    exit /b 1
)

REM Запускаем локальный сервер
echo Starting local server on port 8000...
echo.
echo Mycelium will open in your default browser.
echo To stop the server, close this window.
echo.
echo ========================================
echo.

REM Запускаем сервер и открываем браузер
start "" http://localhost:8000
python -m http.server 8000

echo.
echo Mycelium server stopped.
pause 