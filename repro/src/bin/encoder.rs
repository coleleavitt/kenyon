//! Frozen encoder for R3: LeNet5 (with batch norm) trained on KMNIST, as in
//! Shen, Dasgupta & Navlakha 2021, Methods (p.11): Adam, lr 0.001, batch 64,
//! 25 epochs. The 84-unit penultimate layer (after ReLU) is extracted for
//! MNIST and Fashion-MNIST train/test and written as little-endian f32 files.
//!
//! Usage: `encoder [seed] [epochs]`. Output:
//! `$KENYON_DATA/kenyon-features/lenet5_kmnist_s{seed}/{mnist,fashion}_{train,test}.f32`.

use std::io::Write;

use anyhow::{Context, Result};
use candle_core::{D, DType, Device, Tensor};
use candle_nn::{
    BatchNorm, BatchNormConfig, Conv2d, Conv2dConfig, Linear, ModuleT, Optimizer, ParamsAdamW,
    VarBuilder, VarMap,
};
use kenyon::Rng;
use kenyon_repro::{Matrix, data, data_dir};

struct LeNet5 {
    c1: Conv2d,
    b1: BatchNorm,
    c2: Conv2d,
    b2: BatchNorm,
    f1: Linear,
    b3: BatchNorm,
    f2: Linear,
    b4: BatchNorm,
    f3: Linear,
}

impl LeNet5 {
    fn new(vb: VarBuilder) -> Result<Self> {
        let bn = BatchNormConfig::default();
        Ok(Self {
            c1: candle_nn::conv2d(
                1,
                6,
                5,
                Conv2dConfig {
                    padding: 2,
                    ..Default::default()
                },
                vb.pp("c1"),
            )?,
            b1: candle_nn::batch_norm(6, bn, vb.pp("bn1"))?,
            c2: candle_nn::conv2d(6, 16, 5, Default::default(), vb.pp("c2"))?,
            b2: candle_nn::batch_norm(16, bn, vb.pp("bn2"))?,
            f1: candle_nn::linear(400, 120, vb.pp("f1"))?,
            b3: candle_nn::batch_norm(120, bn, vb.pp("bn3"))?,
            f2: candle_nn::linear(120, 84, vb.pp("f2"))?,
            b4: candle_nn::batch_norm(84, bn, vb.pp("bn4"))?,
            f3: candle_nn::linear(84, 10, vb.pp("f3"))?,
        })
    }

    /// Returns (features after the 84-unit ReLU, logits).
    fn forward(&self, x: &Tensor, train: bool) -> Result<(Tensor, Tensor)> {
        let x = self
            .b1
            .forward_t(&x.apply(&self.c1)?, train)?
            .relu()?
            .max_pool2d(2)?;
        let x = self
            .b2
            .forward_t(&x.apply(&self.c2)?, train)?
            .relu()?
            .max_pool2d(2)?;
        let x = x.flatten_from(1)?;
        let x = self.b3.forward_t(&x.apply(&self.f1)?, train)?.relu()?;
        let feat = self.b4.forward_t(&x.apply(&self.f2)?, train)?.relu()?;
        let logits = feat.apply(&self.f3)?;
        Ok((feat, logits))
    }
}

/// Seeded init (PyTorch-style uniform(+-1/sqrt(fan_in)) for conv/linear
/// weights and biases; batch-norm parameters keep their defaults).
fn seed_init(vm: &VarMap, seed: u64) -> Result<()> {
    let data = vm
        .data()
        .lock()
        .map_err(|_| anyhow::anyhow!("varmap lock poisoned"))?;
    let mut names: Vec<&String> = data.keys().collect();
    names.sort();
    let mut rng = Rng::derive(seed, 0x1e7e);
    for name in names {
        if name.starts_with("bn") {
            continue;
        }
        let var = &data[name];
        let shape = var.shape().clone();
        let dims = shape.dims().to_vec();
        let fan_in = if name.ends_with(".weight") {
            dims.iter().skip(1).product::<usize>()
        } else {
            // bias: fan_in of the matching weight
            let w = name.replace(".bias", ".weight");
            data.get(&w)
                .map_or(1, |v| v.shape().dims().iter().skip(1).product::<usize>())
        };
        let b = 1.0 / (fan_in as f64).sqrt();
        let vals: Vec<f32> = (0..shape.elem_count())
            .map(|_| ((rng.next_f64() * 2.0 - 1.0) * b) as f32)
            .collect();
        var.set(&Tensor::from_vec(vals, shape, &Device::Cpu)?)?;
    }
    Ok(())
}

