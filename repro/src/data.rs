//! Loaders: MNIST-format IDX (gzip), TEXMEX `.fvecs`, DoOR CSVs, FlyWire CSVs.

use std::collections::{BTreeMap, HashMap};
use std::io::Read;
use std::path::Path;

use anyhow::{Context, Result, bail};
use flate2::read::GzDecoder;

use crate::Matrix;

fn read_gz(path: &Path) -> Result<Vec<u8>> {
    let f = std::fs::File::open(path).with_context(|| format!("open {}", path.display()))?;
    let mut out = Vec::new();
    GzDecoder::new(f)
        .read_to_end(&mut out)
        .with_context(|| format!("gunzip {}", path.display()))?;
    Ok(out)
}

fn be_u32(b: &[u8], at: usize) -> Result<usize> {
    let s = b.get(at..at + 4).context("truncated IDX header")?;
    Ok(u32::from_be_bytes([s[0], s[1], s[2], s[3]]) as usize)
}

/// IDX images as `f32` in [0, 255].
pub fn idx_images(path: &Path) -> Result<Matrix> {
    let b = read_gz(path)?;
    if be_u32(&b, 0)? != 2051 {
        bail!("{}: not an IDX image file", path.display());
    }
    let (n, r, c) = (be_u32(&b, 4)?, be_u32(&b, 8)?, be_u32(&b, 12)?);
    let px = b.get(16..16 + n * r * c).context("truncated IDX images")?;
    Ok(Matrix {
        n,
        d: r * c,
        data: px.iter().map(|&v| f32::from(v)).collect(),
    })
}

/// IDX labels.
pub fn idx_labels(path: &Path) -> Result<Vec<u8>> {
    let b = read_gz(path)?;
    if be_u32(&b, 0)? != 2049 {
        bail!("{}: not an IDX label file", path.display());
    }
    let n = be_u32(&b, 4)?;
    Ok(b.get(8..8 + n).context("truncated IDX labels")?.to_vec())
}

/// One MNIST-format split: images + labels.
pub fn mnist_split(dir: &Path, train: bool) -> Result<(Matrix, Vec<u8>)> {
    let p = if train { "train" } else { "t10k" };
    let x = idx_images(&dir.join(format!("{p}-images-idx3-ubyte.gz")))?;
    let y = idx_labels(&dir.join(format!("{p}-labels-idx1-ubyte.gz")))?;
    if x.n != y.len() {
        bail!("image/label count mismatch in {}", dir.display());
    }
    Ok((x, y))
}

/// TEXMEX `.fvecs` (little-endian: i32 dim, then dim f32, per vector).
pub fn fvecs(path: &Path) -> Result<Matrix> {
    let b = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    let d = i32::from_le_bytes(b.get(0..4).context("empty fvecs")?.try_into()?) as usize;
    let rec = 4 + 4 * d;
    if b.len() % rec != 0 {
        bail!("{}: size not a multiple of record", path.display());
    }
    let n = b.len() / rec;
    let mut data = Vec::with_capacity(n * d);
    for r in 0..n {
        let s = &b[r * rec + 4..(r + 1) * rec];
        data.extend(s.as_chunks::<4>().0.iter().map(|c| f32::from_le_bytes(*c)));
    }
    Ok(Matrix { n, d, data })
}

/// Hallem & Carlson 2006 receptors in DoOR 2.0, with their glomeruli.
/// Or33b is co-expressed in DM3 (with Or47a) and DM5 (with Or85a).
/// INFERRED from Couto et al. 2005 and Fishilevich & Vosshall 2005.
pub const HALLEM_RECEPTORS: [(&str, &[&str]); 24] = [
    ("Or2a", &["DA4m"]),
    ("Or7a", &["DL5"]),
    ("Or9a", &["VM3"]),
    ("Or10a", &["DL1"]),
    ("Or19a", &["DC1"]),
    ("Or22a", &["DM2"]),
    ("Or23a", &["DA3"]),
    ("Or33b", &["DM3", "DM5"]),
    ("Or35a", &["VC3"]),
    ("Or43a", &["DA4l"]),
    ("Or43b", &["VM2"]),
    ("Or47a", &["DM3"]),
    ("Or47b", &["VA1v"]),
    ("Or49b", &["VA5"]),
    ("Or59b", &["DM4"]),
    ("Or65a", &["DL3"]),
    ("Or67a", &["DM6"]),
    ("Or67c", &["VC4"]),
    ("Or82a", &["VA6"]),
    ("Or85a", &["DM5"]),
    ("Or85b", &["VM5d"]),
    ("Or85f", &["DL4"]),
    ("Or88a", &["VA1d"]),
    ("Or98a", &["VM5v"]),
];

