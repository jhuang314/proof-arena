// migrated from: https://github.com/PolyhedraZK/ExpanderCompilerCollection/blob/939cccbe0ff25a3f7c9dc2129131be3124c63589/expander_compiler/tests/keccak_gf2_full.rs
use expander_compiler::frontend::*;
mod spj;
pub use spj::*;

#[inline(always)]
pub fn int2bit<C: Config>(api: &mut API<C>, value: u32) -> Vec<Variable> {
    return (0..32).map(|x| api.constant(((value >> x) & 1) as u32)).collect();
}

#[inline(always)]
pub fn rotate_right(bits: &Vec<Variable>, k: usize) -> Vec<Variable> {
    let n = bits.len();
    let s = k & (n - 1);
    let mut new_bits = bits[s as usize..].to_vec();
    new_bits.append(&mut bits[0..s as usize].to_vec());
    new_bits
}

#[inline(always)]
pub fn shift_right<C: Config>(api: &mut API<C>, bits: Vec<Variable>, k: usize) -> Vec<Variable> {
    let n = bits.len();
    let s = k & (n - 1);
    let mut new_bits = bits[s as usize..].to_vec();
    new_bits.append(&mut vec![api.constant(0); s]);
    new_bits
}

// Ch function: (x AND y) XOR (NOT x AND z)
#[inline(always)]
pub fn ch<C: Config>(api: &mut API<C>, x: Vec<Variable>, y: Vec<Variable>, z: Vec<Variable>) -> Vec<Variable> {
    let xy = and(api, x.clone(), y.clone());
    let not_x = not(api, x.clone());
    let not_xz = and(api, not_x, z.clone());
    
    xor(api, xy, not_xz)
}


// Maj function: (x AND y) XOR (x AND z) XOR (y AND z)
#[inline(always)]
pub fn maj<C: Config>(api: &mut API<C>, x: Vec<Variable>, y: Vec<Variable>, z: Vec<Variable>) -> Vec<Variable> {
    let xy = and(api, x.clone(), y.clone());
    let xz = and(api, x.clone(), z.clone());
    let yz = and(api, y.clone(), z.clone());
    let tmp = xor(api, xy, xz);

    xor(api, tmp, yz)
}

// Sigma0 function: ROTR(x, 2) XOR ROTR(x, 13) XOR ROTR(x, 22)
#[inline(always)]
pub fn sigma0<C: Config>(api: &mut API<C>, x: Vec<Variable>) -> Vec<Variable> {
    let rot2 = rotate_right(&x, 2);
    let rot13 = rotate_right(&x, 13);
    let rot22 = rotate_right(&x, 22);
    let tmp = xor(api, rot2, rot13);

    xor(api, tmp, rot22)
}

// Sigma1 function: ROTR(x, 6) XOR ROTR(x, 11) XOR ROTR(x, 25)
#[inline(always)]
pub fn sigma1<C: Config>(api: &mut API<C>, x: Vec<Variable>) -> Vec<Variable> {
    let rot6 = rotate_right(&x, 6);
    let rot11 = rotate_right(&x, 11);
    let rot25 = rotate_right(&x, 25);
    let tmp = xor(api, rot6, rot11);

    xor(api, tmp, rot25)
}

pub const N_HASHES: usize = 1;

#[inline(always)]
pub fn add_const<C: Config>(api: &mut API<C>, a: Vec<Variable>, b: u32) -> Vec<Variable> {
    let n = a.len();
    let mut c = a.clone();
    let mut ci = api.constant(0);
    for i in 0..n {

        if b >> i & 1 == 1 {
            let p = api.add(a[i].clone(), 1);
            c[i] = api.add(p.clone(), ci.clone());

            ci = api.mul(ci, p);
            ci = api.add(ci, a[i].clone());
        } else {
            c[i] = api.add(c[i], ci.clone());
            ci = api.mul(ci, a[i].clone());
        }
    }
    c
}

#[inline(always)]
fn add_brentkung<C: Config>(api: &mut API<C>, a: &Vec<Variable>, b: &Vec<Variable>) -> Vec<Variable> {
    let mut c = vec![api.constant(0); 32];
    let ci = api.constant(0);

    for i in 0..8 {
        let start = i * 4;
        let end = start + 4;

        let (sum, ci) = brent_kung_adder_4_bits(api, &a[start..end].to_vec(), &b[start..end].to_vec(), ci);

        c[start..end].copy_from_slice(&sum);
    }

    c
}

#[inline(always)]
fn brent_kung_adder_4_bits<C: Config>(api: &mut API<C>, a: &Vec<Variable>, b: &Vec<Variable>, carry_in: Variable) -> ([Variable; 4], Variable) {
    let mut g = [api.constant(0); 4];
    let mut p = [api.constant(0); 4];

    // Step 1: Generate and propagate
    for i in 0..4 {
        g[i] = api.mul(a[i], b[i]);
        p[i] = api.add(a[i], b[i]);
    }

    // Step 2: Prefix computation
    let p1g0 = api.mul(p[1], g[0]);
    let p0p1 = api.mul(p[0], p[1]);
    let p2p3 = api.mul(p[2], p[3]);

    let g10 = api.add(g[1], p1g0);
    let g20 = api.mul(p[2], g10);
    let g20 = api.add(g[2], g20);
    let g30 = api.mul(p[3], g20);
    let g30 = api.add(g[3], g30);

    // Step 3: Calculate carries
    let mut c = [api.constant(0); 5];
    c[0] = carry_in;
    let tmp = api.mul(p[0], c[0]);
    c[1] = api.add(g[0], tmp);
    let tmp = api.mul(p0p1, c[0]);
    c[2] = api.add(g10, tmp);
    let tmp = api.mul(p[2], c[0]);
    let tmp = api.mul(p0p1, tmp);
    c[3] = api.add(g20, tmp);
    let tmp = api.mul(p0p1, p2p3);
    let tmp = api.mul(tmp, c[0]);
    c[4] = api.add(g30, tmp);

    // Step 4: Calculate sum
    let mut sum = [api.constant(0); 4];
    for i in 0..4 {
        sum[i] = api.add(p[i], c[i]);
    }

    (sum, c[4])
}

#[inline(always)]
pub fn add<C: Config>(api: &mut API<C>, a: Vec<Variable>, b: Vec<Variable>) -> Vec<Variable> {
    add_brentkung(api, &a, &b)
}


#[inline(always)]
pub fn xor<C: Config>(api: &mut API<C>, a: Vec<Variable>, b: Vec<Variable>) -> Vec<Variable> {
    let nbits = a.len();
    let mut bits_res = vec![api.constant(0); nbits];
    for i in 0..nbits {
        bits_res[i] = api.add(a[i].clone(), b[i].clone());
    }
    bits_res
}

#[inline(always)]
pub fn and<C: Config>(api: &mut API<C>, a: Vec<Variable>, b: Vec<Variable>) -> Vec<Variable> {
    let nbits = a.len();
    let mut bits_res = vec![api.constant(0); nbits];
    for i in 0..nbits {
        bits_res[i] = api.mul(a[i].clone(), b[i].clone());
    }
    bits_res
}

#[inline(always)]
pub fn not<C: Config>(api: &mut API<C>, a: Vec<Variable>) -> Vec<Variable> {
    let mut bits_res = vec![api.constant(0); a.len()];
    for i in 0..a.len() {
        bits_res[i] = api.sub(1, a[i].clone());
    }
    bits_res
}
