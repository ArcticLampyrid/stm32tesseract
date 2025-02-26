use super::GenericPackageAcquirer;

pub struct PackageIndex {
    inner: std::collections::HashMap<String, Vec<GenericPackageAcquirer>>,
}

impl PackageIndex {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    pub fn insert(&mut self, name: String, acquirer: GenericPackageAcquirer) {
        if let Some(acquirers) = self.inner.get_mut(name.as_str()) {
            acquirers.push(acquirer);
        } else {
            self.inner.insert(name, vec![acquirer]);
        }
    }

    pub fn get(&self, name: &str) -> Option<&Vec<GenericPackageAcquirer>> {
        self.inner.get(name)
    }

    pub fn merge(&mut self, other: Self) {
        for (name, acquirers) in other.inner {
            if let Some(self_acquirers) = self.inner.get_mut(&name) {
                self_acquirers.extend(acquirers);
            } else {
                self.inner.insert(name, acquirers);
            }
        }
    }
}
