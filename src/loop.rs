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
        let mut init = 0;
        let mut count = 0;

        loop {
            if count == num {
                break;
            }

            count += 1;
            init += count;
        }

        return init;
    }
    let res = add_num(4);
    println!("{res}");
}
