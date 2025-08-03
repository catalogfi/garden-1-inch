# tsconfig
Common tsconfig for all node.js repositories

## Install

```bash
$ yarn add -D https://github.com/1inch/tsconfig.git#v1.0.0
```

And then setup `tsconfig.json`:
```json
{
  "extends": "@1inch/tsconfig",
  "include": ["src"],
  "compilerOptions": {
    "typeRoots": [
      "./src/types-overrides",
      "./node_modules/@types"
    ],
    "outDir": "./dist",
    "baseUrl": "./"
  }
}
```

## Release new version

```bash
# patch
$ yarn release --patch

# minot
$ yarn release --minor

# major
$ yarn release --major

# exact
$ yarn release v1.0.0
```
