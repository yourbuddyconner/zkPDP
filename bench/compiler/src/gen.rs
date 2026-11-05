//! Term IR to Noir. One function per request environment.

use cedar_policy_symcc::op::Op;
use cedar_policy_symcc::term::{Term, TermPrim};
use cedar_policy_symcc::term_type::TermType;
use envelope::{sha256, Kind, Layouts, MAX_ANC, MAX_ATTRS, MAX_SET};
use smol_str::SmolStr;
use std::collections::BTreeMap;
use std::sync::Arc;

type Rty = Arc<BTreeMap<SmolStr, TermType>>;

/// A translated term: Noir expressions and what they mean.
#[derive(Clone)]
enum Val {
    Bool(String),
    /// An entity UID hash or a string hash.
    B32(String),
    /// A Cedar Long as u64, two's complement.
    Bv(String),
    Opt(String, Box<Val>),
    Rec { present: String, slots: String, rty: Rty },
    /// An expression of type `[[u8; 32]; n]`. Zero slots are padding.
    Set(String, usize),
    Inter(Box<(String, usize)>, Box<(String, usize)>),
    /// Something the translation cannot read. Using it is an error.
    Opaque(String),
}

pub fn kind_of(ty: &TermType) -> Kind {
    match ty {
        TermType::Option { ty } => kind_of(ty),
        TermType::Set { ty } => Kind::Set(Box::new(kind_of(ty))),
        TermType::Record { rty } => Kind::Record(rty.iter().map(|(k, v)| (k.to_string(), kind_of(v))).collect()),
        _ => Kind::Leaf,
    }
}

fn bytes(h: &[u8; 32]) -> String {
    format!("[{}]", h.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(", "))
}

/// What one environment's function needs opened, in slot order.
#[derive(Clone, Default)]
pub struct Needs {
    pub ents: Vec<Term>,
    pub sets: Vec<(Term, Kind)>,
    pub recs: Vec<(Term, Vec<(String, Kind)>)>,
}

pub struct EnvGen<'a> {
    lines: Vec<String>,
    memo: BTreeMap<Term, Val>,
    pub needs: Needs,
    n: usize,
    layouts: &'a mut Layouts,
}

impl<'a> EnvGen<'a> {
    pub fn new(layouts: &'a mut Layouts) -> Self {
        EnvGen { lines: vec![], memo: BTreeMap::new(), needs: Needs::default(), n: 0, layouts }
    }

    fn bind(&mut self, expr: String) -> String {
        self.n += 1;
        let name = format!("t{}", self.n);
        self.lines.push(format!("    let {name} = {expr};"));
        name
    }

    fn boolean(&mut self, t: &Term) -> Result<String, String> {
        match self.val(t)? {
            Val::Bool(e) => Ok(e),
            Val::Opaque(why) => Err(why),
            _ => Err("a non-boolean where a boolean is needed".into()),
        }
    }

    fn set(&mut self, t: &Term) -> Result<(String, usize), String> {
        match self.val(t)? {
            Val::Set(e, n) => Ok((e, n)),
            Val::Opaque(why) => Err(why),
            Val::Inter(..) => Err("a set intersection used as a set".into()),
            _ => Err("a non-set where a set is needed".into()),
        }
    }

    fn slot(&mut self, t: &Term) -> Result<String, String> {
        match self.val(t)? {
            Val::B32(e) => Ok(e),
            Val::Opaque(why) => Err(why),
            _ => Err("set element that is not an entity or a string".into()),
        }
    }

    fn eq(&mut self, a: &Val, b: &Val) -> Result<String, String> {
        Ok(match (a, b) {
            (Val::Bool(x), Val::Bool(y)) | (Val::B32(x), Val::B32(y)) | (Val::Bv(x), Val::Bv(y)) => format!("({x} == {y})"),
            (Val::Opt(pa, va), Val::Opt(pb, vb)) => {
                let inner = self.eq(va, vb)?;
                format!("(({pa} == {pb}) & (!{pa} | {inner}))")
            }
            (Val::Inter(x, y), Val::Set(_, 0)) | (Val::Set(_, 0), Val::Inter(x, y)) => {
                if x.1 == 0 || y.1 == 0 { "true".into() } else { format!("(!intersects({}, {}))", x.0, y.0) }
            }
            (Val::Set(_, 0), Val::Set(_, 0)) => "true".into(),
            (Val::Opaque(why), _) | (_, Val::Opaque(why)) => return Err(why.clone()),
            (Val::Set(..), _) | (_, Val::Set(..)) | (Val::Inter(..), _) | (_, Val::Inter(..)) => return Err("equality of sets".into()),
            (Val::Rec { .. }, _) | (_, Val::Rec { .. }) => return Err("equality of records".into()),
            _ => return Err("equality of unlike values".into()),
        })
    }

