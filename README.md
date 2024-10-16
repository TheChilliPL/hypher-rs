# hypher-rs

A simple WASM wrapper around the Typst's [Hypher](https://github.com/typst/hypher)

## Usage

```js
import { hyphenate, langExists } from "hypher-rs";

if (langExists("en")) {
  console.log(hyphenate("hyphenation", "en").join("-"));
}
```

```
hy-phen-ation
```

## Type definitions

```ts
export function langExists(lang: string): boolean;

export function hyphenate(word: string, lang: string): string[];
```
