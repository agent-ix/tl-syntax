//! Independent generator and gate for `tl-syntax.numeric-corpus/v1` (FR-305).
//!
//! It shares no code with the crate: expression types come from its own
//! interval arithmetic and predicate truth from exact decimal arithmetic in
//! i128 (every value is `coefficient * 10^-scale`, so aligning scales gives
//! exact comparison). It replaces the retained Python generator (review H-1;
//! STD-228 keeps all executable logic in Rust).
//!
//! `cargo run --example numeric_corpus --features serde` regenerates the corpus
//! in memory and fails unless every generated file equals the committed bytes.
//! `-- --write` rewrites `corpus/numeric/` instead. The corpus is immutable by
//! version identity: any change to a case needs `numeric-corpus/v2`.

use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use sha2::{Digest, Sha256};

const I64_MIN: i128 = i64::MIN as i128;
const I64_MAX: i128 = i64::MAX as i128;

// ------------------------------------------------------------------ JSON ---

/// An ordered JSON value; objects keep insertion order like Python dicts.
#[derive(Clone, Debug)]
enum J {
    Int(i128),
    Bool(bool),
    Str(String),
    Arr(Vec<J>),
    Obj(Vec<(String, J)>),
}

fn obj(entries: Vec<(&str, J)>) -> J {
    J::Obj(
        entries
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value))
            .collect(),
    )
}

fn s(text: &str) -> J {
    J::Str(text.to_owned())
}

fn int(value: impl Into<i128>) -> J {
    J::Int(value.into())
}

impl J {
    fn get(&self, key: &str) -> Option<&J> {
        match self {
            J::Obj(entries) => entries
                .iter()
                .find(|(name, _)| name == key)
                .map(|(_, value)| value),
            _ => None,
        }
    }

    fn int(&self, key: &str) -> i128 {
        match self.get(key) {
            Some(J::Int(value)) => *value,
            other => panic!("{key} is not an integer: {other:?}"),
        }
    }

    fn text(&self, key: &str) -> &str {
        match self.get(key) {
            Some(J::Str(value)) => value,
            other => panic!("{key} is not a string: {other:?}"),
        }
    }

    fn items(&self, key: &str) -> &[J] {
        match self.get(key) {
            Some(J::Arr(items)) => items,
            other => panic!("{key} is not an array: {other:?}"),
        }
    }
}

fn quote(text: &str, out: &mut String) {
    out.push('"');
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            other => out.push(other),
        }
    }
    out.push('"');
}

/// Python `json.dumps(value, separators=(",", ":"), ensure_ascii=False)`.
fn compact(value: &J, out: &mut String) {
    match value {
        J::Int(number) => out.push_str(&number.to_string()),
        J::Bool(flag) => out.push_str(if *flag { "true" } else { "false" }),
        J::Str(text) => quote(text, out),
        J::Arr(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                compact(item, out);
            }
            out.push(']');
        }
        J::Obj(entries) => {
            out.push('{');
            for (index, (key, item)) in entries.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                quote(key, out);
                out.push(':');
                compact(item, out);
            }
            out.push('}');
        }
    }
}

fn canonical(value: &J) -> String {
    let mut out = String::new();
    compact(value, &mut out);
    out
}

/// Python `json.dumps(value, indent=2)`.
fn pretty(value: &J, level: usize, out: &mut String) {
    let pad = |level: usize| "  ".repeat(level);
    match value {
        J::Arr(items) if items.is_empty() => out.push_str("[]"),
        J::Obj(entries) if entries.is_empty() => out.push_str("{}"),
        J::Arr(items) => {
            out.push_str("[\n");
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push_str(",\n");
                }
                out.push_str(&pad(level + 1));
                pretty(item, level + 1, out);
            }
            out.push('\n');
            out.push_str(&pad(level));
            out.push(']');
        }
        J::Obj(entries) => {
            out.push_str("{\n");
            for (index, (key, item)) in entries.iter().enumerate() {
                if index > 0 {
                    out.push_str(",\n");
                }
                out.push_str(&pad(level + 1));
                quote(key, out);
                out.push_str(": ");
                pretty(item, level + 1, out);
            }
            out.push('\n');
            out.push_str(&pad(level));
            out.push('}');
        }
        scalar => compact(scalar, out),
    }
}

// -------------------------------------------------------------- catalogs ---

fn signal_decl(id: i128, name: &str, domain: J) -> J {
    obj(vec![("id", int(id)), ("name", s(name)), ("domain", domain)])
}

fn integer_domain(minimum: i128, maximum: i128) -> J {
    obj(vec![
        ("kind", s("integer")),
        ("minimum", int(minimum)),
        ("maximum", int(maximum)),
    ])
}

