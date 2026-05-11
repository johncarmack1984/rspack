//! Terser-compatible known-pure globals for tree-shaking.
//!
//! This models Terser's `compress.builtins_pure` tables for unresolved global
//! symbols. The option is intentionally opt-in because these tables are a
//! compatibility heuristic, not a proof that the runtime operation cannot throw.
//!
//! ## Safety invariants
//!
//! * **Shadowing**: the callee identifier must have the unresolved syntax context
//!   (`ctxt == unresolved_ctxt`), so a module-local `const Set = …` is never
//!   mistaken for the built-in.
//! * **Arguments**: only the small argument gates that Terser applies are
//!   modeled here. Argument expressions are still checked separately for side
//!   effects by the caller.

use swc_core::{
  common::SyntaxContext,
  ecma::ast::{Expr, Lit, MemberExpr, MemberProp},
};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Where the callee appears syntactically.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalleePosition {
  /// `new Callee(…)`
  New,
  /// `Callee(…)` or `Callee.method(…)`
  Call,
}

/// The argument rule required before a known global callee can be treated as
/// side-effect-free.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PureGlobalArgs {
  /// Every argument only needs to be side-effect-free.
  AnyPure,
  /// Terser's `new Array(number)` gate.
  ArrayConstructor,
  /// Terser's `new TypedArray(number | Array)` / `new ArrayBuffer(...)` gate.
  ArrayOrNumericLength,
  /// Terser's `new Map/Set(Array)` gate.
  ArrayIterableOrEmpty,
}

/// Classify `callee` as a known-pure global.
///
/// Returns an argument rule when:
/// 1. The callee resolves to an unresolved global (not a local binding).
/// 2. The name + `position` combination is in the allowlist.
pub fn classify_pure_global(
  callee: &Expr,
  unresolved_ctxt: SyntaxContext,
  position: CalleePosition,
) -> Option<PureGlobalArgs> {
  match callee {
    Expr::Ident(ident) if ident.ctxt == unresolved_ctxt => {
      classify_ident(ident.sym.as_str(), position)
    }
    Expr::Member(member) => {
      let callee = parse_member_callee(member, unresolved_ctxt)?;
      match callee {
        PureGlobalCallee::Direct(name) => classify_ident(name, position),
        PureGlobalCallee::Static { obj, prop } => classify_static_fn(obj, prop),
      }
    }
    _ => None,
  }
}

/// Classify direct global symbol/property reads that Terser treats as safe
/// access under `builtins_pure`.
pub fn is_pure_global_access(expr: &Expr, unresolved_ctxt: SyntaxContext) -> bool {
  match expr {
    Expr::Ident(ident) => {
      ident.ctxt == unresolved_ctxt && is_pure_access_global(ident.sym.as_str())
    }
    Expr::Member(member) => {
      is_static_property(&member.prop)
        && member
          .obj
          .as_ident()
          .is_some_and(|obj| obj.ctxt == unresolved_ctxt && is_pure_access_global(obj.sym.as_str()))
    }
    _ => false,
  }
}

// ---------------------------------------------------------------------------
// Internal classification tables
// ---------------------------------------------------------------------------

fn classify_ident(name: &str, position: CalleePosition) -> Option<PureGlobalArgs> {
  match position {
    CalleePosition::New => classify_new_ident(name),
    CalleePosition::Call => classify_call_ident(name),
  }
}

/// `new Name(…)`
fn classify_new_ident(name: &str) -> Option<PureGlobalArgs> {
  Some(match name {
    "Array" => PureGlobalArgs::ArrayConstructor,
    "ArrayBuffer" | "Float32Array" | "Float64Array" | "Int8Array" | "Int16Array" | "Int32Array"
    | "Uint8Array" | "Uint8ClampedArray" | "Uint16Array" | "Uint32Array" => {
      PureGlobalArgs::ArrayOrNumericLength
    }
    "Map" | "Set" => PureGlobalArgs::ArrayIterableOrEmpty,
    "BigInt"
    | "BigInt64Array"
    | "BigUint64Array"
    | "Boolean"
    | "Date"
    | "Error"
    | "EvalError"
    | "FinalizationRegistry"
    | "Float16Array"
    | "Iterator"
    | "Number"
    | "Promise"
    | "Proxy"
    | "RangeError"
    | "ReferenceError"
    | "String"
    | "Symbol"
    | "SyntaxError"
    | "TypeError"
    | "URIError" => PureGlobalArgs::AnyPure,
    _ => return None,
  })
}