    fn ite(&mut self, c: &str, a: &Val, b: &Val) -> Result<Val, String> {
        let pick = |x: &str, y: &str| format!("(if {c} {{ {x} }} else {{ {y} }})");
        Ok(match (a, b) {
            (Val::Bool(x), Val::Bool(y)) => Val::Bool(pick(x, y)),
            (Val::B32(x), Val::B32(y)) => Val::B32(pick(x, y)),
            (Val::Bv(x), Val::Bv(y)) => Val::Bv(pick(x, y)),
            (Val::Opt(pa, va), Val::Opt(pb, vb)) => Val::Opt(pick(pa, pb), Box::new(self.ite(c, va, vb)?)),
            (Val::Opaque(why), _) | (_, Val::Opaque(why)) => return Err(why.clone()),
            _ => return Err("if-then-else over sets or records".into()),
        })
    }

    fn zero_of(ty: &TermType) -> Val {
        match ty {
            TermType::Bool => Val::Bool("false".into()),
            TermType::Entity { .. } | TermType::String => Val::B32("ZERO".into()),
            TermType::Bitvec { .. } => Val::Bv("0".into()),
            TermType::Option { ty } => Val::Opt("false".into(), Box::new(Self::zero_of(ty))),
            _ => Val::Opaque("an absent set, record or extension value".into()),
        }
    }

    /// Read a value of type `ty` out of a slot.
    fn decode(&mut self, slot: &str, present: &str, ty: &TermType, src: &Term) -> Result<Val, String> {
        Ok(match ty {
            TermType::Bool => Val::Bool(format!("({slot}[31] == 1)")),
            TermType::Entity { .. } | TermType::String => Val::B32(slot.to_string()),
            TermType::Bitvec { n } if n.get() == 64 => Val::Bv(self.bind(format!("be_u64({slot})"))),
            TermType::Option { ty } => Val::Opt(present.to_string(), Box::new(self.decode(slot, present, ty, src)?)),
            TermType::Set { ty } => {
                let j = self.needs.sets.len();
                self.needs.sets.push((src.clone(), kind_of(ty)));
                self.lines.push(format!("    let sz{j} = {slot} == ZERO;"));
                self.lines.push(format!("    assert(!sel | sz{j} | (set_h[{j}] == {slot}));"));
                self.lines.push(format!("    let s{j} = if sz{j} {{ [ZERO; MAX_SET] }} else {{ sets[{j}] }};"));
                Val::Set(format!("s{j}"), MAX_SET)
            }
            TermType::Record { rty } => {
                if rty.len() > MAX_ATTRS {
                    return Err(format!("a record with more than {MAX_ATTRS} attributes"));
                }
                let j = self.needs.recs.len();
                let Kind::Record(layout) = kind_of(ty) else { unreachable!() };
                self.needs.recs.push((src.clone(), layout));
                self.lines.push(format!("    let rz{j} = {slot} == ZERO;"));
                self.lines.push(format!("    assert(!sel | rz{j} | (rec_h[{j}] == {slot}));"));
                self.lines.push(format!("    let r{j} = if rz{j} {{ zero_rec() }} else {{ recs[{j}] }};"));
                Val::Rec { present: format!("r{j}.present"), slots: format!("r{j}.slots"), rty: rty.clone() }
            }
            TermType::Ext { .. } => Val::Opaque("an extension value (datetime, duration, decimal, ipaddr)".into()),
            TermType::Bitvec { .. } => Val::Opaque("a bit-vector that is not 64 bits".into()),
        })
    }

