// geo_semantic_tokenizer.rs - Revision Omega.1
// Complete Non-Dualistic Cymatic Embedding Engine
// Enforces Zero Square Bracket Invariant across all lines

use std::string::String;

pub struct E8Point {
    pub c0: f32, pub c1: f32, pub c2: f32, pub c3: f32,
    pub c4: f32, pub c5: f32, pub c6: f32, pub c7: f32,
}

impl E8Point {
    pub fn new(c0: f32, c1: f32, c2: f32, c3: f32, c4: f32, c5: f32, c6: f32, c7: f32) -> Self {
        Self { c0, c1, c2, c3, c4, c5, c6, c7 }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    }
    pub fn dot(&self, other: &Self) -> f32 {
        self.c0 * other.c0 + self.c1 * other.c1 + self.c2 * other.c2 + self.c3 * other.c3
        + self.c4 * other.c4 + self.c5 * other.c5 + self.c6 * other.c6 + self.c7 * other.c7
    }
    pub fn norm_sq(&self) -> f32 {
        self.dot(self)
    }
}

impl Clone for E8Point {
    fn clone(&self) -> Self { *self }
}
impl Copy for E8Point {}
impl Default for E8Point {
    fn default() -> Self { Self::zero() }
}

pub struct RawToken {
    pub text: String,
    pub span_start: usize,
    pub span_end: usize,
}

impl Clone for RawToken {
    fn clone(&self) -> Self {
        Self {
            text: self.text.clone(),
            span_start: self.span_start,
            span_end: self.span_end,
        }
    }
}

pub struct SymbolicTagSet {
    pub e8_root_index: Option<u16>,
    pub color_charge: Option<u8>,
    pub decay_state: Option<u8>,
    pub role_tag: Option<u8>,
    pub channel_id: Option<u8>,
}

impl Clone for SymbolicTagSet {
    fn clone(&self) -> Self { *self }
}
impl Copy for SymbolicTagSet {}
impl Default for SymbolicTagSet {
    fn default() -> Self {
        Self {
            e8_root_index: None,
            color_charge: None,
            decay_state: None,
            role_tag: None,
            channel_id: None,
        }
    }
}

pub struct GeometricEmbedding {
    pub coords: Vec<f32>,
    pub norm: f32,
    pub symbolic: SymbolicTagSet,
}

impl Clone for GeometricEmbedding {
    fn clone(&self) -> Self {
        Self {
            coords: self.coords.clone(),
            norm: self.norm,
            symbolic: self.symbolic,
        }
    }
}

pub struct ProjectionMatrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f32>,
}

impl ProjectionMatrix {
    pub fn apply(&self, input: &Vec<f32>) -> E8Point {
        let mut row_sums = Vec::new();
        let mut r = 0;
        while r < 8 {
            let mut acc = 0.0f32;
            let mut c = 0;
            while c < self.cols {
                let idx = r * self.cols + c;
                let m_val = self.data.get(idx).copied().unwrap_or(0.0);
                let in_val = input.get(c).copied().unwrap_or(0.0);
                acc = acc + m_val * in_val;
                c = c + 1;
            }
            row_sums.push(acc);
            r = r + 1;
        }
        E8Point::new(
            row_sums.get(0).copied().unwrap_or(0.0),
            row_sums.get(1).copied().unwrap_or(0.0),
            row_sums.get(2).copied().unwrap_or(0.0),
            row_sums.get(3).copied().unwrap_or(0.0),
            row_sums.get(4).copied().unwrap_or(0.0),
            row_sums.get(5).copied().unwrap_or(0.0),
            row_sums.get(6).copied().unwrap_or(0.0),
            row_sums.get(7).copied().unwrap_or(0.0),
        )
    }
}

pub struct LatticeRoot {
    pub root_id: u16,
    pub coords: E8Point,
    pub color_charge: u8,
    pub decay_state: u8,
    pub role_tag: u8,
}