fn decimal_domain(minimum: i128, maximum: i128, scale: i128) -> J {
    obj(vec![
        ("kind", s("fixed_decimal")),
        ("minimum_coefficient", int(minimum)),
        ("maximum_coefficient", int(maximum)),
        ("scale", int(scale)),
    ])
}

fn catalog() -> J {
    obj(vec![
        ("schema_version", s("tl-syntax.signal-catalog/v1")),
        (
            "signals",
            J::Arr(vec![
                signal_decl(1, "flag", obj(vec![("kind", s("boolean"))])),
                signal_decl(2, "count", integer_domain(-5, 7)),
                signal_decl(3, "temp", decimal_domain(-400, 1250, 1)),
                signal_decl(4, "gain", decimal_domain(-3, 2, 2)),
                signal_decl(5, "big", decimal_domain(1, 3, 9)),
                signal_decl(6, "drop", integer_domain(-9, -2)),
            ]),
        ),
        (
            "bindings",
            J::Arr(vec![obj(vec![("proposition", int(1)), ("signal", int(1))])]),
        ),
    ])
}

/// Numeric domains of [`catalog`]: signal -> (scale, minimum, maximum).
fn domains() -> BTreeMap<i128, (u32, i128, i128)> {
    BTreeMap::from([
        (2, (0, -5, 7)),
        (3, (1, -400, 1250)),
        (4, (2, -3, 2)),
        (5, (9, 1, 3)),
        (6, (0, -9, -2)),
    ])
}

const DECLARED: [i128; 6] = [1, 2, 3, 4, 5, 6];

// ---------------------------------------------------------------- typing ---

type Ty = (u32, i128, i128);

fn fits(value: i128) -> bool {
    (I64_MIN..=I64_MAX).contains(&value)
}

fn operands(node: &J) -> Vec<usize> {
    ["operand", "left", "right"]
        .iter()
        .filter_map(|key| node.get(key).map(|_| node.int(key) as usize))
        .collect()
}

fn derive(expressions: &[J]) -> Result<Vec<Ty>, &'static str> {
    let domains = domains();
    let mut types: Vec<Ty> = Vec::new();
    for node in expressions {
        let kind = node.text("kind");
        let ty = match kind {
            "signal" => {
                let signal = node.int("signal");
                match domains.get(&signal) {
                    Some(ty) => *ty,
                    None if DECLARED.contains(&signal) => return Err("NonNumericSignal"),
                    None => return Err("UnknownSignal"),
                }
            }
            "integer" => (0, node.int("value"), node.int("value")),
            "decimal" => {
                if node.int("scale") > 18 {
                    return Err("ScaleOverflow");
                }
                let coefficient = node.int("coefficient");
                (node.int("scale") as u32, coefficient, coefficient)
            }
            "slot_signal" => return Err("SlotOutsideTemplate"),
            _ => {
                if operands(node).iter().any(|operand| *operand >= types.len()) {
                    return Err("OperandNotPreceding");
                }
                let at = |key: &str| types[node.int(key) as usize];
                match kind {
                    "neg" => {
                        let (scale, low, high) = at("operand");
                        (scale, -high, -low)
                    }
                    "abs" => {
                        let (scale, low, high) = at("operand");
                        let upper = low.abs().max(high.abs());
                        let lower = if low <= 0 && 0 <= high {
                            0
                        } else {
                            low.abs().min(high.abs())
                        };
                        (scale, lower, upper)
                    }
                    "add" | "sub" => {
                        let ((s1, l1, h1), (s2, l2, h2)) = (at("left"), at("right"));
                        if s1 != s2 {
                            return Err("ScaleMismatch");
                        }
                        if kind == "add" {
                            (s1, l1 + l2, h1 + h2)
                        } else {
                            (s1, l1 - h2, h1 - l2)
                        }
                    }
                    "mul" => {
                        let ((s1, l1, h1), (s2, l2, h2)) = (at("left"), at("right"));
                        if s1 + s2 > 18 {
                            return Err("ScaleOverflow");
                        }
                        let corners = [l1 * l2, l1 * h2, h1 * l2, h1 * h2];
                        let low = corners.iter().copied().min().unwrap_or(0);
                        let high = corners.iter().copied().max().unwrap_or(0);
                        (s1 + s2, low, high)
                    }
                    "rescale" => {
                        let (scale, low, high) = at("operand");
                        let target = node.int("scale") as u32;
                        if target > 18 {
                            return Err("ScaleOverflow");
                        }
                        if target < scale {
                            return Err("RescaleNarrowing");
                        }
                        let factor = 10_i128.pow(target - scale);
                        (target, low * factor, high * factor)
                    }
                    other => panic!("unknown expression kind {other}"),
                }
            }
        };
        if !(fits(ty.1) && fits(ty.2)) {
            return Err("RangeOverflow");
        }
        types.push(ty);
    }
    Ok(types)
}

