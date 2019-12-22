pub mod process {
    trait StripFloat {
        fn strip(&self) -> i64;
    }

    impl StripFloat for f64 {
        fn strip(&self) -> i64 {
            format!("{0:.2}", self).replace('.', "").parse::<i64>().unwrap()
        }
    }

    enum Corners {
        Left,
        Right,
        None,
    }

    use crate::node::node::Node;

    pub fn process(list: &mut Vec<Node>) {
        let length = list.len();
        println!("List length - {}", length);
        let mut i: usize = 0;
        let mut iterations: u64 = 0;
        print_list(list);
        loop {
            let corner = is_corner_case(i, length);
            if push(list, i, corner) {
                break;
            }
            i += 1;
            if i == (length)  {
                i = 0;
            }
            iterations += 1;
            if iterations > 12000 {
                break;
            }
            print_list(list);
        }
        print_list(list);
        println!("Total iterations = {0}", iterations);
    }

    fn is_corner_case(i: usize, length: usize) -> Corners {
        println!("Corner - {}", i);
        if i == 0 { return Corners::Left }
        if i == (length - 1) { return Corners::Right }
        Corners::None
    }

    fn print_list(list: &Vec<Node>) {
        let (flatten1, flatten2): (Vec<String>, Vec<String>) = list.iter()
            .map(|i| (i.pushed.to_string(), i.weight.to_string())).unzip();
        print!("\n{0}", flatten1.join("   "));
        print!("\n{0}\n", flatten2.join("   "));
        let mut delimiter = String::from("-");
        flatten1.join("  ").chars().for_each(|_i| delimiter.push('-'));
        delimiter.push('-');
        print!("{0}", delimiter);
    }

    fn push(list: &mut Vec<Node>, i: usize, corner: Corners) -> bool {
        let current: usize = i;
        let mut finished = true;
        let current_sum = list[current].weight + list[current].pushed;
        match corner {
            Corners::Left => {
                println!("Left");
                let right: usize = i + 1;
                let right_sum = &list[right].weight + &list[right].pushed;
                if current_sum.strip() < right_sum.strip() {
                    let half = list[right].pushed / 2.0;
                    list[right].pushed -= half;
                    list[current].pushed += half;
                    finished = false
                }
            }
            Corners::Right => {
                println!("Right");
                let left: usize = i - 1;
                let left_sum = list[left].weight + list[left].pushed;
                if current_sum.strip() < left_sum.strip() {
                    let half = list[left].pushed / 2.0;
                    list[left].pushed -= half;
                    list[current].pushed += half;
                    finished = false;
                }
            }
            Corners::None => {
                println!("None");
                let right: usize = i + 1;
                let left: usize = i - 1;
                let right_sum = list[right].weight + list[right].pushed;
                if current_sum.strip() < right_sum.strip() {
                    let half = list[right].pushed / 2.0;
                    list[right].pushed -= half;
                    list[current].pushed += half;
                    finished = false;
                } else {
                let left_sum = list[left].weight + list[left].pushed;
                if current_sum.strip() < left_sum.strip() {
                    let half = list[left].pushed / 2.0;
                    list[left].pushed -= half;
                    list[current].pushed += half;
                    finished = false;
                }
                }
            }
        }
        finished
    }
}
