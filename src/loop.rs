fn main() {
    // let mut a = 0;

    // loop {
    //     if a == 100 {
    //         break;
    //     }

    //     println!("{a}");
    //     a += 1;
    // }

    fn add_num(num: i32) -> i32 {
        let mut ans = 0;
        let mut count = 0;

        loop {
            if count == num {
                break;
            }
            count += 1;
            ans += count;
        }

        return ans;
    }
    let res = add_num(5);
    println!("{res}");
}