fn check_table(table: &J) -> Result<Vec<Ty>, &'static str> {
    let expressions = table.items("expressions");
    let types = derive(expressions)?;
    let mut depth: Vec<usize> = Vec::new();
    for node in expressions {
        let deepest = operands(node).iter().map(|operand| depth[*operand]).max();
        depth.push(1 + deepest.unwrap_or(0));
        if depth[depth.len() - 1] > 256 {
            return Err("ExpressionLimitExceeded");
        }
    }
    let mut previous = None;
    for predicate in table.items("predicates") {
        let proposition = predicate.int("proposition");
        if previous.is_some_and(|previous| previous >= proposition) {
            return Err("PropositionNotIncreasing");
        }
        previous = Some(proposition);
        for key in ["left", "right"] {
            if predicate.int(key) as usize >= types.len() {
                return Err("RootOutOfRange");
            }
        }
        let left = types[predicate.int("left") as usize].0;
        let right = types[predicate.int("right") as usize].0;
        if left != right {
            return Err("PredicateScaleMismatch");
        }
    }
    Ok(types)
}

// ----------------------------------------------------------------- truth ---

/// An exact decimal `coefficient * 10^-scale`.
#[derive(Clone, Copy)]
struct Decimal {
    coefficient: i128,
    scale: u32,
}

impl Decimal {
    fn at_scale(self, scale: u32) -> i128 {
        self.coefficient
            .checked_mul(10_i128.pow(scale - self.scale))
            .expect("exact decimal fits i128")
    }

    fn align(self, other: Self) -> (i128, i128, u32) {
        let scale = self.scale.max(other.scale);
        (self.at_scale(scale), other.at_scale(scale), scale)
    }
}

fn evaluate(expressions: &[J], index: usize, valuation: &BTreeMap<i128, i128>) -> Decimal {
    let node = &expressions[index];
    let at = |key: &str| evaluate(expressions, node.int(key) as usize, valuation);
    match node.text("kind") {
        "signal" => {
            let signal = node.int("signal");
            Decimal {
                coefficient: valuation[&signal],
                scale: domains()[&signal].0,
            }
        }
        "integer" => Decimal {
            coefficient: node.int("value"),
            scale: 0,
        },
        "decimal" => Decimal {
            coefficient: node.int("coefficient"),
            scale: node.int("scale") as u32,
        },
        "neg" => {
            let value = at("operand");
            Decimal {
                coefficient: -value.coefficient,
                ..value
            }
        }
        "abs" => {
            let value = at("operand");
            Decimal {
                coefficient: value.coefficient.abs(),
                ..value
            }
        }
        // Rescale changes the representation, not the value.
        "rescale" => at("operand"),
        kind => {
            let (left, right) = (at("left"), at("right"));
            match kind {
                "mul" => Decimal {
                    coefficient: left.coefficient * right.coefficient,
                    scale: left.scale + right.scale,
                },
                "add" | "sub" => {
                    let (a, b, scale) = left.align(right);
                    Decimal {
                        coefficient: if kind == "add" { a + b } else { a - b },
                        scale,
                    }
                }
                other => panic!("unknown expression kind {other}"),
            }
        }
    }
}

fn compare(operator: &str, left: Decimal, right: Decimal) -> bool {
    let (a, b, _) = left.align(right);
    match operator {
        "lt" => a < b,
        "le" => a <= b,
        "gt" => a > b,
        "ge" => a >= b,
        "eq" => a == b,
        "ne" => a != b,
        other => panic!("unknown operator {other}"),
    }
}

fn signals_read(expressions: &[J]) -> Vec<i128> {
    let mut signals: Vec<i128> = expressions
        .iter()
        .filter(|node| node.text("kind") == "signal")
        .map(|node| node.int("signal"))
        .collect();
    signals.sort_unstable();
    signals.dedup();
    signals
}

fn sample_values(signal: i128) -> Vec<i128> {
    let (_, low, high) = domains()[&signal];
    let mut values = vec![low, high];
    if low <= 0 && 0 <= high {
        values.push(0);
    }
    values.sort_unstable();
    values.dedup();
    values
}

