
#[derive(Rand, Debug)]
pub struct FVector{
    x: f32,
    y: f32,
    z: f32,
}

impl FVector{
    pub fn new() -> Self{
        FVector{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}