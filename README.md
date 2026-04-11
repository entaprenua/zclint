# zclint

**Zero-code linter for codeless components**

A fast, offline CLI tool for validating storefront code in a codeless component architecture.

## Features

- **Fast** - Local validation, no network latency
- **Offline-first** - No internet required
- **AI-friendly** - JSON output for programmatic use
- **Simple** - Single binary, multiple commands

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

### Event Handlers

```tsx
// ❌ Blocked
onClick, onChange, onSubmit, onInput, onMouseEnter, onMouseLeave, etc.

// ✅ Fix
Use platform components: <Button>, <Input>, <Select>, etc.
```

### Reactive Primitives

```tsx
// ❌ Blocked
createSignal, createEffect, createMemo, createStore, createContext, etc.

// ✅ Fix
Use platform components with built-in state management.
```

### Operators

```tsx
// ❌ Blocked
condition ? a : b          (ternary)
condition && <Component />  (logical AND)

// ✅ Fix
<Show when={condition}>
  <Component />
</Show>
```

### Plain TypeScript Files

```tsx
// ❌ Blocked
*.ts, *.js files (except index.ts, index.js)

// ✅ Fix
Rename to .tsx or .jsx extension.
```

## Protected Directories

The following directories are automatically skipped:

- `components/ui/**` - Platform UI components
- `lib/**` - Platform library code

## Configuration

Create a `zclint.yaml` config file:

```yaml
include:
  - "**/*.{tsx,jsx}"

exclude:
  - "node_modules/**"
  - "dist/**"

rules:
  no-event-handlers: error
  no-reactive-primitives: error
  no-ternary: error
  no-logical-and: error
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
├── Cargo.toml
└── README.md
```

## License

MIT
