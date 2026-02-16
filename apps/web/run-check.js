const { spawn } = require('child_process');

const proc = spawn('npx', ['svelte-check', '--tsconfig', './tsconfig.json'], {
  cwd: '/Users/billyribeiro/Documents/trade-master-ai/apps/web',
  stdio: ['inherit', 'pipe', 'pipe']
});

let stdout = '';
let stderr = '';

proc.stdout.on('data', (data) => {
  const str = data.toString();
  stdout += str;
  process.stdout.write(str);
});

proc.stderr.on('data', (data) => {
  const str = data.toString();
  stderr += str;
  process.stderr.write(str);
});

proc.on('close', (code) => {
  console.log('\n--- CAPTURED OUTPUT ---');
  console.log(stdout);
  console.log(stderr);
  process.exit(code);
});
