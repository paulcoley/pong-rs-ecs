use crate::components::{CMovement2D, CCollision2D, CText};
use crate::components::{CPaddleInfo, CTexture, CPosition2D, CButtonInfo};
use crate::components::Components;

#[derive(Default)]
pub struct ComponentManager {
    pub id_allocator: IDAllocator,

    // Component lists
    pub ccollision_2d: Components<CCollision2D>,
    pub cmovement_2d: Components<CMovement2D>,
    pub cpaddle_info: Components<CPaddleInfo>,
    pub cposition_2d: Components<CPosition2D>,
    pub ctexture: Components<CTexture>,
    pub cbutton_info: Components<CButtonInfo>,
    pub ctext: Components<CText>
}

impl ComponentManager {
    pub fn free_entity(&mut self, id: usize) {
        self.ccollision_2d.remove(&id);
        self.cmovement_2d.remove(&id);
        self.cpaddle_info.remove(&id);
        self.cposition_2d.remove(&id);
        self.ctexture.remove(&id);
        self.cbutton_info.remove(&id);
        self.ctext.remove(&id);

        self.id_allocator.free_number(id);
    }
}

#[derive(Default)]
pub struct IDAllocator {
    next_id: usize,
    free_ids: Vec<usize>,
    allocated_ids: Vec<usize>
}

impl IDAllocator {
    pub fn get_number(&mut self) -> usize {
        if let Some(id) = self.free_ids.pop() {
            self.allocated_ids.push(id);
            id
        }
        else {
            let id = self.next_id;
            self.next_id += 1;
            self.allocated_ids.push(id);
            id
        }
    }

    /*
        Prefer calling wrapper function of the same name in utility.rs
    */
    pub fn get_allocated_ids(&self) -> Vec<usize> {
        self.allocated_ids[..].to_vec()
    }

    pub fn free_number(&mut self, id: usize) {
        if !self.free_ids.contains(&id) {
            self.free_ids.push(id);
            self.allocated_ids.retain(|&x| x != id )
        }
    }
}