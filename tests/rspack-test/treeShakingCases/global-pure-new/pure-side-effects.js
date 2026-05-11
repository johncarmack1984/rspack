// This module is imported only for side effects. With builtinPureGlobals enabled,
// all top-level statements below follow Terser's builtins_pure table and the
// whole module should be dropped.
new Set();
new Map();
new Uint8Array(16);
Array.isArray([1, 2, 3]);
Object.is(1, 2);
new Number(1n);
new String(/x/);