fn to_tensor(x: &Matrix, idx: &[usize]) -> Result<Tensor> {
    let mut v = Vec::with_capacity(idx.len() * 784);
    for &i in idx {
        v.extend(x.row(i).iter().map(|p| p / 255.0));
    }
    Ok(Tensor::from_vec(v, (idx.len(), 1, 28, 28), &Device::Cpu)?)
}

fn features(model: &LeNet5, x: &Matrix) -> Result<(Vec<f32>, Vec<u32>)> {
    let mut feats = Vec::with_capacity(x.n * 84);
    let mut preds = Vec::with_capacity(x.n);
    for start in (0..x.n).step_by(1000) {
        let idx: Vec<usize> = (start..(start + 1000).min(x.n)).collect();
        let (f, logits) = model.forward(&to_tensor(x, &idx)?, false)?;
        feats.extend(f.flatten_all()?.to_vec1::<f32>()?);
        preds.extend(logits.argmax(D::Minus1)?.to_vec1::<u32>()?);
    }
    Ok((feats, preds))
}

fn write_f32(path: &std::path::Path, n: usize, d: usize, v: &[f32]) -> Result<()> {
    let mut f = std::io::BufWriter::new(
        std::fs::File::create(path).with_context(|| path.display().to_string())?,
    );
    f.write_all(&(n as u32).to_le_bytes())?;
    f.write_all(&(d as u32).to_le_bytes())?;
    for x in v {
        f.write_all(&x.to_le_bytes())?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let seed: u64 = std::env::args()
        .nth(1)
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(0);
    let epochs: usize = std::env::args()
        .nth(2)
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(25);
    let dd = data_dir();
    let (kx, ky) = data::mnist_split(&dd.join("kmnist"), true)?;
    let (kxt, kyt) = data::mnist_split(&dd.join("kmnist"), false)?;
    let vm = VarMap::new();
    let model = LeNet5::new(VarBuilder::from_varmap(&vm, DType::F32, &Device::Cpu))?;
    seed_init(&vm, seed)?;
    let mut opt = candle_nn::AdamW::new(
        vm.all_vars(),
        ParamsAdamW {
            lr: 1e-3,
            weight_decay: 0.0,
            ..Default::default()
        },
    )?;
    let mut rng = Rng::derive(seed, 0xba7c);
    for ep in 0..epochs {
        let mut order: Vec<usize> = (0..kx.n).collect();
        rng.shuffle(&mut order);
        let mut loss_sum = 0f64;
        let mut nb = 0;
        for b in order.chunks(64) {
            let xb = to_tensor(&kx, b)?;
            let yb = Tensor::from_vec(
                b.iter().map(|&i| u32::from(ky[i])).collect::<Vec<_>>(),
                b.len(),
                &Device::Cpu,
            )?;
            let (_, logits) = model.forward(&xb, true)?;
            let loss = candle_nn::loss::cross_entropy(&logits, &yb)?;
            opt.backward_step(&loss)?;
            loss_sum += f64::from(loss.to_scalar::<f32>()?);
            nb += 1;
        }
        let (_, p) = features(&model, &kxt)?;
        let acc = p
            .iter()
            .zip(&kyt)
            .filter(|(a, b)| **a == u32::from(**b))
            .count() as f64
            / kxt.n as f64;
        eprintln!(
            "epoch {ep}: loss {:.4} kmnist test acc {acc:.4}",
            loss_sum / nb as f64
        );
    }
    let out = dd
        .join("kenyon-features")
        .join(format!("lenet5_kmnist_s{seed}"));
    std::fs::create_dir_all(&out)?;
    let (_, p) = features(&model, &kxt)?;
    let acc = p
        .iter()
        .zip(&kyt)
        .filter(|(a, b)| **a == u32::from(**b))
        .count() as f64
        / kxt.n as f64;
    std::fs::write(
        out.join("encoder.txt"),
        format!("seed {seed}\nepochs {epochs}\nkmnist_test_acc {acc:.4}\n"),
    )?;
    for (name, dir) in [("mnist", "mnist"), ("fashion", "fashion-mnist")] {
        for train in [true, false] {
            let (x, _) = data::mnist_split(&dd.join(dir), train)?;
            let (f, _) = features(&model, &x)?;
            let split = if train { "train" } else { "test" };
            write_f32(&out.join(format!("{name}_{split}.f32")), x.n, 84, &f)?;
        }
    }
    eprintln!("wrote {} (kmnist test acc {acc:.4})", out.display());
    Ok(())
}
