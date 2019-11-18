#[derive(Debug)]
pub struct Input {
    pub universal_regions: Vec<String>,
    pub placeholders: Vec<Placeholder>,
    pub known_subsets: Vec<KnownSubset>,
    pub blocks: Vec<Block>,
    pub var_uses_region: Vec<(String, String)>,
    pub var_drops_region: Vec<(String, String)>,
}

impl Input {
    pub fn new(
        universal_regions: Vec<String>,
        placeholders: Option<Vec<Placeholder>>,
        known_subsets: Option<Vec<KnownSubset>>,
        var_uses_region: Option<Vec<(String, String)>>,
        var_drops_region: Option<Vec<(String, String)>>,
        blocks: Vec<Block>,
    ) -> Input {
        Input {
            universal_regions,
            placeholders: placeholders.unwrap_or_default(),
            known_subsets: known_subsets.unwrap_or_default(),
            var_uses_region: var_uses_region.unwrap_or_default(),
            var_drops_region: var_drops_region.unwrap_or_default(),
            blocks,
        }
    }
}

#[derive(Debug)]
pub struct Block {
    pub name: String,
    pub statements: Vec<Statement>,
    pub goto: Vec<String>,
}

#[derive(Debug)]
pub struct Statement {
    /// Effects destined to be emitted at the Statement's Start point
    pub effects_start: Vec<Effect>,

    /// Effects destined to be emitted at the Statement's Mid point
    pub effects: Vec<Effect>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Effect {
    Use { origins: Vec<String> },
    Fact(Fact),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Fact {
    Outlives { a: String, b: String },
    BorrowRegionAt { origin: String, loan: String },
    Invalidates { loan: String },
    Kill { loan: String },
    RegionLiveAt { origin: String },
    DefineVariable { variable: String },
    UseVariable { variable: String },
}

#[derive(Debug, PartialEq)]
pub struct KnownSubset {
    pub a: String,
    pub b: String,
}

#[derive(Debug, PartialEq)]
pub struct Placeholder {
    pub origin: String,
    pub loan: String,
}

impl Statement {
    pub(crate) fn new(effects: Vec<Effect>) -> Self {
        // Anything live on entry to the "mid point" is also live on
        // entry to the start point.
        let effects_start = effects
            .iter()
            .filter(|effect| match effect {
                Effect::Fact(Fact::RegionLiveAt { .. }) => true,
                _ => false,
            })
            .cloned()
            .collect();

        Self {
            effects_start,
            effects,
        }
    }
}
