#![cfg_attr(not(test), no_std)]

// TODO: All kinds of things are wrong here. Need a way to store an indeterminate amount of values
// in one key... Failing that, just count occurrences of sorted values in a single slice


#[derive(Default)]
pub struct Rect {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

pub trait Sort {
    fn sort(&mut self);
}

impl<T> Sort for &'_ mut [T] 
where T: PartialOrd + Clone
{
    fn sort(&mut self) {
        let mut sorted = false;
        while !sorted {
            let mut change_needed = false;
            for i in 0..self.len() {
                if i == 0 {
                    continue;
                }
                if (self[i-1]) > (self[i]) {
                    change_needed = true;
                    let current = self[i].clone();
                    self[i]   = self[i-1].clone();
                    self[i-1] = current;
                }
                if i == self.len()-1 && !change_needed {
                    sorted = true;
                }
            }
        }
    }
}

pub struct DataWindow<'a, T> {
    pub data: &'a [T], // One is the data, the other is the key
    miny: f32,
    maxy: f32,
    minx: f32,
    maxx: f32,
    pub as_ratio: bool,
    pub margin: Rect, // percentage or fixed amount
}

impl<T> core::default::Default for DataWindow<'_, T> 
where T: core::default::Default {
    fn default() -> Self {
        DataWindow {
            data: &[],
            miny: 0.0,
            maxy: 100.0,
            minx: 0.0,
            maxx: 100.0,
            as_ratio: true,
            margin: Rect::default(),
        }
    }
}

impl<T> DataWindow<'_, T> where T: Ord + Default + Clone{
    // Can a slice argument take an array?
    pub fn new(dimensions: Rect, as_ratio: bool) -> Self {
        let (miny, maxy, minx, maxx) = (
            dimensions.bottom,
            dimensions.top,
            dimensions.left,
            dimensions.right
        );
        DataWindow {
            
            miny,
            maxy,
            minx,
            maxx,
            as_ratio,
            ..Default::default()
        }
    }

    pub fn from_data(data: &mut [T], bounds: Rect) -> Self {
        #[allow(unused_mut)]
        let mut sorted: &mut [T] = &mut [];
        data.clone_from_slice(sorted);
        sorted.sort();
        Self {
            data: sorted,
            miny: bounds.bottom,
            maxy: bounds.top,
            minx: bounds.left,
            maxx: bounds.right,
            as_ratio: false,
            margin: Rect::default(),
        }
    }

    pub fn set_fixed(&mut self) {
        self.as_ratio = false;
    }

    pub fn set_auto(&mut self) {
        self.as_ratio = true;
    }
}

#[allow(clippy::let_and_return)]
pub fn init() -> u32 {
    let _val = 1u32;
    _val
}

#[test]
pub fn lib_test1() {
    println!("Hello, Library!");
}

#[test]
pub fn sort_test() {
    let sorted = &mut [690, 69, 420, 42, 6969, 696, 696969];
    { dbg!(&sorted); }
    println!("Sorting `sorted`...");
    sorted.sort();
    { dbg!(&sorted); }
}

#[test]
pub fn sort_benchmark() {

    for _ in 0..5 {
        let start = std::time::Instant::now();

        let max = 100000;
        for _ in 0..max {
            let sorted = &mut [690, 69, 420, 42, 6969, 696, 696969];
            sorted.sort();
        }
        println!("{} runs took {}ms", max, (start.elapsed().as_micros() as f64) / 1000.0);
    }

}
