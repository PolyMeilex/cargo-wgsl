from subprocess import Popen, PIPE, STDOUT

process = Popen(["./target/debug/cargo-wgsl", "--server"],
                stdout=PIPE, stdin=PIPE, stderr=STDOUT)

event = """{"event":{"ValidatePath": "./tests/test.wgsl"}}"""

p_stdout = process.communicate(input=str.encode(event))[0]

print(p_stdout.decode())
