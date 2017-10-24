
const NAME_NO_NUMBER_INTERNAL : i32 = 0;

pub struct FMinimalName{
    pub index: i32,
    pub number: i32,
}

impl FMinimalName{
    pub fn new() ->Self{
        FMinimalName{
            index: 0,
            number: NAME_NO_NUMBER_INTERNAL,
        }
    }

    pub fn new_with_index(n: i32) -> Self{
        FMinimalName{
            index: n,
            number: NAME_NO_NUMBER_INTERNAL,
        }
    }

    pub fn new_with_index_and_number(n: i32, num: i32) -> Self{
        FMinimalName{
            index: n,
            number: num,
        }
    }
}



