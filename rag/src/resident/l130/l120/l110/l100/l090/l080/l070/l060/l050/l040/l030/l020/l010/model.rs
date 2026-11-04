use std::{collections::BTreeMap, sync::Arc};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name(String);

impl Name {
    pub fn new(s: impl Into<String>) -> Result<Name, ModelError> {
        let s = s.into();
        if s.is_empty() {
            return Err(ModelError::EmptyName);
        }
        Ok(Name(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Param {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct StageSpec {
    pub name: Option<Name>,
    pub params: BTreeMap<String, Param>,
}

impl StageSpec {
    pub fn new(name: Name) -> Self {
        StageSpec {
            name: Some(name),
            params: BTreeMap::new(),
        }
    }

    pub fn with_param(mut self, key: impl Into<String>, value: Param) -> Self {
        self.params.insert(key.into(), value);
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModelSpec {
    id: Name,
    encoder: StageSpec,
    scorer: StageSpec,
}

impl ModelSpec {
    pub fn new(id: Name, encoder: StageSpec, scorer: StageSpec) -> Result<ModelSpec, ModelError> {
        if encoder.name.is_none() || scorer.name.is_none() {
            return Err(ModelError::EmptyName);
        }
        Ok(ModelSpec {
            id,
            encoder,
            scorer,
        })
    }

    pub fn id(&self) -> &Name {
        &self.id
    }

    pub fn encoder(&self) -> &StageSpec {
        &self.encoder
    }

    pub fn scorer(&self) -> &StageSpec {
        &self.scorer
    }

    pub fn resolve<E, S>(
        &self,
        encoders: &Registry<E>,
        scorers: &Registry<S>,
    ) -> Result<ResolvedModel<E, S>, ModelError> {
        let enc_name = self.encoder.name.as_ref().ok_or(ModelError::EmptyName)?;
        let sc_name = self.scorer.name.as_ref().ok_or(ModelError::EmptyName)?;
        let encoder = encoders
            .get(enc_name)
            .ok_or_else(|| ModelError::UnknownEncoder(enc_name.as_str().to_owned()))?;
        let scorer = scorers
            .get(sc_name)
            .ok_or_else(|| ModelError::UnknownScorer(sc_name.as_str().to_owned()))?;
        Ok(ResolvedModel {
            spec: self.clone(),
            encoder: Arc::clone(encoder),
            scorer: Arc::clone(scorer),
        })
    }
}

pub struct Registry<T> {
    entries: BTreeMap<Name, Arc<T>>,
}

impl<T> Registry<T> {
    pub fn new() -> Self {
        Registry {
            entries: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, name: Name, entry: Arc<T>) -> Result<(), ModelError> {
        if self.entries.contains_key(&name) {
            return Err(ModelError::DuplicateName(name.as_str().to_owned()));
        }
        self.entries.insert(name, entry);
        Ok(())
    }

    pub fn get(&self, name: &Name) -> Option<&Arc<T>> {
        self.entries.get(name)
    }

    pub fn contains(&self, name: &Name) -> bool {
        self.entries.contains_key(name)
    }

    pub fn names(&self) -> impl Iterator<Item = &Name> {
        self.entries.keys()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl<T> Default for Registry<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub type EncoderRegistry<E> = Registry<E>;
pub type ScorerRegistry<S> = Registry<S>;

pub struct ResolvedModel<E, S> {
    pub spec: ModelSpec,
    pub encoder: Arc<E>,
    pub scorer: Arc<S>,
}

impl<E, S> Clone for ResolvedModel<E, S> {
    fn clone(&self) -> Self {
        ResolvedModel {
            spec: self.spec.clone(),
            encoder: Arc::clone(&self.encoder),
            scorer: Arc::clone(&self.scorer),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ModelError {
    #[error("empty name")]
    EmptyName,
    #[error("duplicate name: {0}")]
    DuplicateName(String),
    #[error("unknown encoder: {0}")]
    UnknownEncoder(String),
    #[error("unknown scorer: {0}")]
    UnknownScorer(String),
}

#[cfg(test)]
#[path = "test/model.rs"]
mod tests;
