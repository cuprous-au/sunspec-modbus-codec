use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// Rust keywords that cannot be used as a bare identifier. Treated as already-taken so a name
/// that collides with one falls back the same way it would against another generated name.
const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield", "union",
];

/// A generated identifier, in both the casing conventions codegen needs. Held behind a
/// [`NameRef`] so a [`NameTable`]'s deduplication pass can overwrite it in place once every
/// name in its scope is known, and anything holding the ref sees the final value with no extra
/// propagation step.
pub struct Name {
    pub snake_case: String,
    pub pascal_case: String,
}

impl Name {
    /// Builds a fresh, unshared name ref holding `snake_case`/`pascal_case` as given. This does
    /// not register the name with any [`NameTable`] - the caller decides whether and where it
    /// needs to be deduplicated.
    pub fn new(snake_case: String, pascal_case: String) -> NameRef {
        Rc::new(RefCell::new(Name {
            snake_case,
            pascal_case,
        }))
    }
}

/// A shared, mutable handle to a [`Name`]. Cloning a `NameRef` (cheap: it's a refcounted
/// pointer) shares the same underlying name, so mutating it through one clone - as
/// [`NameTable::deduplicate`] does - is visible through every other.
pub type NameRef = Rc<RefCell<Name>>;

struct Registration {
    name: NameRef,
    fallback: Option<(String, String)>,
}

/// Collects one broad category of generated names - points, groups, enums, or models - that
/// all live in the same generated-code scope and so must be pairwise unique (e.g. one model's
/// `Point` enum variants, or the model marker structs shared across the whole crate).
///
/// Naming happens in two steps: every name in the scope is [`NameTable::register`]ed first,
/// while resolving the SunSpec schema and before any name is known to collide with any other;
/// [`NameTable::deduplicate`] is then called once, after every entity in the scope has been
/// resolved, and resolves every collision together. This keeps the schema walk itself free of
/// order-dependent naming decisions - a point doesn't need to know about its siblings to pick
/// its own name.
#[derive(Default)]
pub struct NameTable {
    pending: HashMap<String, Vec<Registration>>,
}

impl NameTable {
    /// Enrolls `name` (already built by [`Name::new`] and held by the object it names) for
    /// deduplication, with `fallback` - typically a shorter, already-unique source identifier -
    /// substituted in if `name`'s current value turns out to collide with a Rust keyword or
    /// another name registered in this table.
    pub fn register(&mut self, name: NameRef, fallback: (String, String)) {
        self.push_registration(Registration {
            name,
            fallback: Some(fallback),
        });
    }

    /// Enrolls `name`, which is already expected to be unique in this scope and so has nothing
    /// shorter to fall back to. [`NameTable::deduplicate`] still checks it: a genuine collision
    /// fails the build loudly instead of silently producing two generated items sharing an
    /// identifier.
    pub fn register_unique(&mut self, name: NameRef) {
        self.push_registration(Registration {
            name,
            fallback: None,
        });
    }

    fn push_registration(&mut self, registration: Registration) {
        let key = registration.name.borrow().snake_case.clone();
        self.pending.entry(key).or_default().push(registration);
    }

    /// Resolves every collision among the names registered so far.
    pub fn deduplicate(&mut self) {
        for (snake, registrations) in self.pending.drain() {
            if registrations.len() > 1 || RUST_KEYWORDS.contains(&snake.as_str()) {
                for registration in registrations {
                    let mut name = registration.name.borrow_mut();
                    if let Some((fallback_snake_case, fallback_pascal_case)) = registration.fallback
                    {
                        name.snake_case = fallback_snake_case;
                        name.pascal_case = fallback_pascal_case;
                    } else {
                        panic!(
                            "name `{}` was registered as unique but collides with another name",
                            name.snake_case,
                        )
                    }
                }
            }
        }
    }
}

/// A resolved schema object with a generated identifier, accessible in both casing
/// conventions via a shared, potentially-not-yet-deduplicated [`NameRef`].
pub trait Named {
    fn name_ref(&self) -> &NameRef;

    fn name_snake_case(&self) -> String {
        self.name_ref().borrow().snake_case.clone()
    }

    fn name_pascal_case(&self) -> String {
        self.name_ref().borrow().pascal_case.clone()
    }
}
