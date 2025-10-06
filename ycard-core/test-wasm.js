const ycard = require('./pkg/ycard_core.js');

// Test basic functionality
console.log('yCard WASM module loaded successfully!');
console.log('Available functions:', Object.keys(ycard));

// Test clearing aliases (should return 0 for success)
const result = ycard.yc_clear_aliases();
console.log('Clear aliases result:', result);