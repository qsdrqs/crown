use std::ops::Range;

use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;

use crate::data_structure::vec_vec::VecVec;

pub struct Discretization<Idx> {
    pub did_idx: FxHashMap<DefId, usize>,
    /// [`DefId`] -> entity -> [`std::ops::Range<Idx>`]
    pub contents: VecVec<Idx>,
}

impl<Idx: Copy> Discretization<Idx> {
    #[inline]
    pub fn contents_iter(&self, did: &DefId) -> Result<impl Iterator<Item = Range<Idx>> + '_, String> {
        if self.did_idx.get(did).is_none() {
            return Err(format!("Discretization: DefId {did:?} not found"));
        }
        let idx = self.did_idx[did];
        if idx >= self.contents.len() {
            return Err(format!("Discretization: DefId {did:?} not found"));
        }
        Ok(self.contents[idx]
            .array_windows()
            .map(|&[start, end]| start..end))
    }

    #[inline]
    pub fn content(&self, did: &DefId, idx: usize) -> Result<Range<Idx>, String> {
        if self.did_idx.get(did).is_none() {
            return Err(format!("Discretization: DefId {did:?} not found"));
        }
        if idx >= self.contents[self.did_idx[did]].len() {
            return Err(format!("Discretization: DefId {did:?} does not have {idx}th entity"));
        }
        let outer_idx = self.did_idx[did];
        Ok(self.contents[outer_idx][idx]..self.contents[outer_idx][idx + 1])
    }
}

pub struct StructFields<Idx>(pub Discretization<Idx>);
pub struct FnLocals<Idx>(pub Discretization<Idx>);

impl<Idx: Copy> StructFields<Idx> {
    /// [`fields()`] returns a slice of [`Range<T>`] that is in lock-step with [`all_fields()`]
    #[inline]
    pub fn fields(&self, did: &DefId) -> Result<impl Iterator<Item = Range<Idx>> + '_, String> {
        self.0.contents_iter(did)
    }

    #[inline]
    pub fn field(&self, did: &DefId, f: usize) -> Result<Range<Idx>, String> {
        self.0.content(did, f)
    }
}

impl<Idx: Copy> FnLocals<Idx> {
    /// [`locals()`] returns a slice of [`Range<Var>`] that is in lock-step with [`local_decls`]
    /// #[inline]
    pub fn locals(&self, did: &DefId) -> Result<impl Iterator<Item = Range<Idx>> + '_, String> {
        self.0.contents_iter(did)
    }

    #[inline]
    pub fn local(&self, did: &DefId, local: usize) -> Result<Range<Idx>, String> {
        self.0.content(did, local)
    }
}
