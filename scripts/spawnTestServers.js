// node scripts/spawnTestServers.js startServers {server count}
// for server count servers
// or 
// node scripts/spawnTestServers.js 
// for single server on port 3000

const { spawn } = require('child_process');
const http = require('http');
const process = require('process');


if (process.argv[2] === 'startServers') {
  const numberOfServers = process.argv[3] || 1;

  for (let i = 1; i <= numberOfServers; i++) {
    const serverProcess = spawn('node', [__filename, i]);

    serverProcess.stdout.on('data', (data) => {
      console.log(`Server ${i} : ${data}`);
    });

    serverProcess.stderr.on('data', (data) => {
      console.error(`Server ${i} : ${data}`);
    });

    serverProcess.on('close', (code) => {
      console.log(`Server ${i} exited with code ${code}`);
    });
  }
} else {
  const serverNumber = process.argv[2] || 0;

  const port = 3000 + parseInt(serverNumber, 10);
  const server = http.createServer((req, res) => {
    console.log("request came ");
    res.writeHead(200, { 'Content-Type': 'text/plain' });

    res.end(`Hello from server ${serverNumber}`);
  });

  server.listen(port, () => {
    console.log(`Server is listening on port ${port}`);
  });
}