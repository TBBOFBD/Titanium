if __name__ != "titanium.utils": raise Exception("This file is not meant to be run directly")
from abc import ABC, abstractmethod
from typing import Callable
from enum import Enum
HANDLED = True
NOT_HANDLED = False
class BuildTarget(Enum):
    Desktop = 0
    Web = 1
    JVM = 2
    NONE_OR_INVALID = 3
    def __str__(self) -> str: 
        if self == BuildTarget.JVM: return "JVM"
        elif self == BuildTarget.NONE_OR_INVALID: return "NONE/INVALID"
        else: return self.name.lower()
    def n(self) -> str: return self.__str__()
    @staticmethod
    def parse(string: str) -> 'BuildTarget':
        if string.lower() == "desktop": return BuildTarget.Desktop
        elif string.lower() == "web": return BuildTarget.Web
        elif string.lower() == "jvm": return BuildTarget.JVM
        else: return BuildTarget.NONE_OR_INVALID
def conc(*args: object) -> list[object]:
    out: list[object] = []
    for arg in args:
        if isinstance(arg, list): out += arg
        else: out.append(arg)
    return out
def check_version(cmd: str, args: list[str]) -> bool:
    meta = Metadata.get(args)
    ARGS = conc(args, cmd)
    def ev(color: bool = True) -> None:
        LOG.log(f"Enderpearl Version: {COLORS.HEADER if color else ''}33a810f{COLORS.ENDC if color else ''}")
    def verbose(color: bool = True) -> None:
        LOG.log(f"Build Output Triple: {COLORS.HEADER if color else ''}{meta.triple.split('-')[2].lower()}-{meta.target.n()}-{('release' if meta.release else 'debug')}{COLORS.ENDC if color else ''}")
        LOG.log(f"RustC Target Triple: {COLORS.HEADER if color else ''}{meta.triple}{COLORS.ENDC if color else ''}")
        ev(color)
    VERBOSE = [
        "-vV",
        "-Vv",
    ]
    RAW = [
        "-vr",
        "-rv",
    ]
    VERBOSE_RAW = [
        "-vVr",
        "-Vvr",
        "-vrV",
        "-Vrv",
        "-rvV",
        "-rVv",
    ]
    for vr in VERBOSE_RAW:
        if vr in ARGS: verbose(False); return HANDLED
    for r in RAW:
        if r in ARGS: ev(False); return HANDLED
    for v in VERBOSE:
        if v in ARGS: verbose(); return HANDLED
    if "-v" in ARGS:
        ev(); return HANDLED
    return NOT_HANDLED
class COLORS:
    GRAY = "\033[90m"
    HEADER = "\033[95m"
    OKBLUE = "\033[94m"
    OKCYAN = "\033[96m"
    OKGREEN = "\033[92m"
    WARNING = "\033[93m"
    FAIL = "\033[91m"
    ENDC = "\033[0m"
    BOLD = "\033[1m"
    UNDERLINE = "\033[4m"
class Command():
    def __init__(self, name: str, **args: str) -> None:
        self.name: str = name
        self.simple: str = args.get("simple", "No simple discription provided")
        self.description: str = args.get("description", "No description provided")
        self.color: str = args.get("color", COLORS.OKGREEN)
    def as_simple_help(self) -> str: return f"{self.color}{self.name}{COLORS.ENDC}: {self.simple}"
    def as_list_help(self) -> str: return f"  - {self.color}{self.name}{COLORS.ENDC}: {self.simple}"
    def as_full_help(self) -> str: return f"{self.color}{self.name}{COLORS.ENDC}: {self.description}"
    @abstractmethod
    def run(self, cliargs: list[str], **args: object) -> bool: pass
class CommandGroup(ABC):
    def __init__(self) -> None: self.commands: dict[str, Command] = {}
    def add(self, command: Command) -> None: self.commands[command.name] = command
    def get(self, name: str) -> Command: return self.commands[name]
    def as_list_help(self) -> str:
        res = ""
        i = 0
        LEN = self.commands.__len__()
        for command in self.commands:
            i += 1
            res += self.commands[command].as_list_help() + ("\n" if i < LEN else "")
        return res
    def display_command_help(self, name: str) -> None: LOG.log(self.commands[name].as_full_help())
    def __iter__(self): return iter(self.commands)
    def __str__(self) -> str: return self.as_list_help()
class Metadata():
    def __init__(self, release: bool, target: BuildTarget, triple: str) -> None:
        self.release: bool = release
        self.target: BuildTarget = target
        self.triple: str = triple
    def is_empty(self) -> bool:
        return self.target == BuildTarget.NONE_OR_INVALID and self.triple == ""
    def __str__(self) -> str: return "Metadata: { release: "+COLORS.HEADER+self.release.__str__()+COLORS.ENDC+", target: "+COLORS.HEADER+self.target.__str__()+COLORS.ENDC+", triple: "+COLORS.HEADER+self.triple+COLORS.ENDC+"}"
    @staticmethod
    def get(args: list[str]) -> 'Metadata':
        import os
        release = False
        if "--release" in args: release = True
        if "--debug" in args: release = False
        rustc: str = os.popen("rustc -Vv").read().split("host: ")[1].splitlines()[0]
        buildtarget: BuildTarget = BuildTarget.JVM
        if rustc.startswith("x86_64"): buildtarget = BuildTarget.Desktop
        elif rustc.startswith("wasm32") or rustc.startswith("wasm64"): buildtarget = BuildTarget.Web
        return Metadata(release, buildtarget, rustc)
    @staticmethod
    def empty() -> 'Metadata': return Metadata(False, BuildTarget.NONE_OR_INVALID, "")
class Logger():
    def __init__(self, logger: Callable[..., None]) -> None: self.logger: Callable[..., None] = logger
    def log(self, *args: object): self.logger(" ".join([arg.__str__() for arg in args]))
def h(cmd: str, args: list[str], checkForLength: bool = True) -> bool:
    if (
        args.__contains__("-h") or
        args.__contains__("--help") or
        args.__contains__("help") or
        ((args.__len__() <= 0) if checkForLength else False)
    ):
        COMMANDS.display_command_help(cmd)
        return HANDLED
    return NOT_HANDLED
def failableRun(cmd: str):
    import os
    res = os.system(cmd)
    if res != 0:
        LOG.log(COLORS.FAIL+"Failed to run command: "+COLORS.ENDC+cmd)
        exit(res)
COMMANDS = CommandGroup()
LOG = Logger(print)

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