/// Rows in `itertools.product` order over the ascending signals.
fn truth_table(table: &J) -> J {
    let expressions = table.items("expressions");
    let signals = signals_read(expressions);
    let samples: Vec<Vec<i128>> = signals.iter().map(|signal| sample_values(*signal)).collect();
    let mut combinations: Vec<Vec<i128>> = vec![Vec::new()];
    for values in &samples {
        let mut next = Vec::new();
        for prefix in &combinations {
            for value in values {
                let mut row = prefix.clone();
                row.push(*value);
                next.push(row);
            }
        }
        combinations = next;
    }
    let mut rows = Vec::new();
    for combination in combinations {
        let valuation: BTreeMap<i128, i128> =
            signals.iter().copied().zip(combination.iter().copied()).collect();
        let truth = table
            .items("predicates")
            .iter()
            .map(|predicate| {
                let left = evaluate(expressions, predicate.int("left") as usize, &valuation);
                let right = evaluate(expressions, predicate.int("right") as usize, &valuation);
                obj(vec![
                    ("proposition", int(predicate.int("proposition"))),
                    ("value", J::Bool(compare(predicate.text("operator"), left, right))),
                ])
            })
            .collect();
        rows.push(obj(vec![
            (
                "valuation",
                J::Arr(
                    signals
                        .iter()
                        .map(|signal| {
                            obj(vec![
                                ("signal", int(*signal)),
                                ("coefficient", int(valuation[signal])),
                            ])
                        })
                        .collect(),
                ),
            ),
            ("truth", J::Arr(truth)),
        ]));
    }
    J::Arr(rows)
}

// ----------------------------------------------------------------- cases ---

fn sig(signal: i128) -> J {
    obj(vec![("kind", s("signal")), ("signal", int(signal))])
}

fn integer(value: i128) -> J {
    obj(vec![("kind", s("integer")), ("value", int(value))])
}

fn decimal(coefficient: i128, scale: i128) -> J {
    obj(vec![
        ("kind", s("decimal")),
        ("coefficient", int(coefficient)),
        ("scale", int(scale)),
    ])
}

fn unary(kind: &str, operand: i128) -> J {
    obj(vec![("kind", s(kind)), ("operand", int(operand))])
}

fn binary(kind: &str, left: i128, right: i128) -> J {
    obj(vec![("kind", s(kind)), ("left", int(left)), ("right", int(right))])
}

fn rescale(operand: i128, scale: i128) -> J {
    obj(vec![
        ("kind", s("rescale")),
        ("operand", int(operand)),
        ("scale", int(scale)),
    ])
}

fn pred(proposition: i128, operator: &str, left: i128, right: i128) -> J {
    obj(vec![
        ("proposition", int(proposition)),
        ("operator", s(operator)),
        ("left", int(left)),
        ("right", int(right)),
    ])
}

fn table(expressions: Vec<J>, predicates: Vec<J>) -> J {
    obj(vec![
        ("schema_version", s("tl-syntax.predicate-table/v1")),
        ("expressions", J::Arr(expressions)),
        ("predicates", J::Arr(predicates)),
    ])
}

fn valid_cases() -> Vec<(&'static str, J)> {
    vec![
        (
            "every-node-and-operator",
            table(
                vec![
                    sig(2),
                    integer(3),
                    binary("add", 0, 1),
                    binary("sub", 0, 1),
                    unary("neg", 0),
                    unary("abs", 0),
                    binary("mul", 0, 1),
                    sig(3),
                    rescale(0, 1),
                    decimal(255, 1),
                    sig(4),
                    binary("mul", 10, 7),
                    unary("abs", 10),
                    decimal(0, 3),
                    sig(6),
                    unary("abs", 14),
                ],
                vec![
                    pred(10, "lt", 0, 1),
                    pred(11, "le", 2, 1),
                    pred(12, "gt", 7, 9),
                    pred(13, "ge", 8, 9),
                    pred(14, "eq", 4, 5),
                    pred(15, "ne", 6, 3),
                    pred(16, "lt", 11, 13),
                    pred(17, "le", 12, 10),
                    pred(18, "gt", 15, 1),
                ],
            ),
        ),
        (
            "scales-0-1-2-9-18",
            table(
                vec![
                    sig(5),
                    binary("mul", 0, 0),
                    decimal(4, 18),
                    rescale(0, 18),
                    sig(2),
                    decimal(5, 1),
                    sig(4),
                    decimal(1, 9),
                ],
                vec![
                    pred(20, "gt", 1, 2),
                    pred(21, "lt", 3, 1),
                    pred(22, "ge", 4, 4),
                    pred(23, "ne", 6, 6),
                    pred(24, "le", 0, 7),
                ],
            ),
        ),
    ]
}

