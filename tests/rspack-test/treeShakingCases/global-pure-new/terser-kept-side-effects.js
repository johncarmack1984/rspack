// Terser `builtins_pure` does not mark uppercase globals as pure when they are
// called without `new`. This module should therefore be kept.
const local = {};

Boolean(local);
Date(local);
Object(local);
new Object();
new WeakMap();
new WeakSet();
