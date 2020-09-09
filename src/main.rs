#![allow(dead_code)]
#[allow(unused_macros)]
macro_rules! require_multiple_of_eight {
    ($e:expr) => {
        let _: $crate::MultipleOfEight<[(); $e % 8]>;
    };
}

type MultipleOfEight<T> = <<T as Array>::Marker as TotalSizeIsMultipleOfEightBits>::Check;

enum ZeroMod8 {}
enum OneMod8 {}
enum TwoMod8 {}
enum ThreeMod8 {}
enum FourMod8 {}
enum FiveMod8 {}
enum SixMod8 {}
enum SevenMod8 {}

trait Array {
    type Marker;
}

impl Array for [(); 0] {
    type Marker = ZeroMod8;
}

impl Array for [(); 1] {
    type Marker = OneMod8;
}

impl Array for [(); 2] {
    type Marker = TwoMod8;
}

impl Array for [(); 3] {
    type Marker = ThreeMod8;
}

impl Array for [(); 4] {
    type Marker = FourMod8;
}

impl Array for [(); 5] {
    type Marker = FiveMod8;
}

impl Array for [(); 6] {
    type Marker = SixMod8;
}

impl Array for [(); 7] {
    type Marker = SevenMod8;
}

trait TotalSizeIsMultipleOfEightBits {
    type Check;
}

impl TotalSizeIsMultipleOfEightBits for ZeroMod8 {
    type Check = ();
}

fn main() {
    require_multiple_of_eight!(7);
}