    /// The opened entity whose UID is the value of `arg`.
    fn entity(&mut self, arg: &Term) -> Result<usize, String> {
        if let Some(k) = self.needs.ents.iter().position(|t| t == arg) {
            return Ok(k);
        }
        let u = match self.val(arg)? {
            Val::B32(u) => u,
            Val::Opaque(why) => return Err(why),
            _ => return Err("attributes of a non-entity".into()),
        };
        let k = self.needs.ents.len();
        self.needs.ents.push(arg.clone());
        self.lines.push(format!("    let u{k} = {u};"));
        self.lines.push(format!("    let z{k} = u{k} == ZERO;"));
        self.lines.push(format!("    assert(!sel | z{k} | ((ents[{k}].uid_h == u{k}) & ent_ok[{k}]));"));
        self.lines.push(format!("    let e{k} = if z{k} {{ zero_entity() }} else {{ ents[{k}] }};"));
        Ok(k)
    }

    fn val(&mut self, t: &Term) -> Result<Val, String> {
        if let Some(v) = self.memo.get(t) {
            return Ok(v.clone());
        }
        let v = self.val_uncached(t)?;
        // Name scalar results so expressions stay small.
        let v = match v {
            Val::Bool(e) if e.len() > 24 => Val::Bool(self.bind(e)),
            Val::B32(e) if e.len() > 24 && !e.starts_with("C_") => Val::B32(self.bind(e)),
            other => other,
        };
        self.memo.insert(t.clone(), v.clone());
        Ok(v)
    }