fn refused_cases() -> Vec<(&'static str, J, &'static str)> {
    let mut chain = vec![integer(0)];
    chain.extend((0..256).map(|index| unary("neg", index)));
    vec![
        (
            "range-overflow",
            table(vec![integer(I64_MAX), integer(1), binary("add", 0, 1)], vec![]),
            "RangeOverflow",
        ),
        (
            "scale-mismatch",
            table(vec![sig(2), sig(3), binary("add", 0, 1)], vec![]),
            "ScaleMismatch",
        ),
        (
            "scale-overflow",
            table(vec![decimal(1, 18), decimal(1, 1), binary("mul", 0, 1)], vec![]),
            "ScaleOverflow",
        ),
        (
            "rescale-narrowing",
            table(vec![sig(3), rescale(0, 0)], vec![]),
            "RescaleNarrowing",
        ),
        ("non-numeric-signal", table(vec![sig(1)], vec![]), "NonNumericSignal"),
        ("unknown-signal", table(vec![sig(99)], vec![]), "UnknownSignal"),
        ("expression-depth-limit", table(chain, vec![]), "ExpressionLimitExceeded"),
        (
            "operand-not-preceding",
            table(vec![unary("neg", 0)], vec![]),
            "OperandNotPreceding",
        ),
        (
            "slot-outside-template",
            table(
                vec![obj(vec![("kind", s("slot_signal")), ("depth", int(0))])],
                vec![],
            ),
            "SlotOutsideTemplate",
        ),
        (
            "predicate-scale-mismatch",
            table(vec![sig(2), sig(3)], vec![pred(1, "lt", 0, 1)]),
            "PredicateScaleMismatch",
        ),
        (
            "proposition-not-increasing",
            table(vec![sig(2)], vec![pred(9, "lt", 0, 0), pred(9, "gt", 0, 0)]),
            "PropositionNotIncreasing",
        ),
        (
            "root-out-of-range",
            table(vec![sig(2)], vec![pred(9, "lt", 0, 5)]),
            "RootOutOfRange",
        ),
    ]
}

fn wire_cases() -> Vec<(&'static str, String, &'static str)> {
    let base = canonical(&table(vec![sig(2), integer(3)], vec![pred(9, "lt", 0, 1)]));
    let swap = |to: &str| base.replace("\"value\":3", to);
    vec![
        ("float", swap("\"value\":3.0"), "InvalidDocument"),
        ("exponent", swap("\"value\":3e0"), "InvalidDocument"),
        ("negative-zero", swap("\"value\":-0"), "NonCanonicalDocument"),
        ("leading-zero", swap("\"value\":03"), "InvalidDocument"),
        ("duplicate-member", swap("\"value\":3,\"value\":3"), "InvalidDocument"),
        ("unknown-member", swap("\"value\":3,\"unit\":\"m\""), "InvalidDocument"),
        ("trailing-data", format!("{base} {{}}"), "InvalidDocument"),
    ]
}

// ----------------------------------------------------------- aggregation ---
// FR-308: Boolean members, trace length 1, k in 0..=5, n in 0..=6. The
// hand-written forms are built independently of the crate's expansion
// (left-leaning chains and a disjunction of all n-subsets).

fn boolean_catalog() -> J {
    obj(vec![
        ("schema_version", s("tl-syntax.signal-catalog/v1")),
        (
            "signals",
            J::Arr(
                (1..=5)
                    .map(|id| signal_decl(id, &format!("b{id}"), obj(vec![("kind", s("boolean"))])))
                    .collect(),
            ),
        ),
        (
            "bindings",
            J::Arr(
                (1..=5)
                    .map(|id| obj(vec![("proposition", int(id)), ("signal", int(id))]))
                    .collect(),
            ),
        ),
    ])
}

fn chain_nodes(nodes: &mut Vec<J>, leaves: &[usize], kind: &str) -> usize {
    let mut current = leaves[0];
    for leaf in &leaves[1..] {
        nodes.push(binary(kind, current as i128, *leaf as i128));
        current = nodes.len() - 1;
    }
    current
}

/// Every `n`-subset of `members`, in `itertools.combinations` order.
fn combinations(members: &[i128], n: usize) -> Vec<Vec<i128>> {
    if n == 0 {
        return vec![Vec::new()];
    }
    let mut out = Vec::new();
    for (index, first) in members.iter().enumerate() {
        for mut rest in combinations(&members[index + 1..], n - 1) {
            rest.insert(0, *first);
            out.push(rest);
        }
    }
    out
}

