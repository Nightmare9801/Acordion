use criterion::{criterion_group, criterion_main, Criterion};

pub fn play_move(_move: i16, turn: bool) -> i16{
    let mut my_board = 0;
    let mut opp_board = 0;
    /*if turn {
        my_board |= _move;
    } else {
        opp_board |= _move;
    }*/
    my_board |= _move & -(turn as i16);
    opp_board |= _move & -(!turn as i16);
    //turn = !turn;
    my_board ^ opp_board << 1
    //println!("{}", format!("My Board: {:016b}", self.my_board));
    //println!("{}", format!("Opp Board: {:016b}", self.opp_board));
    //println!("{}", format!("Move: {:016b}", _move));
    /*
    Branchless
    Needs to be benched
    self.my_board |= _move & -(turn as i16);
    self.opp_board |= _move & -(!turn as i16);

     */
}

pub fn test()-> Vec<i16>{
    let mut x: Vec<i16>= Vec::new();
    for i in 0..1000000{
        let _i= play_move(1<<(i&1), i & 127 == 0);
        x.push(_i);
    }
    return x;
}




fn bench_prime_nos(c: &mut Criterion) {
    c.bench_function("Prime No Benchmark", |b| b.iter(|| test() ));
}

// Create a benchmark group
criterion_group!(benches, bench_prime_nos);

// Run the benchmarks
criterion_main!(benches);