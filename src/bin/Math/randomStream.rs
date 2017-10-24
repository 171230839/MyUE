

use rand;
use rand::Rng;
use super::vector::FVector;
use std::rc::Rc;
use std::cell::RefCell;
pub struct FRandomStream{
    rng: Rc<RefCell<rand::ThreadRng>>,
}

impl FRandomStream{
    pub fn new() ->Self{
        FRandomStream{
            rng: Rc::new(RefCell::new(rand::thread_rng())),
        }
    }

    pub fn getFraction(&self) -> f32{
        self.rng.borrow_mut().gen_range::<f32>(0.0, 1.0)
    }

    pub fn getUnsignedInt(&self) -> u32{
        self.rng.borrow_mut().gen::<u32>()
    }

    pub fn genUnitVector(&self) -> FVector{
        self.rng.borrow_mut().gen::<FVector>()
    }

    pub fn randHelper(&self, a : i32) -> i32{
        self.rng.borrow_mut().gen_range::<i32>(0, a)
    }
}