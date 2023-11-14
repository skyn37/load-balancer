//node scripts/testConcurency.js {concurency count}
// defaults to 10 

const { exec } = require('child_process');
const numRequests = process.argv[2] || 10;

function launchCurl() {
    const curlCommand = 'curl http://127.0.0.1:6969 -v';
    return new Promise((resolve, reject) => {
        exec(curlCommand, (error, stdout, stderr) => {
            if (error) {
                reject(error);
            } else {
                resolve(stdout);
            }
        });
    });
}

async function main() {
    const curlPromises = Array.from({ length: numRequests }, () => launchCurl());
    try {
        const responses = await Promise.all(curlPromises);
        responses.forEach((response, index) => {
            console.log(`Response ${index + 1}: ${response}`);
        });
    } catch (error) {
        console.error(`Error: ${error.message}`);
    }
}

main();