fn handwritten(quantifier: &str, k: usize, n: usize) -> J {
    let mut nodes: Vec<J> = Vec::new();
    let constant = |nodes: &mut Vec<J>, value: bool| {
        nodes.push(obj(vec![("kind", s(if value { "true" } else { "false" }))]));
        nodes.len() - 1
    };
    let atoms = |nodes: &mut Vec<J>, members: &[i128]| -> Vec<usize> {
        members
            .iter()
            .map(|member| {
                nodes.push(obj(vec![
                    ("kind", s("proposition")),
                    ("proposition", int(*member)),
                ]));
                nodes.len() - 1
            })
            .collect()
    };
    let members: Vec<i128> = (1..=k as i128).collect();
    let root = match quantifier {
        "for_each" if k == 0 => constant(&mut nodes, true),
        "for_each" => {
            let leaves = atoms(&mut nodes, &members);
            chain_nodes(&mut nodes, &leaves, "and")
        }
        "for_some" if k == 0 => constant(&mut nodes, false),
        "for_some" => {
            let leaves = atoms(&mut nodes, &members);
            chain_nodes(&mut nodes, &leaves, "or")
        }
        _ if n == 0 => constant(&mut nodes, true),
        _ if n > k => constant(&mut nodes, false),
        _ => {
            let mut terms = Vec::new();
            for subset in combinations(&members, n) {
                let leaves = atoms(&mut nodes, &subset);
                terms.push(chain_nodes(&mut nodes, &leaves, "and"));
            }
            chain_nodes(&mut nodes, &terms, "or")
        }
    };
    obj(vec![
        ("schema_version", s("tl-syntax.formula/v1")),
        ("semantic_profile", s("mltl.closed-trace/v1")),
        ("root", int(root as i128)),
        ("nodes", J::Arr(nodes)),
    ])
}

fn aggregation_document(quantifier: &str, k: usize, n: usize) -> J {
    let quantifier = if quantifier == "at_least" {
        obj(vec![("kind", s("at_least")), ("n", int(n as i128))])
    } else {
        obj(vec![("kind", s(quantifier))])
    };
    let template = obj(vec![
        (
            "nodes",
            J::Arr(vec![obj(vec![
                ("kind", s("slot_proposition")),
                ("depth", int(0)),
            ])]),
        ),
        ("root", int(0)),
        ("expressions", J::Arr(vec![])),
        ("predicates", J::Arr(vec![])),
    ]);
    obj(vec![
        ("schema_version", s("tl-syntax.set-aggregation/v1")),
        (
            "sets",
            J::Arr(vec![obj(vec![
                ("name", s("members")),
                ("members", J::Arr((1..=k as i128).map(int).collect())),
            ])]),
        ),
        ("first_proposition", int(100)),
        (
            "aggregations",
            J::Arr(vec![obj(vec![
                ("set", s("members")),
                ("quantifier", quantifier),
                ("template", template),
            ])]),
        ),
    ])
}

