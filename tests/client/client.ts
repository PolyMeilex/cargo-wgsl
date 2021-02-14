import { spawn } from "child_process";

const p = spawn("../../target/debug/cargo-wgsl", ["--server"]);

p.stdin.setDefaultEncoding('utf8');
p.stdout.setEncoding('utf8');
p.stderr.setEncoding('utf8');

// process.stdin.setDefaultEncoding('utf8');
// process.stdout.setEncoding('utf8');

// process.stdin.pipe(p.stdin)
// p.stdout.pipe(process.stdout)

// const req = { event: { ValidatePath: "../test.wgsl" } };
const req = { jsonrpc: "2.0", method: "say_hello", params: [42, 23], id: 1 };


p.stdin.write(JSON.stringify(req) + '\n');

p.stdout.on("data", (data) => {
  console.log(`req: `, req);

  try {
    let json = JSON.parse(data);
    console.log(`res: `, json);
  }
  catch (e) {
    console.error(e);
  }

});

// p.stdout.on('end', function () {
//   console.log(`stdend: `);
// })

// p.stderr.on("data", (data) => {
//   console.error(`stderr: ${data}`);
// });


p.on("close", (code) => {
  console.log(`child process exited with code ${code}`);
});

