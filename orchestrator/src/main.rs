use std::time::{Duration, Instant};

use cpu_ai_net_core::Phase;

fn main() {
    let start = Instant::now();
    let budget = Duration::from_millis(800);

    let mut phase = Phase::Gather;

    loop {
        match phase {
            Phase::Gather => {
                println!("[Gather] collect external knowledge / queries");
                phase = Phase::Generate;
            }
            Phase::Generate => {
                println!("[Generate] spawn hypothesis/evidence packets");
                phase = Phase::Refute;
            }
            Phase::Refute => {
                println!("[Refute] generate refutations & counter-evidence");
                phase = Phase::Aggregate;
            }
            Phase::Aggregate => {
                println!("[Aggregate] update confidence / select top claims");
                phase = Phase::Freeze;
            }
            Phase::Freeze => {
                println!("[Freeze] finalize result + persona rewrite");
                break;
            }
        }

        if start.elapsed() > budget {
            println!("[TIMEOUT] budget exceeded -> forcing Freeze");
            break;
        }
    }

    println!("done in {:?}", start.elapsed());
}
