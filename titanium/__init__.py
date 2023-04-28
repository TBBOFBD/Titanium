if __name__ != "titanium": raise Exception("This file is not meant to be run directly")
from titanium.commands import *
from titanium.utils import *

VERSION = "1.0.0-alpha.5"

COMMANDS.add(HELP)
COMMANDS.add(RUN)
COMMANDS.add(BUILD)
COMMANDS.add(CLEAN)
COMMANDS.add(START)

def command_parser(cmd: str, args: list[str]) -> bool:
    METADATA = Metadata.get(args)

    if check_version(cmd, args): return HANDLED

    if cmd.startswith("help") or cmd.startswith("h"):
        if HELP.run(args): return HANDLED
    
    elif cmd.startswith("build") or cmd.startswith("b"):
        if BUILD.run(args, METADATA): return HANDLED
    
    elif cmd.startswith("clean") or cmd.startswith("c"):
        if CLEAN.run(args): return HANDLED
    
    elif cmd.startswith("start") or cmd.startswith("s"):
        if START.run(args): return HANDLED
    
    elif cmd.startswith("run") or cmd.startswith("r"):
        if RUN.run(args): return HANDLED

    elif cmd.startswith("version") or cmd.startswith("v"):
        print(f"Titanium version {VERSION}")
        return HANDLED
    
    elif cmd.startswith("installDeps"):
        return INSTALL_DEPS.run(args)

    return NOT_HANDLED

def helper() -> str: return COMMANDS.as_list_help()

# 
# MIT License
# 
# Copyright (c) 2022 AtomicGamer9523
# 
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
# 
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
# 
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.
# 
