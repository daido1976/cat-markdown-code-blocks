```sh
$ wasm-pack build --target nodejs
```

```sandbox.mjs
import { add } from "./wasm/pkg/wasm.js";

console.log(add(1, 2));
```

```sh
$ node sandbox.mjs
# 3
```
