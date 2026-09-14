import os
import sys
import subprocess

class RegResult:
    def __init__(self, register, value):
        self.register = register
        self.value    = value

class Test:
    def __init__(self, file, results):
        self.file    = file
        self.results = results

    def verify(self, output):
        if "error" in output:
            print(f"!! Test {self.file} failed:")
            print(output)
            return False

        lines = list(filter(lambda line: line.startswith("$$$"), output.split("\n")))

        for line in lines:
            parts = line[4:].split(" ")

            if len(parts) != 3:
                print("INVALID test statement:")
                print(line)
                sys.exit(1)

            if parts[0] == "reg":
                result2 = RegResult(parts[1], parts[2])

                for result in self.results:
                    if result.register != result2.register:
                        continue

                    if result.value != result2.value:
                        print("\n".join(lines))
                        print(f"\n\n!! Test {self.file} failed:")
                        print(f"!! {result.register} should be {result.value}, but is {result2.value}")
                        return False

        print(f"   Test {self.file} passed")
        return True

    def run(self):
        if os.system(f"customasm ../custom-asm/rules.asm {self.file} -o ./test.bin") != 0:
            print(f"FATAL: Failed to assembly {self.file}")
            sys.exit(1)
        print(f"Assembled {self.file}")

        res = subprocess.run(
            "../emu/y8emu --rom ../rom/rom.bin --cart test.bin --test", shell = True,
            check = True, capture_output = True
        )

        if res.returncode != 0:
            print(f"FATAL: Failed to run emulator:{res.stderr}\n")
            sys.exit(1)

        return self.verify(res.stdout.decode("ascii"))

tests = {
    "add": Test("cpu/add.asm", [RegResult("a", "0C"), RegResult("b", "04"), RegResult("cd", "8004")]),
    "div": Test("cpu/div.asm", [RegResult("a", "02"), RegResult("b", "04")]),
    "mul": Test("cpu/mul.asm", [RegResult("a", "20"), RegResult("b", "04")]),
    "sub": Test("cpu/sub.asm", [RegResult("a", "04"), RegResult("b", "04"), RegResult("cd", "7FFC")]),

    "mov1": Test("cpu/mov1.asm", [RegResult("ab", "0808"), RegResult("cd", "0808")]),
    "mov2": Test("cpu/mov2.asm", [RegResult("a", "08"), RegResult("cd", "0200")]),
    "mov3": Test("cpu/mov3.asm", [RegResult("a", "08"), RegResult("cd", "0200")]),
    "mov4": Test("cpu/mov4.asm", [RegResult("a", "FF")]),
    "mov5": Test("cpu/mov5.asm", [RegResult("a", "FF")]),
    "mov6": Test("cpu/mov6.asm", [RegResult("ab", "0200"), RegResult("cd", "0200")])
}

if len(sys.argv) >= 2:
    print(f"Running one test: {sys.argv[1]}")

    if not sys.argv[1] in tests:
        print(f"Test '{sys.argv[1]}' not found")
        sys.exit()

    if not tests[sys.argv[1]].run():
        print(f"Test {sys.argv[1]} failed")
        sys.exit(1)

    sys.exit()

for key in tests:
    print(f"----- Running test: {key}")

    if not tests[key].run():
        print(f"Test {key} failed")
        sys.exit(1)

print("All tests succeeded")
