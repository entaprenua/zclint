# zclint

**Zero-code linter for codeless components**

A fast, offline CLI tool for validating storefront code in a codeless component architecture.

## Features

- **Fast** - Local validation, no network latency
- **Offline-first** - No internet required
- **AI-friendly** - JSON output for programmatic use
- **Simple** - Single binary, multiple commands
- **Secure** - Blocks dangerous patterns and imports

## Installation

### Binary (Recommended)

```bash
# Linux/macOS
curl -fsSL https://get.zclint.dev | sudo install

# Or download from releases
curl -L https://github.com/entaprenua/zclint/releases/latest/download/zclint -o zclint
chmod +x zclint
sudo mv zclint /usr/local/bin/
```

### Local install (No sudo required)

```bash
mkdir -p ~/.local/bin
cp /path/to/zclint ~/.local/bin/zclint
chmod +x ~/.local/bin/zclint

# Add to PATH (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.local/bin:$PATH"
```

### Build from source

```bash
git clone https://github.com/entaprenua/zclint.git
cd zclint
cargo build --release
./target/release/zclint --help
```

## Usage

### CLI

```bash
# Validate directory (default: current directory)
zclint check

# Validate specific directory
zclint check src/

# Validate specific file
zclint check src/components/Button.tsx

# Validate multiple files/directories
zclint check src/ lib/

# JSON output (for CI/AI)
zclint check --json

# Install pre-commit hook
zclint install

# Initialize config file
zclint init
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | No errors |
| 1 | Validation errors found |
| 2 | File not found or other error |

## Rules

### 1. Allowed Imports

Only specific imports from whitelisted packages are permitted.

```tsx
// ✅ Allowed
import { splitProps, mergeProps } from "solid-js";
import { MetaProvider, Title } from "@solidjs/meta";
import { Router, A } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { ArrowRight } from "lucide-solid";

// ❌ Blocked
import { createSignal } from "solid-js";
import axios from "axios";
```

| Package | Allowed Imports |
|---------|----------------|
| `lucide-solid` | Any |
| `solid-js` | `splitProps`, `mergeProps`, `Suspense` |
| `@solidjs/meta` | `MetaProvider`, `Title`, `Meta`, `Link`, `Base` |
| `@solidjs/router` | `Router`, `Routes`, `A` |
| `@solidjs/start` | `clientOnly` |
| `@solidjs/start/router` | `FileRoutes` |

### 2. Event Handlers

Any attribute starting with `on` is blocked.

```tsx
// ❌ Blocked
<button onClick={handleClick}>
<input onInput={handleInput}>
<Button onCustom={handle}>

// ✅ Fix
Use platform components: <Button>, <Input>, <Select>, etc.
```

### 3. Inline Functions

Inline functions returning JSX are not allowed.

```tsx
// ❌ Blocked
<div>{() => { return <p>hello</p>; }}</div>

// ✅ Fix
Use JSX composition instead of inline functions
```

### 4. Disallowed Patterns

The following patterns are blocked for security:

| Pattern | Reason |
|---------|--------|
| `window` | No direct browser API access |
| `document` | No direct DOM access |
| `localStorage`, `sessionStorage` | No client storage |
| `document.cookie` | No cookie manipulation |
| `fetch`, `WebSocket`, `postMessage` | No networking |
| `eval`, `new Function`, `setTimeout`, `setInterval` | No dynamic code execution |
| `import()` | No dynamic imports |
| `with` | No confusing scoping |
| `.innerHTML`, `.outerHTML`, `dangerouslySetInnerHTML` | No XSS |
| `<script`, `<iframe`, `<embed`, `<object` | No script injection |
| `javascript:`, `data:` | No XSS in URLs |
| `console` | No logging |
| `debugger` | No debug statements |

### 5. Plain TypeScript Files

`.ts` and `.js` files are not allowed in user space.

```tsx
// ❌ Blocked
// utils.ts, helpers.js

// ✅ Fix
Rename to .tsx or .jsx extension.
```

## Protected Directories

The following directories are automatically skipped (platform code):

- `~/components/ui/**` - Platform UI components
- `~/lib/**` - Platform library code

## Configuration

Create a `zclint.yaml` config file:

```yaml
include:
  - "**/*.{tsx,jsx}"

exclude:
  - "node_modules/**"
  - "dist/**"
  - "~/components/ui/**"
  - "~/lib/**"

rules:
  no-disallowed-imports: error
  no-disallowed-patterns: error
  no-event-handlers: error
  no-inline-functions: error
  no-plain-ts: error
```

## Project Structure

```
zclint/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── cli/              # CLI commands and output
│   ├── core/             # Core types and linter
│   └── rules/            # Rule implementations
├── rules.md              # Detailed rules documentation
├── README.md
├── Cargo.toml
└── .gitignore
```

## License

MIT
