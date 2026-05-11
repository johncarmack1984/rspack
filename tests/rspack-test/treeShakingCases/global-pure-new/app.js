import "./shadow";

export const marker = 1;

// === Should be tree-shaken by the side-effects heuristic ===

// Terser builtins_pure: collections with no args.
let unusedSet = new Set();
let unusedMap = new Map();

// TypedArrays with Terser's numeric range gate.
let unusedTyped = new Uint8Array(16);
let unusedTypedFractional = new Uint8Array(1.5);
let unusedBuf = new ArrayBuffer(0);

// Pure static functions (args themselves must be pure too).
let unusedArrIsArray = Array.isArray([1, 2, 3]);
let unusedObjectIs = Object.is(1, 2);

// Constructors Terser marks as pure under builtins_pure.
let unusedNumberBigInt = new Number(1n);
let unusedStringObject = new String(/x/);
let unusedDateBigInt = new Date(1n);
let unusedArrayFractional = new Array(1.5);

function impureArg() { console.log("keep"); return 1; }

// === MUST be kept by the Terser-compatible heuristic ===

let unusedSetLiteral = new Set(1);
let unusedNullSet = new Set(null);
let unusedMapLiteral = new Map("foo");
let unusedUndefMap = new Map(undefined);
let unusedWeakMap = new WeakMap();
let unusedWeakSet = new WeakSet();
let unusedArrayNegative = new Array(-1);
let unusedTypedNegative = new Uint8Array(-1);

// Uppercase globals are not pure in Terser builtins_pure when called without
// `new`.
let unusedString = String("hello");
let unusedObject = Object("y");
let unusedBool = Boolean({});
let unusedBoolVar = Boolean(marker);
let unusedSymbol = Symbol("desc");

// RegExp literals are pure argument expressions, but coercing them can invoke
// user code through RegExp.prototype.toString.
RegExp.prototype.toString = impureArg;
let unusedRegexToString = String(/x/);
let unusedRegexSymbolDesc = Symbol(/x/);

let dynamic = { length: 16 };
let unusedWithDynamic = new Uint8Array(dynamic);

// Impure nested arguments are still kept.

let unusedWithImpureArg = new Set([impureArg()]);
let unusedBoolImpure = Boolean(impureArg());
