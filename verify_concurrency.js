const http = require('http');

console.log('🚀 Starting Concurrency Test');
const start = Date.now();

// 1. Slow Request (2s delay)
http.get('http://localhost:8080/slow', (res) => {
    const elapsed = Date.now() - start;
    console.log('🐢 Slow finished at: ' + elapsed + 'ms');
}).on('error', (e) => console.error('Slow Error:', e.message));

// 2. Fast Request (should finish QUICKLY, not blocked)
setTimeout(() => {
    http.get('http://localhost:8080/fast', (res) => {
        const elapsed = Date.now() - start;
        console.log('🐇 Fast finished at: ' + elapsed + 'ms');
        
        if (elapsed < 1500) {
            console.log('✅ PASS: Fast request was NOT blocked!');
        } else {
            console.log('❌ FAIL: Fast request was blocked!');
        }
    }).on('error', (e) => console.error('Fast Error:', e.message));
}, 500); // Start 500ms after slow