/// Hallem 2006 odour responses: `(odour names, matrix odours x 24 receptors)`,
/// receptor order as in [`HALLEM_RECEPTORS`]. With `add_spontaneous` the
/// receptor's spontaneous rate is added (absolute firing rate); otherwise the
/// values are DoOR's `Hallem.2006.EN` as given (change from spontaneous).
/// Only odours with a value for every receptor are kept.
pub fn hallem(dir: &Path, add_spontaneous: bool) -> Result<(Vec<String>, Matrix)> {
    let mut per: Vec<HashMap<String, f32>> = Vec::new();
    let mut sfr: Vec<f32> = Vec::new();
    for (r, _) in HALLEM_RECEPTORS {
        let text = std::fs::read_to_string(dir.join(format!("{r}.csv")))?;
        let mut lines = text.lines();
        let header: Vec<String> = split_csv(lines.next().context("empty csv")?);
        let col = header
            .iter()
            .position(|h| h == "Hallem.2006.EN")
            .context("no Hallem column")?;
        let name_col = header
            .iter()
            .position(|h| h == "Name")
            .context("no Name column")?;
        let ik_col = header
            .iter()
            .position(|h| h == "InChIKey")
            .context("no InChIKey")?;
        // Data rows carry an extra leading row-name column (R write.csv2).
        let mut m = HashMap::new();
        let mut spont = None;
        for line in lines {
            let f = split_csv(line);
            if f.len() != header.len() + 1 {
                continue;
            }
            let v = &f[col + 1];
            if v == "NA" || v.is_empty() {
                continue;
            }
            let v: f32 = v.parse().with_context(|| format!("{r}: bad value {v}"))?;
            if f[ik_col + 1] == "SFR" {
                spont = Some(v);
            } else {
                m.insert(f[name_col + 1].clone(), v);
            }
        }
        sfr.push(spont.with_context(|| format!("{r}: no SFR row"))?);
        per.push(m);
    }
    let mut names: Vec<String> = per[0]
        .keys()
        .filter(|k| per.iter().all(|m| m.contains_key(*k)))
        .cloned()
        .collect();
    names.sort();
    let mut data = Vec::with_capacity(names.len() * 24);
    for n in &names {
        for (j, m) in per.iter().enumerate() {
            data.push(m[n] + if add_spontaneous { sfr[j] } else { 0.0 });
        }
    }
    Ok((
        names.clone(),
        Matrix {
            n: names.len(),
            d: 24,
            data,
        },
    ))
}

fn split_csv(line: &str) -> Vec<String> {
    line.split(';')
        .map(|s| s.trim().trim_matches('"').to_string())
        .collect()
}

/// A PN or KC in the FlyWire PN->KC subgraph.
#[derive(Debug, Clone)]
pub struct Cell {
    /// FlyWire root id.
    pub root_id: String,
    /// `left` / `right`.
    pub side: String,
    /// `uniglomerular` / `multiglomerular` / other.
    pub sub_class: String,
    /// Consolidated primary type (e.g. `DA1_lPN`).
    pub cell_type: String,
}

/// The FlyWire ALPN -> Kenyon_Cell subgraph.
#[derive(Debug, Clone)]
pub struct PnKc {
    /// PNs (sorted by root id).
    pub pns: Vec<Cell>,
    /// KCs (sorted by root id).
    pub kcs: Vec<Cell>,
    /// `(pn index, kc index, synapse count summed over neuropils)`.
    pub edges: Vec<(usize, usize, f32)>,
}

fn gz_lines(path: &Path) -> Result<String> {
    let b = read_gz(path)?;
    String::from_utf8(b).context("utf8")
}

