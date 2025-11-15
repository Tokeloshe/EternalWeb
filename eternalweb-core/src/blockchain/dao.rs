//! DAO implementation

use super::*;

/// DAO structure
pub struct DAO {
    pub id: String,
    pub proposals: Vec<Proposal>,
}

impl DAO {
    pub fn new(id: String) -> Self {
        Self {
            id,
            proposals: vec![],
        }
    }

    pub fn add_proposal(&mut self, proposal: Proposal) {
        self.proposals.push(proposal);
    }
}
