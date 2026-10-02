//! R3: FlyModel class-incremental MNIST-20 (Shen, Dasgupta & Navlakha 2021).
//! Protocol and bars: RESULTS.md part 1, R3.
//! Usage: `r3_flymodel [encoder_seed]` (features from the `encoder` binary).

use anyhow::Result;
use kenyon::{Coding, FlyModel, FlyModelParams, LearningRule};
use kenyon_repro::stats::{mean, sd};
use kenyon_repro::{Matrix, data, data_dir, out_dir};
use rayon::prelude::*;

struct Split {
    x: Matrix,
    y: Vec<usize>,
}

fn load(feat: &std::path::Path, raw: bool, train: bool) -> Result<Split> {
    let split = if train { "train" } else { "test" };
    let dd = data_dir();
    let (mut xs, mut ys) = (Vec::new(), Vec::new());
    let mut d = 0;
    for (name, dir, off) in [("mnist", "mnist", 0usize), ("fashion", "fashion-mnist", 10)] {
        let (img, lab) = data::mnist_split(&dd.join(dir), train)?;
        let x = if raw {
            img
        } else {
            data::read_f32(&feat.join(format!("{name}_{split}.f32")))?
        };
        anyhow::ensure!(
            x.n == lab.len(),
            "feature/label count mismatch for {name} {split}"
        );
        d = x.d;
        xs.extend_from_slice(&x.data);
        ys.extend(lab.iter().map(|&l| usize::from(l) + off));
    }
    Ok(Split {
        x: Matrix {
            n: ys.len(),
            d,
            data: xs,
        },
        y: ys,
    })
}

/// Returns accuracy on seen classes after each task, and per-task memory loss.
fn run(p: FlyModelParams, tr: &Split, te: &Split) -> Result<(Vec<f64>, Vec<f64>)> {
    let mut m = FlyModel::new(p)?;
    let acc_on = |m: &mut FlyModel, classes: &[usize]| -> Result<f64> {
        let idx: Vec<usize> = (0..te.n())
            .filter(|&i| classes.contains(&te.y[i]))
            .collect();
        let mut ok = 0usize;
        for &i in &idx {
            if m.predict(te.x.row(i))? == te.y[i] {
                ok += 1;
            }
        }
        Ok(ok as f64 / idx.len().max(1) as f64)
    };
    let mut seen_acc = Vec::new();
    let mut immediate = Vec::new();
    for t in 0..10 {
        for c in [2 * t, 2 * t + 1] {
            for i in (0..tr.n()).filter(|&i| tr.y[i] == c) {
                m.train_one(tr.x.row(i), c)?;
            }
        }
        let seen: Vec<usize> = (0..2 * t + 2).collect();
        seen_acc.push(acc_on(&mut m, &seen)?);
        immediate.push(acc_on(&mut m, &[2 * t, 2 * t + 1])?);
    }
    let mut loss = Vec::new();
    for t in 0..10 {
        loss.push(immediate[t] - acc_on(&mut m, &[2 * t, 2 * t + 1])?);
    }
    Ok((seen_acc, loss))
}

impl Split {
    fn n(&self) -> usize {
        self.y.len()
    }
}

fn main() -> Result<()> {
    let enc_seed: u64 = std::env::args()
        .nth(1)
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(0);
    let feat = data_dir()
        .join("kenyon-features")
        .join(format!("lenet5_kmnist_s{enc_seed}"));
    let tr = load(&feat, false, true)?;
    let te = load(&feat, false, false)?;
    eprintln!("features: train {} x {}, test {}", tr.n(), tr.x.d, te.n());
    let base = FlyModelParams::mnist20(0);
    let variants: Vec<(&str, FlyModelParams)> = vec![
        ("flymodel", base),
        (
            "abl_dense_coding",
            FlyModelParams {
                coding: Coding::Dense,
                ..base
            },
        ),
        (
            "abl_perceptron_v1",
            FlyModelParams {
                rule: LearningRule::PerceptronV1,
                ..base
            },
        ),
        (
            "abl_perceptron_v2",
            FlyModelParams {
                rule: LearningRule::PerceptronV2,
                ..base
            },
        ),
        (
            "abl_perceptron_v3",
            FlyModelParams {
                rule: LearningRule::PerceptronV3,
                ..base
            },
        ),
        (
            "abl_dense_perceptron_v1",
            FlyModelParams {
                coding: Coding::Dense,
                rule: LearningRule::PerceptronV1,
                ..base
            },
        ),
    ];
    let mut out = Vec::new();
    println!(
        "| variant | acc@10 classes | acc@20 classes | mean memory loss | sd acc@10 | sd acc@20 |"
    );
    println!("|---|---|---|---|---|---|");
    let mut record = |name: String, runs: Vec<(Vec<f64>, Vec<f64>)>| {
        let a10: Vec<f64> = runs.iter().map(|r| r.0[4]).collect();
        let a20: Vec<f64> = runs.iter().map(|r| r.0[9]).collect();
        let ml: Vec<f64> = runs.iter().map(|r| mean(&r.1)).collect();
        let curve: Vec<String> = (0..10)
            .map(|t| {
                format!(
                    "{:.4}",
                    mean(&runs.iter().map(|r| r.0[t]).collect::<Vec<_>>())
                )
            })
            .collect();
        println!(
            "| {name} | {:.3} | {:.3} | {:.3} | {:.4} | {:.4} |",
            mean(&a10),
            mean(&a20),
            mean(&ml),
            sd(&a10),
            sd(&a20)
        );
        out.push(format!(
            "    {{\"variant\": \"{name}\", \"acc10\": {:.4}, \"acc20\": {:.4}, \"memory_loss\": {:.4}, \"sd10\": {:.4}, \"sd20\": {:.4}, \"curve\": [{}]}}",
            mean(&a10),
            mean(&a20),
            mean(&ml),
            sd(&a10),
            sd(&a20),
            curve.join(", ")
        ));
    };
    for (name, p) in variants {
        let runs: Vec<(Vec<f64>, Vec<f64>)> = (0..5u64)
            .into_par_iter()
            .map(|s| run(FlyModelParams { seed: s, ..p }, &tr, &te))
            .collect::<Result<_>>()?;
        record(name.to_string(), runs);
    }
    // Frozen-encoder control: raw pixels (m = 40 d, 10% fan-in, l = m/20).
    let trr = load(&feat, true, true)?;
    let ter = load(&feat, true, false)?;
    let raw = FlyModelParams {
        n_inputs: 784,
        n_kc: 31_360,
        fan_in: 78,
        n_active: 1568,
        ..base
    };
    let runs: Vec<(Vec<f64>, Vec<f64>)> = (0..5u64)
        .into_par_iter()
        .map(|s| run(FlyModelParams { seed: s, ..raw }, &trr, &ter))
        .collect::<Result<_>>()?;
    record("ctl_raw_pixels_no_encoder".to_string(), runs);
    let enc = std::fs::read_to_string(feat.join("encoder.txt"))
        .unwrap_or_default()
        .replace('\n', "; ");
    std::fs::write(
        out_dir().join(format!("r3_flymodel_enc{enc_seed}.json")),
        format!(
            "{{\n  \"experiment\": \"r3_flymodel_mnist20\",\n  \"encoder\": \"{enc}\",\n  \"rows\": [\n{}\n  ]\n}}\n",
            out.join(",\n")
        ),
    )?;
    Ok(())
}
