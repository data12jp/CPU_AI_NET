use std::collections::BTreeMap;

pub type PacketId = String;
pub type NodeId = String;
pub type Confidence = f32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketType {
    Claim,
    Evidence,
    Refutation,
    Hypothesis,
    Query,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Persona,
    Orchestrator,
    Gatherer,
    Hypothesizer,
    Refuter,
    Aggregator,
    Evaluator,
}

#[derive(Debug, Clone)]
pub struct CognitivePacket {
    pub id: PacketId,
    pub packet_type: PacketType,
    pub role: Role,
    pub author: NodeId,

    pub content: String,
    pub confidence: Confidence,

    pub references: Vec<String>,
    pub depends_on: Vec<PacketId>,

    pub meta: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ClaimNode {
    pub claim: CognitivePacket,
    pub supports: Vec<PacketId>,
    pub refutes: Vec<PacketId>,
    pub notes: Vec<PacketId>,
}

impl ClaimNode {
    pub fn new(claim: CognitivePacket) -> Self {
        Self { claim, supports: vec![], refutes: vec![], notes: vec![] }
    }
    pub fn add_support(&mut self, id: PacketId) { self.supports.push(id); }
    pub fn add_refute(&mut self, id: PacketId) { self.refutes.push(id); }
    pub fn add_note(&mut self, id: PacketId) { self.notes.push(id); }
}

#[derive(Debug, Clone)]
pub struct FrozenResult {
    pub root_claim_id: PacketId,
    pub final_answer: String,
    pub confidence: Confidence,
    pub used_packets: Vec<PacketId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Gather,
    Generate,
    Refute,
    Aggregate,
    Freeze,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_claim_node() {
        let mut meta = BTreeMap::new();
        meta.insert("model".to_string(), "dummy".to_string());

        let claim = CognitivePacket {
            id: "c1".to_string(),
            packet_type: PacketType::Claim,
            role: Role::Hypothesizer,
            author: "node_h1".to_string(),
            content: "X is Y".to_string(),
            confidence: 0.6,
            references: vec!["wiki:example".to_string()],
            depends_on: vec![],
            meta,
        };

        let mut node = ClaimNode::new(claim);
        node.add_refute("r1".to_string());
        assert_eq!(node.refutes.len(), 1);
    }
}
