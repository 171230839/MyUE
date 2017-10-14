

// bitflags! {
//     pub struct FPlatformAffinity : u64
//     {
//         const Mask = 0xFFFFFFFFFFFFFFFF;
//     }
  
// }

#[derive(Debug)]
pub enum FPlatformAffinity{
    Mask = 0xFFFFFFFFFFFFFFFF,
}

#[derive(Debug, PartialEq,Clone,  Eq)]
pub enum EThreadPriority
{
	TPri_Normal,
	TPri_AboveNormal,
	TPri_BelowNormal,
	TPri_Highest,
	TPri_Lowest,
	TPri_SlightlyBelowNormal,
}