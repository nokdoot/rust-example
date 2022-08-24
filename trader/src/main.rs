use std::env;

fn main() {
    let args: Vec<_> = env::args().take(4).collect();
    let [program, base, tick, step] = match &*args {
        [program, base, tick, step] => [program, base, tick, step],
        _ => panic!(),
    };

    list_price(
        match base.parse::<i32>() {
            Ok(x) => x,
            _ => panic!("base type is not i32")
        },
        match tick.parse::<i32>() {
            Ok(x) => x,
            _ => panic!("tick type is not i32")
        },
        10,
        match step.parse::<i32>() {
            Ok(x) => x,
            _ => panic!("step type is not i32")
        }
    );
}

fn floor_by_tick(value: i32, tick: i32, math_fn: &str) -> i32 {
    let r = value % tick;
    match math_fn {
        "ceil" => {
            if r == 0 {
                value
            }
            else {
                value - r + tick
            }
        },
        "floor" => {
            value - r
        },
        _ => {
            1
        }
    }
}

fn new_base(base: i32, tick: i32, step: i32) -> i32 {
    let rebase = base * (100 + step) / 100;
    if step < 0 {
        floor_by_tick(rebase, tick, "floor")
    }
    else {
        floor_by_tick(rebase, tick, "ceil")
    }

}

fn list_under_price(mut base: i32, tick: i32, count: i32, step: i32) -> Vec<i32> {
    let mut array = Vec::new();
    for _i in 1..count {
        base = new_base(base, tick, step);
        array.push(base);
    }
    array.reverse();
    array
}

fn list_upper_price(mut base: i32, tick: i32, count: i32, step: i32) -> Vec<i32> {
    let mut array = Vec::new();
    for _i in 1..count {
        base = new_base(base, tick, step);
        array.push(base);
    }
    array
}

fn list_price(base: i32, tick: i32, count: i32, step: i32) {
    let mut under = list_under_price(base, tick, count, -step);
    let mut upper = list_upper_price(base, tick, count, step);
    let mut prices = Vec::new();
    prices.append(&mut under);
    prices.push(base);
    prices.append(&mut upper);
    println!("{:?}", prices);
}