/// `Name(…)`
fn classify_call_ident(name: &str) -> Option<PureGlobalArgs> {
  Some(match name {
    "escape" | "isFinite" | "isNaN" | "parseFloat" | "parseInt" => PureGlobalArgs::AnyPure,
    _ => return None,
  })
}

/// `Obj.method(…)` and `new Obj.method(…)`.
fn classify_static_fn(obj: &str, prop: &str) -> Option<PureGlobalArgs> {
  let is_known = match obj {
    "Array" => matches!(prop, "isArray" | "of"),
    "ArrayBuffer" => prop == "isView",
    "Date" => matches!(prop, "now" | "parse" | "UTC"),
    "Error" => prop == "isError",
    "Math" => matches!(
      prop,
      "abs"
        | "acos"
        | "acosh"
        | "asin"
        | "asinh"
        | "atan"
        | "atan2"
        | "atanh"
        | "cbrt"
        | "ceil"
        | "clz32"
        | "cos"
        | "cosh"
        | "exp"
        | "expm1"
        | "f16round"
        | "floor"
        | "fround"
        | "hypot"
        | "imul"
        | "log"
        | "log10"
        | "log1p"
        | "log2"
        | "max"
        | "min"
        | "pow"
        | "round"
        | "sign"
        | "sin"
        | "sinh"
        | "sqrt"
        | "tan"
        | "tanh"
        | "trunc"
    ),
    "Number" => matches!(
      prop,
      "isFinite" | "isInteger" | "isNaN" | "isSafeInteger" | "parseFloat" | "parseInt"
    ),
    "Object" => matches!(prop, "is" | "isExtensible" | "isFrozen" | "isSealed"),
    "Promise" => prop == "withResolvers",
    "String" => prop == "fromCharCode",
    "Uint16Array" | "Uint32Array" | "Uint8Array" | "Uint8ClampedArray" => prop == "of",
    _ => false,
  };
  if is_known {
    Some(PureGlobalArgs::AnyPure)
  } else {
    None
  }
}

enum PureGlobalCallee<'a> {
  Direct(&'a str),
  Static { obj: &'a str, prop: &'a str },
}

fn parse_member_callee<'a>(
  member: &'a MemberExpr,
  unresolved_ctxt: SyntaxContext,
) -> Option<PureGlobalCallee<'a>> {
  let prop = static_member_name(&member.prop)?;
  match member.obj.as_ref() {
    Expr::Ident(obj) if obj.ctxt == unresolved_ctxt => {
      if obj.sym.as_str() == "globalThis" {
        Some(PureGlobalCallee::Direct(prop))
      } else {
        Some(PureGlobalCallee::Static {
          obj: obj.sym.as_str(),
          prop,
        })
      }
    }
    Expr::Member(inner) => {
      let inner_prop = static_member_name(&inner.prop)?;
      let Expr::Ident(obj) = inner.obj.as_ref() else {
        return None;
      };
      if obj.ctxt == unresolved_ctxt && obj.sym.as_str() == "globalThis" {
        Some(PureGlobalCallee::Static {
          obj: inner_prop,
          prop,
        })
      } else {
        None
      }
    }
    _ => None,
  }
}

fn static_member_name(prop: &MemberProp) -> Option<&str> {
  match prop {
    MemberProp::Ident(ident) => Some(ident.sym.as_str()),
    _ => None,
  }
}

fn is_static_property(prop: &MemberProp) -> bool {
  match prop {
    MemberProp::Ident(_) => true,
    MemberProp::Computed(computed) => matches!(
      computed.expr.as_ref(),
      Expr::Lit(Lit::Str(_) | Lit::Num(_) | Lit::Bool(_) | Lit::Null(_))
    ),
    MemberProp::PrivateName(_) => false,
  }
}

fn is_pure_access_global(name: &str) -> bool {
  matches!(
    name,
    "Array"
      | "Boolean"
      | "Date"
      | "Error"
      | "EvalError"
      | "Function"
      | "JSON"
      | "Math"
      | "Number"
      | "Object"
      | "RangeError"
      | "ReferenceError"
      | "RegExp"
      | "String"
      | "SyntaxError"
      | "TypeError"
      | "URIError"
      | "clearInterval"
      | "clearTimeout"
      | "console"
      | "decodeURI"
      | "decodeURIComponent"
      | "encodeURI"
      | "encodeURIComponent"
      | "escape"
      | "eval"
      | "globalThis"
      | "isFinite"
      | "isNaN"
      | "parseFloat"
      | "parseInt"
      | "setInterval"
      | "setTimeout"
      | "unescape"
  )
}