fn aggregation_cases() -> Vec<(String, &'static str, usize, usize)> {
    let mut grid: Vec<(&'static str, usize, usize)> = Vec::new();
    grid.extend((0..6).map(|k| ("for_each", k, 0)));
    grid.extend((0..6).map(|k| ("for_some", k, 0)));
    for k in 0..6 {
        grid.extend((0..7).map(|n| ("at_least", k, n)));
    }
    grid.into_iter()
        .map(|(quantifier, k, n)| {
            let suffix = if quantifier == "at_least" {
                format!("-n{n}")
            } else {
                String::new()
            };
            (format!("{quantifier}-k{k}{suffix}"), quantifier, k, n)
        })
        .collect()
}

// ------------------------------------------------------------- contracts ---
// FR-309: one formula/v2 graph and contract sets bound to it by the
// CONTENT_IDENTITY_DOMAIN_V1 identity (ADR-019 register item 3, retained).

fn interval(start: i128, end: i128) -> J {
    obj(vec![("start", int(start)), ("end", int(end))])
}

fn contract_formula() -> J {
    obj(vec![
        ("schema_version", s("tl-syntax.formula/v2")),
        ("semantic_profile", s("mltl.closed-trace/v1")),
        ("root", int(4)),
        (
            "nodes",
            J::Arr(vec![
                obj(vec![("kind", s("proposition")), ("proposition", int(1))]),
                obj(vec![("kind", s("proposition")), ("proposition", int(2))]),
                obj(vec![
                    ("kind", s("future")),
                    ("interval", interval(0, 2)),
                    ("operand", int(0)),
                ]),
                obj(vec![
                    ("kind", s("globally")),
                    ("interval", interval(0, 1)),
                    ("operand", int(1)),
                ]),
                binary("and", 2, 3),
            ]),
        ),
    ])
}

fn content_identity(contract: &str, text: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(b"tl-syntax.content-identity/v1\0");
    digest.update(contract.as_bytes());
    digest.update([0]);
    digest.update(text.as_bytes());
    format!("{:x}", digest.finalize())
}

fn contract_set(identity: &str, contracts: &[(i128, &str, i128, i128)], profile: &str) -> J {
    obj(vec![
        ("schema_version", s("tl-syntax.contract-set/v1")),
        ("semantic_profile", s(profile)),
        ("graph_identity", s(identity)),
        (
            "contracts",
            J::Arr(
                contracts
                    .iter()
                    .map(|(id, name, assumption, guarantee)| {
                        obj(vec![
                            ("id", int(*id)),
                            ("name", s(name)),
                            ("assumption", int(*assumption)),
                            ("guarantee", int(*guarantee)),
                        ])
                    })
                    .collect(),
            ),
        ),
    ])
}

fn contract_cases() -> Vec<(&'static str, J, &'static str)> {
    let identity = content_identity("tl-syntax.formula/v2", &canonical(&contract_formula()));
    let closed = "mltl.closed-trace/v1";
    let good = [(1, "door-alarm", 2, 3), (2, "same-root", 2, 2)];
    let zeros = "0".repeat(64);
    vec![
        ("valid-shared-roots", contract_set(&identity, &good, closed), "valid"),
        ("valid-empty", contract_set(&identity, &[], closed), "valid"),
        (
            "root-out-of-range",
            contract_set(&identity, &[(1, "a", 9, 3)], closed),
            "RootOutOfRange",
        ),
        (
            "duplicate-name",
            contract_set(&identity, &[(1, "a", 2, 3), (2, "a", 2, 3)], closed),
            "DuplicateName",
        ),
        (
            "id-not-increasing",
            contract_set(&identity, &[(2, "a", 2, 3), (2, "b", 2, 3)], closed),
            "ContractIdNotIncreasing",
        ),
        (
            "empty-name",
            contract_set(&identity, &[(1, "", 2, 3)], closed),
            "EmptyName",
        ),
        (
            "graph-identity-mismatch",
            contract_set(&zeros, &good, closed),
            "GraphIdentityMismatch",
        ),
        (
            "profile-mismatch",
            contract_set(&identity, &good, "mltl.online-prefix/v1"),
            "ProfileMismatch",
        ),
    ]
}

// --------------------------------------------------------------- corpus ---

/// The whole corpus: relative path -> exact bytes, plus the manifest.
fn generate() -> BTreeMap<String, String> {
    let mut files = BTreeMap::new();
    let mut cases: Vec<J> = Vec::new();
    files.insert(
        "catalogs/catalog-1.json".to_owned(),
        canonical(&catalog()) + "\n",
    );
    for (name, value) in valid_cases() {
        let types = check_table(&value).unwrap_or_else(|class| panic!("{name}: {class}"));
        files.insert(format!("valid/{name}.json"), canonical(&value));
        files.insert(format!("truth/{name}.json"), canonical(&truth_table(&value)) + "\n");
        cases.push(obj(vec![
            ("id", s(name)),
            ("class", s("valid")),
            ("catalog", s("catalogs/catalog-1.json")),
            ("table", J::Str(format!("valid/{name}.json"))),
            (
                "expected_types",
                J::Arr(
                    types
                        .iter()
                        .map(|(scale, low, high)| J::Arr(vec![int(*scale), int(*low), int(*high)]))
                        .collect(),
                ),
            ),
            ("truth", J::Str(format!("truth/{name}.json"))),
        ]));
    }
    for (name, value, refusal) in refused_cases() {
        assert_eq!(check_table(&value).err(), Some(refusal), "{name}");
        files.insert(format!("refused/{name}.json"), canonical(&value));
        cases.push(obj(vec![
            ("id", s(name)),
            ("class", s("refused-table")),
            ("catalog", s("catalogs/catalog-1.json")),
            ("table", J::Str(format!("refused/{name}.json"))),
            ("refusal", s(refusal)),
        ]));
    }
    for (name, text, refusal) in wire_cases() {
        files.insert(format!("wire/{name}.json"), text);
        cases.push(obj(vec![
            ("id", J::Str(format!("wire-{name}"))),
            ("class", s("refused-wire")),
            ("catalog", s("catalogs/catalog-1.json")),
            ("table", J::Str(format!("wire/{name}.json"))),
            ("refusal", s(refusal)),
        ]));
    }
    files.insert(
        "catalogs/boolean-members.json".to_owned(),
        canonical(&boolean_catalog()) + "\n",
    );
    let mut compared = 0_i128;
    for (name, quantifier, k, n) in aggregation_cases() {
        files.insert(
            format!("aggregation/{name}.json"),
            canonical(&aggregation_document(quantifier, k, n)),
        );
        files.insert(
            format!("handwritten/{name}.json"),
            canonical(&handwritten(quantifier, k, n)) + "\n",
        );
        let traces = 1_i128 << k;
        compared += traces;
        cases.push(obj(vec![
            ("id", J::Str(format!("aggregation-{name}"))),
            ("class", s("aggregation")),
            ("catalog", s("catalogs/boolean-members.json")),
            ("document", J::Str(format!("aggregation/{name}.json"))),
            ("handwritten", J::Str(format!("handwritten/{name}.json"))),
            (
                "population",
                obj(vec![
                    ("members", int(k as i128)),
                    ("valuations", J::Arr(vec![J::Bool(false), J::Bool(true)])),
                    ("trace_length", int(1)),
                ]),
            ),
            ("traces", int(traces)),
        ]));
    }
    assert_eq!(compared, 567);
    files.insert(
        "contracts/formula.json".to_owned(),
        canonical(&contract_formula()),
    );
    for (name, document, outcome) in contract_cases() {
        files.insert(format!("contracts/{name}.json"), canonical(&document));
        cases.push(obj(vec![
            ("id", J::Str(format!("contract-{name}"))),
            ("class", s("contract-set")),
            ("formula", s("contracts/formula.json")),
            ("document", J::Str(format!("contracts/{name}.json"))),
            ("outcome", s(outcome)),
        ]));
    }
    let mut counts: Vec<(String, J)> = Vec::new();
    for case in &cases {
        let class = case.text("class").to_owned();
        match counts.iter_mut().find(|(name, _)| *name == class) {
            Some((_, J::Int(count))) => *count += 1,
            _ => counts.push((class, int(1))),
        }
    }
    let manifest = obj(vec![
        ("corpus", s("tl-syntax.numeric-corpus/v1")),
        ("role", s("evidence-input")),
        ("predicate_table", s("tl-syntax.predicate-table/v1")),
        ("generator", s("examples/numeric_corpus.rs")),
        ("class_counts", J::Obj(counts)),
        ("aggregation_compared_traces", int(compared)),
        ("cases", J::Arr(cases)),
    ]);
    let mut text = String::new();
    pretty(&manifest, 0, &mut text);
    text.push('\n');
    files.insert("manifest.json".to_owned(), text);
    files
}

fn corpus_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus/numeric")
}

