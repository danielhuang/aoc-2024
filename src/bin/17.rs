#![feature(unbounded_shifts)]
use aoc_2024::*;
use z3::{
    Config, Context, Optimize, SatResult,
    ast::{Ast, BV},
};

fn combo(op: Z, registers: &[Z; 3]) -> Z {
    assert!(op >= 0);
    if op <= 3 {
        op
    } else if (4..=6).contains(&op) {
        registers[(op - 4) as usize]
    } else {
        unreachable!()
    }
}

fn combo_z3<'a>(op: Z, registers: &[BV<'a>; 3], ctx: &'a Context) -> BV<'a> {
    assert!(op >= 0);
    if op <= 3 {
        BV::from_i64(ctx, op as _, 64)
    } else if (4..=6).contains(&op) {
        registers[(op - 4) as usize].clone()
    } else {
        unreachable!()
    }
}

fn exec(program: &[Z], mut registers: [Z; 3]) -> Vec<Z> {
    let mut ip = 0;

    let mut output = vec![];

    while ip < program.len() {
        let ins = program[ip];
        let op = program[ip + 1];

        if ins == 0 {
            let num = registers[0];
            let den = combo(op, &registers);
            registers[0] = num >> den;
            ip += 2;
        } else if ins == 1 {
            let b = registers[1];
            registers[1] = b ^ op;
            ip += 2;
        } else if ins == 2 {
            let b = combo(op % 8, &registers) % 8;
            registers[1] = b;
            ip += 2;
        } else if ins == 3 {
            if registers[0] != 0 {
                ip = op.uint();
            } else {
                ip += 2;
            }
        } else if ins == 4 {
            let b = registers[1];
            let c = registers[2];
            registers[1] = b ^ c;
            ip += 2;
        } else if ins == 5 {
            output.push(combo(op % 8, &registers) % 8);
            ip += 2;
        } else if ins == 6 {
            let num = registers[0];
            let den = combo(op, &registers);
            registers[1] = num >> den;
            ip += 2;
        } else if ins == 7 {
            let num = registers[0];
            let den = combo(op, &registers);
            registers[2] = num.unbounded_shr(den as _);
            ip += 2;
        }
    }

    output
}

fn main() {
    let input = load_input();

    let [head, body] = input.paras().ca();

    let program = body.ints();

    let registers: [Z; 3] = head.ints().ca();

    cp(exec(&program, registers).ii().join(","));

    let ctx = Context::new(&Config::default());
    let opt = Optimize::new(&ctx);

    let a = BV::fresh_const(&ctx, "a", 64);

    let mut registers = [
        a.clone(),
        BV::from_i64(&ctx, 0, 64),
        BV::from_i64(&ctx, 0, 64),
    ];

    for (i, output) in program.cii().enumerate() {
        for (ins, op) in program.cii().tuples() {
            if ins == 0 {
                let op = combo_z3(op % 8, &registers, &ctx);
                registers[0] = registers[0].bvlshr(&op);
            } else if ins == 1 {
                let b = registers[1].clone();
                registers[1] = b ^ BV::from_i64(&ctx, op as _, 64);
            } else if ins == 2 {
                let b = combo_z3(op % 8, &registers, &ctx).bvurem(&BV::from_i64(&ctx, 8, 64));
                registers[1] = b;
            } else if ins == 3 {
                // Assuming this only occurs as the last instruction
                assert!(op == 0);
                if i == program.len() - 1 {
                    opt.assert(&registers[0]._eq(&BV::from_i64(&ctx, 0, 64)));
                } else {
                    opt.assert(&!registers[0]._eq(&BV::from_i64(&ctx, 0, 64)));
                }
            } else if ins == 4 {
                let b = registers[1].clone();
                let c = registers[2].clone();
                registers[1] = b ^ c;
            } else if ins == 5 {
                let op = combo_z3(op % 8, &registers, &ctx);
                opt.assert(&op.bvurem(&BV::from_i64(&ctx, 8, 64))._eq(&BV::from_i64(
                    &ctx,
                    output as _,
                    64,
                )));
            } else if ins == 6 {
                let op = combo_z3(op % 8, &registers, &ctx);
                registers[1] = registers[0].bvlshr(&op);
            } else if ins == 7 {
                let op = combo_z3(op % 8, &registers, &ctx);
                registers[2] = registers[0].bvlshr(&op);
            }
        }
    }

    opt.minimize(&a);

    assert!(opt.check(&[]) == SatResult::Sat);

    let model = opt.get_model().unwrap();
    cp(model.eval(&a, true).unwrap().as_i64().unwrap());
}