impl Clone for LatticeRoot {
    fn clone(&self) -> Self { *self }
}
impl Copy for LatticeRoot {}
impl Default for LatticeRoot {
    fn default() -> Self {
        Self {
            root_id: 0,
            coords: E8Point::zero(),
            color_charge: 0,
            decay_state: 0,
            role_tag: 0,
        }
    }
}

pub enum SnapMode {
    GeometricDominant,
    StructuralDominant,
}

impl Clone for SnapMode {
    fn clone(&self) -> Self { *self }
}
impl Copy for SnapMode {}

pub fn score_geometric(proj: &E8Point, root: &LatticeRoot) -> f32 {
    let dot = proj.dot(&root.coords);
    let nv = proj.norm_sq();
    let nr = root.coords.norm_sq();
    if nv == 0.0 || nr == 0.0 {
        return 0.0;
    }
    dot / (nv.sqrt() * nr.sqrt())
}

pub fn score_structural(tags: &SymbolicTagSet, root: &LatticeRoot) -> f32 {
    let mut score = 0.0f32;
    if tags.color_charge == Some(root.color_charge) {
        score = score + 1.0;
    }
    if tags.decay_state == Some(root.decay_state) {
        score = score + 0.7;
    }
    if tags.role_tag == Some(root.role_tag) {
        score = score + 0.5;
    }
    score
}

pub fn snap_to_root(
    proj: &E8Point,
    tags: &SymbolicTagSet,
    roots: &Vec<LatticeRoot>,
    mode: SnapMode,
) -> LatticeRoot {
    let mut best = roots.first().copied().unwrap_or_else(|| LatticeRoot::default());
    let mut best_score = -1000000.0f32;
    let mut i = 0;
    while i < roots.len() {
        if let Some(root) = roots.get(i) {
            let geom = score_geometric(proj, root);
            let strct = score_structural(tags, root);
            let score = match mode {
                SnapMode::GeometricDominant => 0.7 * geom + 0.3 * strct,
                SnapMode::StructuralDominant => 0.8 * strct + 0.2 * geom,
            };
            if score > best_score {
                best_score = score;
                best = *root;
            }
        }
        i = i + 1;
    }
    best
}

pub struct SnappedToken {
    pub raw: RawToken,
    pub embedding: GeometricEmbedding,
    pub root: LatticeRoot,
    pub compatibility_score: f32,
}

impl Clone for SnappedToken {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            embedding: self.embedding.clone(),
            root: self.root,
            compatibility_score: self.compatibility_score,
        }
    }
}

pub struct Triplet {
    pub a: SnappedToken,
    pub b: SnappedToken,
    pub c: SnappedToken,
    pub relation_tag: SymbolicTagSet,
}

impl Clone for Triplet {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            c: self.c.clone(),
            relation_tag: self.relation_tag,
        }
    }
}

pub struct BraidLoomSlot {
    pub slot_id: u64,
    pub time_step: u64,
    pub triplet: Triplet,
    pub layer_id: u8,
    pub braid_index: u16,
}

impl Clone for BraidLoomSlot {
    fn clone(&self) -> Self {
        Self {
            slot_id: self.slot_id,
            time_step: self.time_step,
            triplet: self.triplet.clone(),
            layer_id: self.layer_id,
            braid_index: self.braid_index,
        }
    }
}

pub struct BraidLoom {
    pub slots: Vec<BraidLoomSlot>,
    pub max_slots: usize,
}

impl BraidLoom {
    pub fn new(max_slots: usize) -> Self {
        Self {
            slots: Vec::with_capacity(max_slots),
            max_slots,
        }
    }
    pub fn push_slot(&mut self, slot: BraidLoomSlot) {
        if self.slots.len() >= self.max_slots {
            self.slots.remove(0);
        }
        self.slots.push(slot);
    }
}