/// Every generated file must equal the committed bytes.
fn differences(root: &Path, files: &BTreeMap<String, String>) -> Vec<String> {
    files
        .iter()
        .filter_map(|(path, text)| match fs::read(root.join(path)) {
            Ok(bytes) if bytes == text.as_bytes() => None,
            Ok(_) => Some(format!("{path}: committed bytes differ from the generator")),
            Err(error) => Some(format!("{path}: {error}")),
        })
        .collect()
}

fn main() -> ExitCode {
    let files = generate();
    let root = corpus_root();
    if env::args().any(|argument| argument == "--write") {
        for (path, text) in &files {
            let target = root.join(path);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent).expect("create corpus directory");
            }
            fs::write(&target, text).expect("write corpus file");
        }
        return ExitCode::SUCCESS;
    }
    let differences = differences(&root, &files);
    for difference in &differences {
        eprintln!("{difference}");
    }
    if differences.is_empty() {
        println!("numeric corpus: {} files reproduced", files.len());
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// FR-305: the committed corpus is exactly what the independent
    /// generator produces, so it stays reproducible.
    // Trace: TC-1511, TC-1532, FR-305-AC-1, NFR-019-AC-2
    #[test]
    fn committed_numeric_corpus_is_reproduced() {
        let files = generate();
        assert_eq!(differences(&corpus_root(), &files), Vec::<String>::new());
    }

    /// The JSON writers match Python's `json.dumps` layouts they replace.
    // Trace: TC-1532, NFR-019-AC-2
    #[test]
    fn writers_match_the_reference_layouts() {
        let value = obj(vec![
            ("a", J::Arr(vec![int(0), int(-5)])),
            ("b", J::Arr(vec![])),
            ("c", J::Bool(false)),
        ]);
        assert_eq!(canonical(&value), r#"{"a":[0,-5],"b":[],"c":false}"#);
        let mut text = String::new();
        pretty(&value, 0, &mut text);
        assert_eq!(
            text,
            "{\n  \"a\": [\n    0,\n    -5\n  ],\n  \"b\": [],\n  \"c\": false\n}"
        );
    }

    /// A mutated generator output is detected as a difference.
    // Trace: TC-1511, FR-305-AC-1
    #[test]
    fn a_changed_case_is_detected() {
        let mut files = generate();
        files.insert("valid/every-node-and-operator.json".to_owned(), "{}".to_owned());
        assert_eq!(differences(&corpus_root(), &files).len(), 1);
    }
}
