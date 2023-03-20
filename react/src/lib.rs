/// `InputCellId` is a unique identifier for an input cell.

/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```

use std::collections::{HashMap, HashSet};
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

type UpdateFunc<'a, T> = dyn 'a + Fn(&[T]) -> T;
type Callback<'a, T> = dyn 'a + Fn(T);

struct ComputeCell<'a, T: Copy + PartialEq> {
    value: T,
    update_fn: Box<UpdateFunc<'a, T>>,
    deps: Vec<CellId>,
    callbacks: HashMap<CallbackId, Box<Callback<'a, T>>>
}

impl<'a, T: Copy + PartialEq> ComputeCell<'a, T> {
    pub fn new(value: T, deps: &[CellId], update_fn: Box<UpdateFunc<'a, T>>) -> ComputeCell<'a, T> {
        ComputeCell {
            value,
            deps: deps.to_vec(),
            update_fn,
            callbacks: HashMap::new(),
        }
    }
    pub fn run_callbacks(&self) { self.callbacks.values().for_each(|cb| (cb)(self.value)) }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

#[derive(Default)]
pub struct Reactor<'a, T: 'a + Copy + PartialEq> {
    input_cells: HashMap<InputCellId, T>,
    compute_cells: HashMap<ComputeCellId, ComputeCell<'a, T>>,
    dependency_map: HashMap<CellId, Vec<ComputeCellId>>,
}

impl<'a, T: 'a + Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            input_cells: HashMap::new(),
            compute_cells: HashMap::new(),
            dependency_map: HashMap::new(),
        }
    }

    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let id = InputCellId(generate_id());
        self.input_cells.insert(id, initial);
        id
    }

    pub fn create_compute<F: 'a + Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        // Make sure all deps exist, & return error if not
        for dep_id in dependencies {
            if !match dep_id {
                CellId::Input(id) => self.input_cells.contains_key(id),
                CellId::Compute(id) => self.compute_cells.contains_key(id),
            } { return Err(*dep_id) }
        }
        // create compute cell id, add it to the dependency_map, with dependency vec as the value
        let id = ComputeCellId(generate_id());
        for source in dependencies {
            let entry = self.dependency_map.entry(*source).or_insert(Vec::new());
            entry.push(id)
        }
        // add the updater fn to the store vector
        let arg_vals = self.get_values(dependencies);
        let initial_val = compute_func(&arg_vals[..]);
        self.compute_cells.insert(
            id,
            ComputeCell::new(initial_val, dependencies, Box::new(compute_func)),
        );
        Ok(id)
    }


    pub fn value(&self, id: CellId) -> Option<T> {
        Some(*match id {
            CellId::Input(id) => self.input_cells.get(&id)?,
            CellId::Compute(id) => &self.compute_cells.get(&id)?.value,
        })
    }

    fn get_values(&self, ids: &[CellId]) -> Vec<T> {
        ids.iter().map(|&id| self.value(id).unwrap()).collect()
    }

    pub fn set_value(&mut self, id: InputCellId, new_val: T) -> bool {
        if let Some(var) = self.input_cells.get_mut(&id) {
            *var = new_val;
            // update dependencies then fire off callbacks for cells that changed
            let have_changed = self.update_deps(CellId::Input(id));
            have_changed.iter()
                .for_each(|id| self.compute_cells.get_mut(id).unwrap().run_callbacks());
            true
        } else { false }
    }

    pub fn update_deps(&mut self, id: CellId) -> HashSet<ComputeCellId> {
        let mut have_changed = HashSet::new();
        let mut ids = vec![id];
        while !ids.is_empty() {
            let _id = ids.remove(0);
            if let Some(dependents) = self.dependency_map.get(&_id) {
                for comp_cell_id in dependents {
                    let comp_cell = &self.compute_cells.get(comp_cell_id).unwrap();
                    let new_val = (*comp_cell.update_fn)(&self.get_values(&comp_cell.deps)[..]);
                    if new_val != comp_cell.value {
                        have_changed.insert(*comp_cell_id);
                        self.compute_cells.get_mut(comp_cell_id).unwrap().value = new_val;
                    }
                }
                ids.extend(dependents.iter().cloned().map(CellId::Compute))
            }
        }
        have_changed
    }
    pub fn add_callback<Fm: 'a + Fn(T)>(
        &mut self,
        id: ComputeCellId,
        callback: Fm,
    ) -> Option<CallbackId> {
        if let Some(comp_cell) = self.compute_cells.get_mut(&id) {
            let _cb_id = CallbackId(generate_id());
            comp_cell.callbacks.insert(_cb_id, Box::new(callback));
            Some(_cb_id)
        } else { None }
    }

    pub fn remove_callback(
        &mut self,
        cell_id: ComputeCellId,
        callback_id: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if let Some(cell) = self.compute_cells.get_mut(&cell_id) {
            match cell.callbacks.remove(&callback_id) {
                Some(_) => Ok(()),
                None => Err(RemoveCallbackError::NonexistentCallback) }
        } else { Err(RemoveCallbackError::NonexistentCell) }
    }
}

pub fn generate_id() -> u64 {
    rand::thread_rng().gen_range(0..u64::MAX)
}






