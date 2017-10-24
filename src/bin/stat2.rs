use FThreadSafeCounter;
use UObject::FMinimalName;

const NAME_SIZE = 1024;

enum Name{
    AnsiName()
}
struct FNameEntry{
    index: i32,
    hashNext: Box<FNameEntry>,
    
}


macro_rules! Array{
    ($T: ident, $elementType: ty, $maxTotalElements: expr, $elementsPerChunk: expr) => (
        const chunkTableSize : usize = ($maxTotalElements + $elementsPerChunk - 1) /  $elementsPerChunk;
        #[derive(Debug, Default)]
        struct $T{
            num : [[$elementType ; $elementsPerChunk]; chunkTableSize],
        }

        impl $T{
            pub fn new() -> Self{
                $T{
                    num: Default::default(),
                }
            }
        }
    )
}
Array!(TNameEntryArray, i32, 2 * 1024 * 1024, 16384);



struct TStatId{
    name: FMinimalName,
    ansiString: usize,
    wideString: usize,
}

impl TStatId{
    pub fn new() -> Self{
        TStatId{
            name: FMinimalName::new(),
            ansiString: 0,
            wideString: 0,
        }
    }

    pub fn isValidStat(&self) -> bool{
        !self.isNone()
    }

    pub fn isNone(&self) -> bool{
        self.name.index == 0 && self.name.number == 0
    }

    pub fn getName(&self) -> String{

    }
}

struct FThreadStatsStaticMembers{
    masterEnableCounter: FThreadSafeCounter,
    masterEnableUpdateNumber: FThreadSafeCounter,
    masterDisableChangeTagLock:  FThreadSafeCounter,
}

impl FThreadStatsStaticMembers{
    pub fn new()-> Self{
        FThreadStatsStaticMembers{
            masterEnableCounter: FThreadSafeCounter::new(),
            masterEnableUpdateNumber: FThreadSafeCounter::new(),
            masterDisableChangeTagLock:  FThreadSafeCounter::new(),
        }
    }
}

lazy_static!{
    pub static ref GThreadStatsStaticMembers : FThreadStatsStaticMembers = FThreadStatsStaticMembers::new();
}


struct FThreadStats{
    gThreadStatsStaticMembers: GThreadStatsStaticMembers,
}


impl FThreadStats{

    pub fn new() -> Self{
        FThreadStats{
            
        }
    }

    pub fn startThread(){

    }
}