/// Load the PN->KC subgraph with the filters of `research/wiring/PREREG.md`.
pub fn flywire_pn_kc(dir: &Path) -> Result<PnKc> {
    let cls_text = gz_lines(&dir.join("classification.csv.gz"))?;
    let mut lines = cls_text.lines();
    let h: Vec<&str> = lines
        .next()
        .context("empty classification")?
        .split(',')
        .collect();
    let pos = |name: &str| {
        h.iter()
            .position(|c| *c == name)
            .with_context(|| format!("no column {name}"))
    };
    let (c_id, c_class, c_sub, c_side) = (
        pos("root_id")?,
        pos("class")?,
        pos("sub_class")?,
        pos("side")?,
    );
    let mut info: HashMap<String, (String, String, String)> = HashMap::new();
    for l in lines {
        let f: Vec<&str> = l.split(',').collect();
        if f.len() < h.len() {
            continue;
        }
        let class = f[c_class];
        if class == "ALPN" || class == "Kenyon_Cell" {
            info.insert(
                f[c_id].to_string(),
                (
                    class.to_string(),
                    f[c_sub].to_string(),
                    f[c_side].to_string(),
                ),
            );
        }
    }
    let types_text = gz_lines(&dir.join("consolidated_cell_types.csv.gz"))?;
    let mut types: HashMap<String, String> = HashMap::new();
    for l in types_text.lines().skip(1) {
        let mut f = l.split(',');
        if let (Some(id), Some(t)) = (f.next(), f.next())
            && info.contains_key(id)
        {
            types.insert(id.to_string(), t.to_string());
        }
    }
    let conn = gz_lines(&dir.join("connections.csv.gz"))?;
    let mut lines = conn.lines();
    let h: Vec<&str> = lines
        .next()
        .context("empty connections")?
        .split(',')
        .collect();
    let pos = |name: &str| {
        h.iter()
            .position(|c| *c == name)
            .with_context(|| format!("no column {name}"))
    };
    let (c_pre, c_post, c_syn) = (pos("pre_root_id")?, pos("post_root_id")?, pos("syn_count")?);
    let mut pairs: BTreeMap<(String, String), f32> = BTreeMap::new();
    for l in lines {
        let f: Vec<&str> = l.split(',').collect();
        if f.len() < h.len() {
            continue;
        }
        let (a, b) = (f[c_pre], f[c_post]);
        let is = |id: &str, c: &str| info.get(id).is_some_and(|i| i.0 == c);
        if is(a, "ALPN") && is(b, "Kenyon_Cell") {
            let s: f32 = f[c_syn]
                .parse()
                .with_context(|| format!("syn_count {}", f[c_syn]))?;
            *pairs.entry((a.to_string(), b.to_string())).or_insert(0.0) += s;
        }
    }
    let mut pn_ids: Vec<&String> = pairs.keys().map(|k| &k.0).collect();
    pn_ids.sort();
    pn_ids.dedup();
    let mut kc_ids: Vec<&String> = pairs.keys().map(|k| &k.1).collect();
    kc_ids.sort();
    kc_ids.dedup();
    let cell = |id: &String| {
        let (_, sub, side) = info[id].clone();
        Cell {
            root_id: id.clone(),
            side,
            sub_class: sub,
            cell_type: types.get(id).cloned().unwrap_or_default(),
        }
    };
    let pns: Vec<Cell> = pn_ids.iter().map(|id| cell(id)).collect();
    let kcs: Vec<Cell> = kc_ids.iter().map(|id| cell(id)).collect();
    let pi: HashMap<&String, usize> = pn_ids.iter().enumerate().map(|(i, id)| (*id, i)).collect();
    let ki: HashMap<&String, usize> = kc_ids.iter().enumerate().map(|(i, id)| (*id, i)).collect();
    let edges = pairs.iter().map(|((a, b), &w)| (pi[a], ki[b], w)).collect();
    Ok(PnKc { pns, kcs, edges })
}

/// Split one CSV line, honouring double-quoted fields (`"a,b"`) and doubled
/// quotes inside them. The MCNS export quotes fields that contain commas.
pub fn split_quoted_csv(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match (c, quoted) {
            ('"', true) if chars.peek() == Some(&'"') => {
                cur.push('"');
                chars.next();
            }
            ('"', _) => quoted = !quoted,
            (',', false) => out.push(std::mem::take(&mut cur)),
            _ => cur.push(c),
        }
    }
    out.push(cur);
    out
}

/// The MCNS uniglomerular rule (PREREG.md amendment A2): a PN type is
/// uniglomerular iff it does not start with `M_` and the part before the
/// first `_` has no `+`. Returns that part (the glomerulus).
pub fn mcns_glomerulus(cell_type: &str) -> Option<&str> {
    if cell_type.starts_with("M_") {
        return None;
    }
    let g = cell_type.split('_').next()?;
    (!g.is_empty() && !g.contains('+') && cell_type.contains('_')).then_some(g)
}

