if __name__ != "titanium.commands": raise Exception("This file is not meant to be run directly")

from titanium.utils import *
import platform, os

class BuildCommand(Command):
    def __init__(self, target_triple: str = "", release: bool = False, target: BuildTarget = BuildTarget.NONE_OR_INVALID) -> None:
        opsy: str = ""
        try: opsy = target_triple.split("-")[2].capitalize()
        except Exception: opsy = "Windows" if platform.system() == "Windows" else "Linux"
        PREFIX = (".\\" if opsy == "Windows" else "./") + "epearl"
        super().__init__("build",
            simple = "builds the project",
            description = f"""Builds the project. Allows you to specify the {COLORS.OKCYAN}platform{COLORS.ENDC}, {COLORS.OKCYAN}target{COLORS.ENDC}, and {COLORS.OKCYAN}build type{COLORS.ENDC}.
       The default {COLORS.OKCYAN}platform{COLORS.ENDC} is {COLORS.WARNING}the current platform{COLORS.ENDC} ({COLORS.HEADER}{opsy}{COLORS.ENDC}).
       The default {COLORS.OKCYAN}build type{COLORS.ENDC} is {COLORS.WARNING}debug{COLORS.ENDC} ({COLORS.HEADER}{("release" if release else "debug")}{COLORS.ENDC}).
       The default {COLORS.OKCYAN}target{COLORS.ENDC} is {COLORS.WARNING}desktop{COLORS.ENDC} ({COLORS.HEADER}{target.n()}{COLORS.ENDC}).
    
{COLORS.GRAY}Examples:{COLORS.ENDC}
{PREFIX} build --status      {COLORS.GRAY} Prints the status and the default output of the build{COLORS.ENDC}
{PREFIX} build desktop       {COLORS.GRAY} Builds for the desktop target in debug mode{COLORS.ENDC}
{PREFIX} build web --debug   {COLORS.GRAY} Builds for the web target in debug mode{COLORS.ENDC}
{PREFIX} build JVM --release {COLORS.GRAY} Builds for the JVM target in release mode{COLORS.ENDC}
""",
            color = COLORS.OKBLUE
        )
    def run(self, cliargs: list[str], metadata: Metadata = Metadata.empty()) -> bool:
        if h(self.name, cliargs): return HANDLED
        if cliargs[0] == "--status":
            LOG.log(COLORS.OKCYAN + "Build Status:" + COLORS.ENDC + f""" Building for {COLORS.HEADER}{metadata.target.n()}{COLORS.ENDC} on {COLORS.HEADER}{metadata.triple}{COLORS.ENDC} in {COLORS.HEADER}{'release' if metadata.release else 'debug'}{COLORS.ENDC} mode.""")
            return HANDLED
        full_build = False
        if "--full" in cliargs: full_build = True

        def build_desktop():
            config: list[str] = (
                ["--config \"target.'cfg(not(target_os=\\\"windows\\\"))'.runner='sudo'\""]
                if metadata.triple.split("-")[2] == "linux" else
                []
            )
            cmd = f"""cargo +nightly {(" ".join(config))} build -Z unstable-options --target-dir .build/rust --out-dir .build/bin --target {metadata.triple}"""
            os.system(cmd)
            return HANDLED
        
        def build_web():
            LOG.log("Building for the web target is not yet supported!")
            return HANDLED
        
        def build_jvm():
            LOG.log("Building for the JVM target is not yet supported!")
            return HANDLED
        
        if full_build:
            LOG.log("Full build requested! Are you sure you want to do this? (y/n)")
            INPUT = input()
            if INPUT.lower() != "y":
                LOG.log("Aborting...")
                return HANDLED
            LOG.log("Cleaning...")
            CLEAN.run([])
            LOG.log("Building...")
            build_desktop()
            build_web()
            build_jvm()
            LOG.log("Done!")
            return HANDLED

        buildtarget = BuildTarget.parse(cliargs[0])
        if buildtarget == BuildTarget.NONE_OR_INVALID:
            LOG.log(f"""{COLORS.FAIL}'{COLORS.OKBLUE}{cliargs[0]}{COLORS.FAIL}' doesn't seem to be a valid target! Valid Targets include: '{COLORS.OKCYAN}desktop{COLORS.FAIL}', '{COLORS.OKCYAN}web{COLORS.FAIL}', and '{COLORS.OKCYAN}jvm{COLORS.FAIL}'{COLORS.ENDC}""")
            return HANDLED

        if buildtarget == BuildTarget.Desktop: return build_desktop()
        if buildtarget == BuildTarget.Web: return build_web()
        if buildtarget == BuildTarget.JVM: return build_jvm()

        return HANDLED
