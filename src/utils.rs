// use std::fs::File;
// use std::io::Read;

//读取文件
// pub fn read_file(content: &mut String) {
//     let mut file = File::open("./params.json").unwrap();
//     file.read_to_string(content).unwrap();
// }

#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
pub fn random() -> f64{
    rand::random::<f64>()
}

#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
pub fn random() -> f64{
    use stdweb::unstable::TryInto;
    return js! {return Math.random();}.try_into().unwrap();
}

pub fn sqrt_usize(val: &usize) -> usize {
    let f = *val as f32;
    f.sqrt() as usize
}

//返回-1 <n <1范围内的随机浮点数
// pub fn random_clamped() -> f32 {
//     rand::random::<f32>() - rand::random::<f32>()
// }
pub fn random_clamped() -> f64 {
    random() - random()
}

pub fn rand_int(l: i32, b: i32) -> i32 {
	(random()*(b as f64 - l as f64 + 1.0)).floor() as i32 + l
}

pub fn rand_usize(l: usize, b: usize) -> usize {
	(random()*(b as f64 - l as f64 + 1.0)).floor() as usize + l
}

// pub fn random_float_64() -> f64 {
//     rand::random::<f64>()
// }
//返回[low, low] 区间的数
// pub fn random_int(low: i32, high: i32) -> i32 {
//     if high < low {
//         return low;
//     }
//     //返回[low, high)区间的数
//     //println!("low={},high={}", low, high);
    
//     rand::thread_rng().gen_range(low, high + 1)
// }
//返回[low, low] 区间的数
// pub fn random_usize(low: usize, high: usize) -> usize {
//     if high < low {
//         return low;
//     }
//     rand::thread_rng().gen_range(low as i32, high as i32 + 1) as usize
// }

pub fn clamp(arg: &mut f32, min: f32, max: f32) {
    if *arg < min {
        *arg = min;
    }
    if *arg > max {
        *arg = max;
    }
}
