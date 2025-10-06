# yCard WASM Usage Examples

## Node.js

```javascript
const ycard = require('./pkg/ycard_core.js');

// Clear existing aliases
ycard.yc_clear_aliases();

// Parse a yCard (basic example - real implementation would handle memory properly)
console.log('yCard WASM functions available:', Object.keys(ycard));
```

## Browser (ES Modules)

```javascript
import init, { yc_clear_aliases, yc_parse } from './pkg-web/ycard_core.js';

async function run() {
  await init();
  
  // Now you can use the WASM functions
  yc_clear_aliases();
  console.log('yCard WASM initialized in browser!');
}

run();
```

## Webpack/Bundler

```javascript
import init, * as ycard from './pkg-bundler/ycard_core.js';

async function initializeYCard() {
  await init();
  return ycard;
}

// Use in your application
initializeYCard().then(ycard => {
  ycard.yc_clear_aliases();
  // Ready to use yCard functions
});
```

## TypeScript Support

All packages include TypeScript definitions (`ycard_core.d.ts`) for full type safety:

```typescript
import init, { yc_parse, yc_validate } from './pkg/ycard_core.js';

async function parseYCard(input: string): Promise<number> {
  await init();
  // TypeScript will provide full type checking
  return yc_parse(/* ptr */ 0, /* len */ 0);
}
```