class CleanCommand(Command):
    def __init__(self) -> None: super().__init__("clean",
        simple = "cleans the project",
        description = f"""Cleans the project. Deletes the {COLORS.OKCYAN}.build{COLORS.ENDC} folder.""",
        color = COLORS.OKBLUE                                   
    )
    def run(self, cliargs: list[str]) -> bool:
        import shutil, os
        if h(self.name, cliargs, False): return HANDLED
        LOG.log("Cleaning...")
        CMD = "cargo +nightly -Z unstable-options clean --quiet"
        os.system(CMD)
        try:
            shutil.rmtree(".build")
            shutil.rmtree("./titanium/utils/__pycache__")
            shutil.rmtree("./titanium/__pycache__")
        except Exception: pass
        LOG.log("Done!")
        return HANDLED
class HelpCommand(Command):
    def __init__(self) -> None: super().__init__("help",
        simple = "shows this help message",
        description = f"""Shows this help message.
      If you need help with a specific command, add a {COLORS.HEADER}-h{COLORS.ENDC} to the end.""",
        color = COLORS.OKGREEN
    )
    def run(self, cliargs: list[str]) -> bool:
        if h(self.name, cliargs, False): return HANDLED
        LOG.log(COMMANDS.as_list_help())
        return HANDLED
class StartCommand(Command):
    def __init__(self) -> None: super().__init__("start",
        simple = "sets up the project",
        description = f"""Sets up the project. Can be either {COLORS.OKCYAN}lib{COLORS.ENDC} or {COLORS.OKCYAN}framework{COLORS.ENDC}.
       Use {COLORS.OKCYAN}lib{COLORS.ENDC} if you are going to import titanium into another project as a dependency.
       Use {COLORS.OKCYAN}framework{COLORS.ENDC} if you are going to use titanium and its dependencies as a framework."""
       ,
        color = COLORS.OKCYAN
    )
    def run(self, cliargs: list[str]) -> bool:
        if h(self.name, cliargs): return HANDLED

        def as_framework() -> bool:
            if os.path.exists("main"):
                LOG.log("Project already set up as a framework!")
                return HANDLED
            LOG.log("Setting up project as a framework...")
            try:
                try: os.mkdir("main")
                except Exception: pass
                mainFile = open("main/main.rs","wt")
                mainFile.write("use titanium::*;\n\nfn main() {\n    hello_world!();\n}")
                mainFile.close()
            except Exception: pass
            f = open("Cargo.toml", "r")
            OLD_FILE = f.read()
            f.close()
            f = open("Cargo.toml", "r")
            newData = f.readlines()
            f.close()
            shouldWriteNextLine = False
            for line in range(len(newData)):
                if shouldWriteNextLine:
                    newData[line] = "[[bin]]\npath = \"main/main.rs\"\nname = \"main\"\n"
                    shouldWriteNextLine = False
                if "#"+"?RESERVED_FOR_TITANIUM_FRAMEWORK_START" in newData[line]: shouldWriteNextLine = True
                if "#"+"?RESERVED_FOR_TITANIUM_FRAMEWORK_END" in newData[line]: shouldWriteNextLine = False
            FILE = open("Cargo.toml", "wt")
            FILE.write("".join(newData))
            FILE.close()
            backup = open("Cargo.toml", "rt")
            backupData = backup.read()
            if ("#" + "!TITANIUM_OK") not in backupData:
                LOG.log("File not updated successfully, rolling back...")
                f = open("Cargo.toml", "wt")
                f.write(OLD_FILE)
                f.close()
            backup.close()
            os.system("cargo +nightly -Z unstable-options check")
            LOG.log("Done!")
            return HANDLED
        
        def as_lib() -> bool:
            LOG.log("Setting up project as a library...")
            return HANDLED

        if "lib" in cliargs[0]: return as_lib()
        if "framework" in cliargs[0]: return as_framework()
        return HANDLED
class RunCommand(Command):
    def __init__(self) -> None: super().__init__("run",
        simple = "runs the project",
        description = f"""Runs the project.""",
        color = COLORS.OKBLUE
    )
    def run(self, cliargs: list[str]) -> bool:
        if h(self.name, cliargs, False): return HANDLED
        CMD = "RUSTFLAGS=\"-Z macro-backtrace\" cargo +nightly -Z unstable-options run"
        os.system(CMD)
        return HANDLED

BUILD = BuildCommand()
CLEAN = CleanCommand()
HELP = HelpCommand()
START = StartCommand()
RUN = RunCommand()


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