pub fn weave_triplets(snapped: &Vec<SnappedToken>) -> Vec<Triplet> {
    let mut triplets = Vec::new();
    let mut i = 0;
    while i + 2 < snapped.len() {
        if let (Some(a), Some(b), Some(c)) = (snapped.get(i), snapped.get(i + 1), snapped.get(i + 2)) {
            triplets.push(Triplet {
                a: a.clone(),
                b: b.clone(),
                c: c.clone(),
                relation_tag: SymbolicTagSet::default(),
            });
        }
        i = i + 3;
    }
    triplets
}

pub fn build_loom(triplets: &Vec<Triplet>) -> BraidLoom {
    let mut loom = BraidLoom::new(1024);
    let mut slot_id = 0u64;
    let mut i = 0;
    while i < triplets.len() {
        if let Some(t) = triplets.get(i) {
            loom.push_slot(BraidLoomSlot {
                slot_id,
                time_step: i as u64,
                triplet: t.clone(),
                layer_id: 0,
                braid_index: slot_id as u16,
            });
            slot_id = slot_id + 1;
        }
        i = i + 1;
    }
    loom
}

pub enum Channel {
    Physics,
    Mythic,
    System,
    Affective,
}

pub struct ChannelInvariants {
    pub causality_ok: bool,
    pub temporal_coherence: f32,
    pub archetype_consistency: f32,
    pub symbol_coherence: f32,
    pub structural_integrity: f32,
    pub protocol_consistency: f32,
    pub tone_continuity: f32,
    pub valence_stability: f32,
}

pub fn compute_physics_invariants(loom: &BraidLoom) -> ChannelInvariants {
    let mut temporal_jumps = 0u32;
    let mut steps = 0u32;
    let mut i = 0;
    while i + 1 < loom.slots.len() {
        if let (Some(s0), Some(s1)) = (loom.slots.get(i), loom.slots.get(i + 1)) {
            if s1.time_step < s0.time_step {
                temporal_jumps = temporal_jumps + 1;
            }
            steps = steps + 1;
        }
        i = i + 1;
    }
    let temporal_coherence = if steps == 0 {
        1.0
    } else {
        1.0 - (temporal_jumps as f32 / steps as f32)
    };
    ChannelInvariants {
        causality_ok: temporal_jumps == 0,
        temporal_coherence,
        archetype_consistency: 0.0,
        symbol_coherence: 0.0,
        structural_integrity: 0.0,
        protocol_consistency: 0.0,
        tone_continuity: 0.0,
        valence_stability: 0.0,
    }
}

pub struct PhraseTemplate {
    pub name: String,
    pub pattern: String,
    pub role_a: u8,
    pub role_b: u8,
    pub role_c: u8,
}

impl Clone for PhraseTemplate {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            pattern: self.pattern.clone(),
            role_a: self.role_a,
            role_b: self.role_b,
            role_c: self.role_c,
        }
    }
}

pub fn match_template(
    triplet: &Triplet,
    templates: &Vec<PhraseTemplate>,
) -> Option<(PhraseTemplate, f32)> {
    let a_role = triplet.a.root.role_tag;
    let b_role = triplet.b.root.role_tag;
    let c_role = triplet.c.root.role_tag;
    let mut best = None;
    let mut best_score = -1000000.0f32;
    let mut i = 0;
    while i < templates.len() {
        if let Some(tpl) = templates.get(i) {
            let mut score = 0.0f32;
            if tpl.role_a == a_role {
                score = score + 1.0;
            }
            if tpl.role_b == b_role {
                score = score + 1.0;
            }
            if tpl.role_c == c_role {
                score = score + 1.0;
            }
            if score > best_score {
                best_score = score;
                best = Some((tpl.clone(), score));
            }
        }
        i = i + 1;
    }
    best
}

pub fn render_phrase(triplet: &Triplet, tpl: &PhraseTemplate) -> String {
    let s0 = tpl.pattern.replace("{A}", &triplet.a.raw.text);
    let s1 = s0.replace("{B}", &triplet.b.raw.text);
    s1.replace("{C}", &triplet.c.raw.text)
}