    fn val_uncached(&mut self, t: &Term) -> Result<Val, String> {
        Ok(match t {
            Term::Prim(TermPrim::Bool(b)) => Val::Bool(b.to_string()),
            Term::Prim(TermPrim::Entity(uid)) => {
                let j = serde_json::json!({ "type": uid.type_name().to_string(), "id": AsRef::<str>::as_ref(uid.id()) });
                Val::B32(bytes(&envelope::uid_hash(&envelope::uid_text(&j).unwrap())))
            }
            Term::Prim(TermPrim::String(s)) => Val::B32(bytes(&sha256(s.as_bytes()))),
            Term::Prim(TermPrim::Bitvec(b)) if b.width().get() == 64 => {
                let i: i64 = b.to_int().to_string().parse().map_err(|_| "an integer literal out of range")?;
                Val::Bv(format!("({} as u64)", i as u64))
            }
            Term::Prim(TermPrim::Bitvec(_)) => return Err("a bit-vector that is not 64 bits".into()),
            Term::Prim(TermPrim::Ext(_)) => return Err("an extension value (datetime, duration, decimal, ipaddr)".into()),
            Term::Var(v) => match v.id.as_str() {
                "principal" => Val::B32("p_uid".into()),
                "resource" => Val::B32("r_uid".into()),
                // The context's attributes are a record bound to a public hash (decision 0024).
                "context" => match &v.ty {
                    TermType::Record { rty } if rty.len() <= MAX_ATTRS => {
                        Val::Rec { present: "ctx.present".into(), slots: "ctx.slots".into(), rty: rty.clone() }
                    }
                    _ => return Err(format!("a request context with more than {MAX_ATTRS} attributes")),
                },
                other => Val::Opaque(format!("the variable `{other}`")),
            },
            Term::None(ty) => Val::Opt("false".into(), Box::new(Self::zero_of(ty))),
            Term::Some(x) => Val::Opt("true".into(), Box::new(self.val(x)?)),
            Term::Record(_) => return Err("a record literal".into()),
            Term::Set { elts, .. } => {
                let mut es = Vec::new();
                for e in elts.iter() {
                    es.push(self.slot(e)?);
                }
                if es.is_empty() { Val::Set(String::new(), 0) } else { Val::Set(format!("[{}]", es.join(", ")), es.len()) }
            }
            Term::App { op, args, ret_ty } => match op {
                Op::And | Op::Or => {
                    let (a, b) = (self.boolean(&args[0])?, self.boolean(&args[1])?);
                    Val::Bool(format!("({a} {} {b})", if matches!(op, Op::And) { "&" } else { "|" }))
                }
                Op::Not => Val::Bool(format!("(!{})", self.boolean(&args[0])?)),
                Op::Eq => {
                    let (a, b) = (self.val(&args[0])?, self.val(&args[1])?);
                    Val::Bool(self.eq(&a, &b)?)
                }
                Op::Ite => {
                    let c = self.boolean(&args[0])?;
                    let (a, b) = (self.val(&args[1])?, self.val(&args[2])?);
                    self.ite(&c, &a, &b)?
                }
                Op::OptionGet => match self.val(&args[0])? {
                    Val::Opt(_, inner) => *inner,
                    Val::Opaque(why) => return Err(why),
                    _ => return Err("option.get of a non-option".into()),
                },
                Op::RecordGet(attr) => match self.val(&args[0])? {
                    Val::Rec { present, slots, rty } => {
                        let i = rty.keys().position(|k| k == attr).ok_or("an attribute missing from its record type")?;
                        let slot = format!("{slots}[{i}]");
                        self.decode(&slot, &format!("bit({present}, {i})"), &rty[attr], t)?
                    }
                    Val::Opaque(why) => return Err(why),
                    _ => return Err("attribute access on a non-record".into()),
                },
                Op::Uuf(f) if f.id.starts_with("attrs[") => {
                    let TermType::Record { rty } = &f.out else { return Err("attributes that are not a record".into()) };
                    if rty.len() > MAX_ATTRS {
                        return Err(format!("an entity type with more than {MAX_ATTRS} attributes"));
                    }
                    if let TermType::Entity { ety } = &f.arg {
                        let Kind::Record(layout) = kind_of(&f.out) else { unreachable!() };
                        self.layouts.insert(ety.to_string(), layout);
                    }
                    let k = self.entity(&args[0])?;
                    Val::Rec { present: format!("e{k}.present"), slots: format!("e{k}.attrs"), rty: rty.clone() }
                }
                Op::Uuf(f) if f.id.starts_with("ancs[") => {
                    let k = self.entity(&args[0])?;
                    Val::Set(format!("e{k}.ancestors"), MAX_ANC)
                }
                Op::Uuf(f) => return Err(format!("entity tags or another uninterpreted function (`{}`)", f.id.split('[').next().unwrap_or(""))),
                Op::SetMember => {
                    let x = self.slot(&args[0])?;
                    let (s, n) = self.set(&args[1])?;
                    Val::Bool(if n == 0 { "false".into() } else { format!("in_set({s}, {x})") })
                }
                Op::SetSubset => {
                    let (a, b) = (self.set(&args[0])?, self.set(&args[1])?);
                    match (a.1, b.1) {
                        (0, _) => Val::Bool("true".into()),
                        (_, 0) => Val::Bool(format!("subset({}, [ZERO])", a.0)),
                        _ => Val::Bool(format!("subset({}, {})", a.0, b.0)),
                    }
                }
                Op::SetInter => Val::Inter(Box::new(self.set(&args[0])?), Box::new(self.set(&args[1])?)),
                Op::Bvslt | Op::Bvsle => {
                    let (Val::Bv(a), Val::Bv(b)) = (self.val(&args[0])?, self.val(&args[1])?) else {
                        return Err("comparison of non-integers".into());
                    };
                    Val::Bool(format!("{}({a}, {b})", if matches!(op, Op::Bvslt) { "slt" } else { "sle" }))
                }
                Op::StringLike(_) => return Err("the `like` operator".into()),
                Op::Ext(_) => return Err("an extension value (datetime, duration, decimal, ipaddr)".into()),
                Op::Bvadd | Op::Bvsub | Op::Bvmul | Op::Bvneg | Op::Bvsaddo | Op::Bvssubo | Op::Bvsmulo | Op::Bvnego => {
                    let _ = ret_ty;
                    return Err("integer arithmetic".into());
                }
                other => return Err(format!("the operator {other:?}")),
            },
        })
    }

    /// The body of an environment function from its policies' match terms.
    pub fn finish(mut self, permits: &[Term], forbids: &[Term]) -> Result<(String, Needs), String> {
        let mut any = |me: &mut Self, ts: &[Term]| -> Result<String, String> {
            let mut parts = Vec::new();
            for t in ts {
                parts.push(me.boolean(t)?);
            }
            Ok(if parts.is_empty() { "false".into() } else { parts.join(" | ") })
        };
        let p = any(&mut self, permits)?;
        let f = any(&mut self, forbids)?;
        self.lines.push(format!("    let permit = {p};"));
        self.lines.push(format!("    let forbid = {f};"));
        self.lines.push("    permit & !forbid".into());
        Ok((self.lines.join("\n"), self.needs))
    }
}
