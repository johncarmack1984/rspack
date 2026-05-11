// These follow Terser's `compress.builtins_pure` builtin symbol table. They
// are imported only for side effects and should not keep this module alive.
escape("%");
isFinite(1);
isNaN(NaN);
parseFloat("1.5");
parseInt("10", 10);

new Error();
new EvalError();
new Promise();
new Proxy();
new Symbol();

new Map([]);
new Set([]);

new ArrayBuffer([]);
new Float32Array([]);
new Int8Array([]);

Array.of(1, 2);
ArrayBuffer.isView(new Uint8Array());
Date.now();
Date.parse("2020-01-01");
Date.UTC(2020, 0, 1);
Math.sin(1);
Number.parseInt("1", 10);
Object.isExtensible({});
String.fromCharCode(65);

globalThis.isFinite(1);
globalThis.Math.cos(1);
new globalThis.Array();
new globalThis.Map([]);

Math.foo;
JSON.foo;
eval.foo;
setTimeout.foo;
