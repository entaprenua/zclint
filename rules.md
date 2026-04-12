# zclint Rules

## 1. Allowed Imports

Only these imports are permitted. Any other imports will be flagged.

| Package | Allowed Imports |
|---------|-----------------|
| `lucide-solid` | `*` (any) |
| `solid-js` | `splitProps`, `mergeProps`, `Suspense` |
| `@solidjs/meta` | `MetaProvider`, `Title`, `Meta`, `Link`, `Base` |
| `@solidjs/router` | `Router`, `Routes`, `A` |
| `@solidjs/start` | `clientOnly` |
| `@solidjs/start/router` | `FileRoutes` |

---

## 2. Disallowed Patterns

Flag any occurrence of these patterns in code:

### Browser APIs
| Pattern | Reason |
|---------|--------|
| `window` | No direct browser API access |
| `document` | No direct DOM access |
| `*.target.*` | No event target access |
| `*.currentTarget.*` | No event target access |
| `localStorage` | No client storage |
| `sessionStorage` | No client storage |
| `document.cookie` | No cookie manipulation |

### Data Fetching / Networking
| Pattern | Reason |
|---------|--------|
| `fetch` | No data fetching |
| `WebSocket` | No websockets |
| `postMessage` | No cross-origin messaging |

### Dynamic Code
| Pattern | Reason |
|---------|--------|
| `eval` | Arbitrary code execution |
| `new Function` | Dynamic code creation |
| `setTimeout` | Dynamic code execution |
| `setInterval` | Dynamic code execution |
| `import(` | Dynamic imports |
| `with (` | Confusing scoping |

### HTML Injection
| Pattern | Reason |
|---------|--------|
| `.innerHTML` | XSS vulnerability |
| `.outerHTML` | XSS vulnerability |
| `dangerouslySetInnerHTML` | XSS vulnerability |
| `<script` | Script injection |
| `<iframe` | Clickjacking / embedding |
| `<embed` | Malicious content |
| `<object` | Malicious content |
| `javascript:` | XSS in URLs |
| `data:` | Data URI attacks |

### Event Handlers
| Pattern | Reason |
|---------|--------|
| `on*=` on HTML elements | Event handlers not allowed |
| `on*=` props on components | Component handler props not allowed |

Example:
```tsx
<button onClick={handleClick}>    // ❌ Blocked
<Button onClick={handler} />      // ❌ Blocked
<Button onDoThis={fn} />         // ❌ Blocked (any on* prop)
```

### Logging / Debugging
| Pattern | Reason |
|---------|--------|
| `console` | No logging allowed |
| `debugger` | Developer artifact |

---

## 3. No Inline Functions

Only JSX composition is allowed. No inline functions:

```tsx
// ❌ Blocked
<div>
  {() => {
    return <p>hello</p>
  }}
</div>

// ❌ Blocked
<button onClick={() => handleClick()}>
  Click
</button>
```

```tsx
// ✅ Allowed - use platform components for interactions
<Button>
  Click
</Button>
```

---

## 4. Plain TypeScript Files

`.ts` and `.js` files are not allowed. Rename to `.tsx` or `.jsx`.

---

## Summary

| Rule | Type |
|------|------|
| Allowed imports | Whitelist |
| Disallowed patterns | Blacklist (20+ patterns) |
| Event handlers | Pattern (`on*=` attributes) |
| No inline functions | Pattern |
| Plain .ts files | File type |