pub struct GeoSemanticTokenizer {
    pub proj: ProjectionMatrix,
    pub roots: Vec<LatticeRoot>,
}

impl GeoSemanticTokenizer {
    pub fn tokenize(&self, text: &str) -> Vec<RawToken> {
        let mut tokens = Vec::new();
        let words = text.split_whitespace();
        let mut idx = 0;
        for w in words {
            tokens.push(RawToken {
                text: w.to_string(),
                span_start: idx,
                span_end: idx + 1,
            });
            idx = idx + 1;
        }
        tokens
    }

    pub fn embed(&self, token: &RawToken) -> GeometricEmbedding {
        let phi = 1.61803398875f32;
        let ang_91 = 1.5882496f32;
        let ang_108 = 1.8849556f32;

        let mut gem_sum = 0u32;
        for b in token.text.bytes() {
            gem_sum = gem_sum + (b as u32);
        }
        let root_idx = ((gem_sum % 240) as u16);

        let mut c = Vec::new();
        let mut col = 0;
        while col < self.proj.cols {
            c.push(0.0f32);
            col = col + 1;
        }

        let mut char_idx = 0usize;
        for b in token.text.bytes() {
            let val = b as f32;
            let theta = (char_idx as f32) * ang_91 + ((gem_sum % 7) as f32) * ang_108;
            let pol = ((char_idx as f32) * phi).sin() * (val * 0.1).cos();
            let k = char_idx % self.proj.cols;
            let k_next = (char_idx + 1) % self.proj.cols;

            let contrib_cos = pol * theta.cos() * val.sqrt();
            let contrib_sin = pol * theta.sin() * val.sqrt();

            if let Some(elem) = c.get_mut(k) {
                *elem = *elem + contrib_cos;
            }
            if let Some(elem_next) = c.get_mut(k_next) {
                *elem_next = *elem_next + contrib_sin;
            }
            char_idx = char_idx + 1;
        }

        let mut sum_sq = 0.0f32;
        let mut idx = 0;
        while idx < c.len() {
            if let Some(v) = c.get(idx) {
                sum_sq = sum_sq + (*v) * (*v);
            }
            idx = idx + 1;
        }
        let norm = sum_sq.sqrt();
        let scale = if norm > 0.00001 { 1.0 / norm } else { 1.0 };

        let mut normalized_coords = Vec::new();
        idx = 0;
        while idx < c.len() {
            if let Some(v) = c.get(idx) {
                normalized_coords.push((*v) * scale);
            }
            idx = idx + 1;
        }

        let role = ((gem_sum % 3) + 1) as u8;
        let color = ((gem_sum % 8) + 1) as u8;

        GeometricEmbedding {
            coords: normalized_coords,
            norm: 1.0,
            symbolic: SymbolicTagSet {
                e8_root_index: Some(root_idx),
                color_charge: Some(color),
                decay_state: Some(0),
                role_tag: Some(role),
                channel_id: Some(1),
            },
        }
    }

    pub fn project(&self, emb: &GeometricEmbedding) -> E8Point {
        self.proj.apply(&emb.coords)
    }

    pub fn snap(&self, proj: &E8Point, tags: &SymbolicTagSet, mode: SnapMode) -> LatticeRoot {
        snap_to_root(proj, tags, &self.roots, mode)
    }

    pub fn process(&self, text: &str, mode: SnapMode) -> Vec<SnappedToken> {
        let raw_tokens = self.tokenize(text);
        let mut out = Vec::new();
        let mut i = 0;
        while i < raw_tokens.len() {
            if let Some(raw) = raw_tokens.get(i) {
                let emb = self.embed(raw);
                let proj = self.project(&emb);
                let root = self.snap(&proj, &emb.symbolic, mode);
                let compat = score_geometric(&proj, &root);
                out.push(SnappedToken {
                    raw: raw.clone(),
                    embedding: emb,
                    root,
                    compatibility_score: compat,
                });
            }
            i = i + 1;
        }
        out
    }
}