/// Load the male-CNS (MCNS v1.0) ALPN -> Kenyon_Cell subgraph with the
/// filters of PREREG.md amendment A2: rows with `syn_count >= 5`, summed per
/// pair. PNs get `sub_class` `uniglomerular` / `multiglomerular` from
/// [`mcns_glomerulus`], so [`glomerulus`] works unchanged.
pub fn mcns_pn_kc(dir: &Path) -> Result<PnKc> {
    let text = gz_lines(&dir.join("neurons.csv.gz"))?;
    let mut lines = text.lines();
    let h = split_quoted_csv(lines.next().context("empty neurons.csv")?);
    let pos = |name: &str| {
        h.iter()
            .position(|c| c == name)
            .with_context(|| format!("no column {name}"))
    };
    let (c_id, c_class, c_side, c_type) = (
        pos("Root ID")?,
        pos("Class")?,
        pos("Soma side")?,
        pos("Primary Cell Type")?,
    );
    let mut info: HashMap<String, (String, String, String)> = HashMap::new();
    let mut types: HashMap<String, String> = HashMap::new();
    for l in lines {
        let f = split_quoted_csv(l);
        if f.len() < h.len() {
            continue;
        }
        let class = f[c_class].as_str();
        if class == "ALPN" || class == "Kenyon_Cell" {
            let sub = if class == "ALPN" {
                if mcns_glomerulus(&f[c_type]).is_some() {
                    "uniglomerular"
                } else {
                    "multiglomerular"
                }
            } else {
                ""
            };
            info.insert(
                f[c_id].clone(),
                (class.to_string(), sub.to_string(), f[c_side].clone()),
            );
            types.insert(f[c_id].clone(), f[c_type].clone());
        }
    }
    let conn = gz_lines(&dir.join("connections_princeton.csv.gz"))?;
    let mut lines = conn.lines();
    let h: Vec<&str> = lines
        .next()
        .context("empty connections")?
        .split(',')
        .collect();
    let pos = |name: &str| {
        h.iter()
            .position(|c| *c == name)
            .with_context(|| format!("no column {name}"))
    };
    let (c_pre, c_post, c_syn) = (pos("pre_root_id")?, pos("post_root_id")?, pos("syn_count")?);
    let mut pairs: BTreeMap<(String, String), f32> = BTreeMap::new();
    for l in lines {
        let f: Vec<&str> = l.split(',').collect();
        if f.len() < 4 {
            continue;
        }
        let (a, b) = (f[c_pre], f[c_post]);
        let is = |id: &str, c: &str| info.get(id).is_some_and(|i| i.0 == c);
        if is(a, "ALPN") && is(b, "Kenyon_Cell") {
            let s: f32 = f[c_syn]
                .parse()
                .with_context(|| format!("syn_count {}", f[c_syn]))?;
            if s >= 5.0 {
                *pairs.entry((a.to_string(), b.to_string())).or_insert(0.0) += s;
            }
        }
    }
    let mut pn_ids: Vec<&String> = pairs.keys().map(|k| &k.0).collect();
    pn_ids.sort();
    pn_ids.dedup();
    let mut kc_ids: Vec<&String> = pairs.keys().map(|k| &k.1).collect();
    kc_ids.sort();
    kc_ids.dedup();
    let cell = |id: &String| {
        let (_, sub, side) = info[id].clone();
        Cell {
            root_id: id.clone(),
            side,
            sub_class: sub,
            cell_type: types.get(id).cloned().unwrap_or_default(),
        }
    };
    let pns: Vec<Cell> = pn_ids.iter().map(|id| cell(id)).collect();
    let kcs: Vec<Cell> = kc_ids.iter().map(|id| cell(id)).collect();
    let pi: HashMap<&String, usize> = pn_ids.iter().enumerate().map(|(i, id)| (*id, i)).collect();
    let ki: HashMap<&String, usize> = kc_ids.iter().enumerate().map(|(i, id)| (*id, i)).collect();
    let edges = pairs.iter().map(|((a, b), &w)| (pi[a], ki[b], w)).collect();
    Ok(PnKc { pns, kcs, edges })
}

/// The glomerulus of a uniglomerular PN type (`DA1_lPN` -> `DA1`).
pub fn glomerulus(cell: &Cell) -> Option<&str> {
    if cell.sub_class != "uniglomerular" {
        return None;
    }
    cell.cell_type.split('_').next().filter(|g| !g.is_empty())
}

/// Read a feature file written by the `encoder` binary
/// (u32 n, u32 d, then n*d little-endian f32).
pub fn read_f32(path: &Path) -> Result<Matrix> {
    let b = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    let n = u32::from_le_bytes(b.get(0..4).context("short")?.try_into()?) as usize;
    let d = u32::from_le_bytes(b.get(4..8).context("short")?.try_into()?) as usize;
    let body = b.get(8..8 + 4 * n * d).context("truncated feature file")?;
    let data = body
        .as_chunks::<4>()
        .0
        .iter()
        .map(|c| f32::from_le_bytes(*c))
        .collect();
    Ok(Matrix { n, d, data })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mcns_csv_and_glomerulus_rule() {
        assert_eq!(
            split_quoted_csv(r#"1,"a, b",x,"say ""hi""",,"#),
            ["1", "a, b", "x", r#"say "hi""#, "", ""]
        );
        assert_eq!(mcns_glomerulus("DA1_lPN"), Some("DA1"));
        assert_eq!(mcns_glomerulus("VM5d_adPN"), Some("VM5d"));
        assert_eq!(mcns_glomerulus("M_vPNml53"), None);
        assert_eq!(mcns_glomerulus("VP1m+VP2_lvPN2"), None);
        assert_eq!(mcns_glomerulus(""), None);
        assert_eq!(mcns_glomerulus("KCg-m"), None);
    }
}
