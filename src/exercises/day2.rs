use rand::Rng;

/// 随机生成双色球号码
/// note: 红色球前6组数字  1-34 且不重复， 蓝色球 1-34
pub fn generate_lottery_nums() -> [u8; 7] {
    let mut arr: [u8; 7] = [0; 7];
    let mut balls = vec![];
    for i in 0..34 {
        balls.push(i + 1);
    }
    let mut rng = rand::thread_rng();
    // 红色球
    for i in 0..arr.len() - 1 {
        let index = rng.gen_range(0..balls.len());
        arr[i] = balls[index];
        balls.remove(index);
    }
    // 红色球排序
    arr[6] = 127;
    arr.sort();
    // 蓝色球
    arr[6] = rng.gen_range(1..35);
    arr
}