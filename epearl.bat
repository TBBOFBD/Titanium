@REM !/bin/bat

@REM
@REM MIT License
@REM 
@REM Copyright (c) 2022 AtomicGamer9523
@REM 
@REM Permission is hereby granted, free of charge, to any person obtaining a copy
@REM of this software and associated documentation files (the "Software"), to deal
@REM in the Software without restriction, including without limitation the rights
@REM to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
@REM copies of the Software, and to permit persons to whom the Software is
@REM furnished to do so, subject to the following conditions:
@REM 
@REM The above copyright notice and this permission notice shall be included in all
@REM copies or substantial portions of the Software.
@REM 
@REM THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
@REM IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
@REM FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
@REM AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
@REM LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
@REM OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
@REM SOFTWARE.
@REM


@echo off
@REM ##########################################################################
@REM
@REM  Enderpearl script for Windows
@REM
@REM ##########################################################################

@REM Set local scope for the variables with windows NT shell
if "%OS%"=="Windows_NT" setlocal

set DIRNAME=%~dp0
if "%DIRNAME%" == "" set DIRNAME=.
set APP_BASE_NAME=%~n0
set APP_HOME=%DIRNAME%

@REM Resolve any "." and ".." in APP_HOME to make it shorter.
for %%i in ("%APP_HOME%") do set APP_HOME=%%~fi

@REM Build
set PYTHON_EXE=python.exe
%PYTHON_EXE% --version >NUL 2>&1
if "%ERRORLEVEL%" == "0" goto execute

echo.
echo ERROR: no 'rustc' command could be found in your PATH.
echo.
echo Please install rust

:execute

@REM Build Enderpearl
"%PYTHON_EXE%" "%APP_HOME%\x.py" "%*%"

:end
@REM End local scope for the variables with windows NT shell
if "%ERRORLEVEL%"=="0" goto mainEnd

:fail
exit /b 1

:mainEnd
if "%OS%"=="Windows_NT" endlocal

:omega