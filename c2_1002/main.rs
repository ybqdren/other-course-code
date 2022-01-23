// 比较取巧的办法
fn avg_2(a: u32, b: u32) -> u32 {
    // 补充你的代码
    (a & b) + ((a^b) >> 1)
}



fn avg(a: u32, b: u32) -> u32 {
    if a > b {
        return  (a - b) / 2 + b;
    }

    return  (b - a) / 2 + a;
}


fn main() {
    assert_eq!(avg(4294967295, 4294967295), 4294967295);
    assert_eq!(avg(0, 0), 0);
    assert_eq!(avg(10, 20), 15);
    assert_eq!(avg(4294967295, 1), 2147483648);
    println!("passed")
}
