//! Collective management

use super::*;
use dashmap::DashMap;

/// Collective manager
pub struct CollectiveManager {
    collectives: DashMap<CollectiveId, Collective>,
}

impl CollectiveManager {
    pub fn new() -> Self {
        Self {
            collectives: DashMap::new(),
        }
    }

    pub fn add_collective(&self, collective: Collective) {
        self.collectives.insert(collective.id.clone(), collective);
    }

    pub fn get_collective(&self, id: &CollectiveId) -> Option<Collective> {
        self.collectives.get(id).map(|c| c.clone())
    }

    pub fn add_member(&self, id: &CollectiveId, member: String) -> Result<()> {
        let mut collective = self.collectives.get_mut(id)
            .ok_or_else(|| Error::Other("Collective not found".to_string()))?;

        if !collective.members.contains(&member) {
            collective.members.push(member);
        }

        Ok(())
    }

    pub fn add_content(&self, id: &CollectiveId, cid: Cid) -> Result<()> {
        let mut collective = self.collectives.get_mut(id)
            .ok_or_else(|| Error::Other("Collective not found".to_string()))?;

        if !collective.content.contains(&cid) {
            collective.content.push(cid);
        }

        Ok(())
    }
}

impl Default for CollectiveManager {
    fn default() -> Self {
        Self::new()
    }
}
