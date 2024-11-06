use std::{arch::asm, io::{self, Write}};

fn main() {
    println!("\n==========================================");
    println!("       RDTSC-Probe VM Detector v1.0.0       ");
    println!("==========================================\n");
    println!(" VERDICT: {}", detect_vm());
    println!("\n==========================================\n");
    println!(""); let _ = io::stdout().flush();
    let mut input = String::new(); let _ = io::stdin().read_line(&mut input);
}

fn detect_vm() -> &'static str {
    let mut vm_score = 0;
    for _ in 0..3 {
        let mut t_score = [0; 4];
        for t_id in 0..4 {
            let mut results = Vec::new();
            for _ in 0..10 {
                let (t1, t2) = unsafe {
                    let (mut t1, mut t2): (u64, u64);
                    asm!("mfence","rdtsc","shl rdx, 32","or rax, rdx",out("rax") t1,out("rdx") _);
                    match t_id {
                        0 => { let mut x = 1u64; for _ in 0..100 { 
                            x = x.wrapping_mul(7).wrapping_add(1); std::hint::black_box(x); }},
                        1 => { let mut x = 1.0f64; for _ in 0..50 { 
                            x = x.sin().cos().sqrt().exp(); std::hint::black_box(x); }},
                        2 => { let mut sum = 0u64; for i in 0..100 {
                            sum = if i % 2 == 0 { sum.wrapping_add(i) }
                            else if i % 3 == 0 { sum.wrapping_mul(2) }
                            else if i % 5 == 0 { sum.wrapping_sub(i) }
                            else { sum.wrapping_div(2) }; std::hint::black_box(sum); }},
                        _ => { let data = vec![0u8; 4096]; let mut sum = 0u8;
                            for i in (0..data.len()).step_by(64) {
                            sum = sum.wrapping_add(data[i]); std::hint::black_box(sum); }}
                    };
                    asm!("mfence","rdtsc","shl rdx, 32","or rax, rdx",out("rax") t2,out("rdx") _);
                    (t1, t2)
                };
                results.push(t2.wrapping_sub(t1));
            }
            
            let avg = results.iter().sum::<u64>() as f64 / results.len() as f64;
            let max = *results.iter().max().unwrap();
            let min = *results.iter().min().unwrap();
            let rng = max - min;
            let var = results.iter().map(|&x| {
                let d = x as f64 - avg; d * d
            }).sum::<f64>() / results.len() as f64;
            let std = var.sqrt();
            let mut uniq = results.clone();
            uniq.sort_unstable();
            uniq.dedup();
            let uniq_cnt = uniq.len();

            t_score[t_id] = if (avg < 20.0 && uniq_cnt >= 5) ||
                                    (rng == 0 && uniq_cnt == 1) ||
                                    (std > 5000.0 && rng > 20000) ||
                                    (avg < 50.0 && uniq_cnt > 4) ||
                                    (rng == 0 && avg > 100.0) ||
                                    (std > 6000.0 && rng > 20000) ||
                                    (avg < 100.0 && std < 10.0 && uniq_cnt > 3) { 1 } else { 0 };
        }
        if t_score.iter().sum::<i32>() >= 2 { vm_score += 1; }
    }
    if vm_score > 1 { "[!] VIRTUAL MACHINE [!] " } else { "[-] No VM Detected [-]" }
}