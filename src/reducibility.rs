use crate::graph::{Angles, Configuration, EDGES, MAXRING};

pub const SIMATCHNUMBER: [usize; 17] = [
    0, 0, 1, 3, 10, 30, 95, 301, 980, 3228, 10797, 36487, 124542, 428506, 1485003, 5178161,
    18155816,
];

#[derive(Debug, Clone)]
pub struct ReductionStep {
    pub round: usize,
    pub remaining_colorings: usize,
    pub remaining_signed_matchings: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReductionType {
    DReducible,
    CReducible,
    NotReducible,
}

#[derive(Debug, Clone)]
pub struct ReducibilityReport {
    pub total_colorings: usize,
    pub extending_colorings: usize,
    pub initial_failed_colorings: usize,
    pub total_signed_matchings: usize,
    pub steps: Vec<ReductionStep>,
    pub is_d_reducible: bool,
    pub is_c_reducible: bool,
    pub reduction_type: ReductionType,
}

impl ReducibilityReport {
    #[allow(dead_code)]
    pub fn is_reducible(&self) -> bool {
        self.is_d_reducible || self.is_c_reducible
    }
}

pub struct ReducibilityEngine {
    pub power: [i64; MAXRING + 3],
}

impl ReducibilityEngine {
    pub fn new() -> Self {
        let mut power = [0i64; MAXRING + 3];
        power[1] = 1;
        for i in 2..=(MAXRING + 2) {
            power[i] = 3 * power[i - 1];
        }
        Self { power }
    }

    pub fn compute_consistent_live(
        &self,
        conf: &Configuration,
        angles: &Angles,
    ) -> (Vec<u8>, usize) {
        let ring = conf.ring();
        assert!(ring <= MAXRING, "Ring size exceeds MAXRING");

        let ncodes = ((self.power[ring] + 1) / 2) as usize;
        let mut live = vec![1u8; ncodes];
        let nlive_init = self.find_live(&mut live, ncodes, angles, conf.extent_claim());

        let nchar = SIMATCHNUMBER[ring] / 8 + 2;
        let mut real = vec![255u8; nchar + 2];
        let mut nlive = nlive_init;

        loop {
            let _nreal = self.test_match(ring, &mut real, &mut live, nchar);
            let prev_nlive = nlive;
            let (new_nlive, changed) = self.update_live(&mut live, ncodes, prev_nlive);
            nlive = new_nlive;
            if !changed || nlive == 0 {
                break;
            }
        }
        (live, nlive)
    }

    pub fn test_configuration(&self, conf: &Configuration, angles: &Angles) -> ReducibilityReport {
        let ring = conf.ring();
        assert!(ring <= MAXRING, "Ring size exceeds MAXRING");

        let ncodes = ((self.power[ring] + 1) / 2) as usize;
        let mut live = vec![1u8; ncodes];

        // 1. Find live: colorings that do NOT extend to the configuration
        let nlive_init = self.find_live(&mut live, ncodes, angles, conf.extent_claim());
        let extending_count = ncodes - nlive_init;

        let nchar = SIMATCHNUMBER[ring] / 8 + 2;
        let mut real = vec![255u8; nchar + 2];

        let mut steps = Vec::new();
        let mut nlive = nlive_init;
        let mut round = 0;

        loop {
            round += 1;
            let nreal = self.test_match(ring, &mut real, &mut live, nchar);
            let prev_nlive = nlive;
            let (new_nlive, changed) = self.update_live(&mut live, ncodes, prev_nlive);
            nlive = new_nlive;

            steps.push(ReductionStep {
                round,
                remaining_colorings: nlive,
                remaining_signed_matchings: nreal,
            });

            if !changed || nlive == 0 {
                break;
            }
        }

        let is_d_reducible = nlive == 0;
        let mut is_c_reducible = false;

        if !is_d_reducible && angles.contract[0] > 0 && self.check_contract(angles, &live, nlive) {
            is_c_reducible = true;
        }

        let reduction_type = if is_d_reducible {
            ReductionType::DReducible
        } else if is_c_reducible {
            ReductionType::CReducible
        } else {
            ReductionType::NotReducible
        };

        ReducibilityReport {
            total_colorings: ncodes,
            extending_colorings: extending_count,
            initial_failed_colorings: nlive_init,
            total_signed_matchings: SIMATCHNUMBER[ring],
            steps,
            is_d_reducible,
            is_c_reducible,
            reduction_type,
        }
    }

    fn find_live(
        &self,
        live: &mut [u8],
        ncodes: usize,
        angles: &Angles,
        _extent_claim: usize,
    ) -> usize {
        let ring = angles.ring;
        let edges = angles.edges;
        let bigno = (self.power[ring + 1] - 1) / 2;

        let mut c = [0i64; EDGES + 1];
        let mut forbidden = [0i64; EDGES + 1];

        c[edges] = 1;
        let mut j = edges - 1;
        c[j] = 2;
        forbidden[j] = 5;

        let mut extent = 0usize;

        loop {
            while (forbidden[j] & c[j]) != 0 {
                c[j] <<= 1;
                while (c[j] & 8) != 0 {
                    if j >= edges - 1 {
                        return ncodes - extent;
                    }
                    j += 1;
                    c[j] <<= 1;
                }
            }

            if j == ring + 1 {
                self.record(&c, ring, angles, live, &mut extent, bigno);
                c[j] <<= 1;
                while (c[j] & 8) != 0 {
                    if j >= edges - 1 {
                        return ncodes - extent;
                    }
                    j += 1;
                    c[j] <<= 1;
                }
            } else {
                j -= 1;
                let am = &angles.angle[j];
                c[j] = 1;
                let mut u = 0i64;
                for i in 1..=am[0] {
                    u |= c[am[i]];
                }
                forbidden[j] = u;
            }
        }
    }

    fn record(
        &self,
        col: &[i64],
        ring: usize,
        angles: &Angles,
        live: &mut [u8],
        extent: &mut usize,
        bigno: i64,
    ) {
        let mut weight = [0i64; 5];
        for i in 1..=ring {
            let e1 = angles.angle[i][1];
            let e2 = angles.angle[i][2];
            let sum_val = 7 - col[e1] - col[e2];
            if sum_val <= 0 || sum_val >= 5 {
                return;
            }
            let sum = sum_val as usize;
            weight[sum] += self.power[i];
        }

        let mut min = weight[4];
        let mut max = weight[4];
        for i in 1..=2 {
            let w = weight[i];
            if w < min {
                min = w;
            } else if w > max {
                max = w;
            }
        }

        let colno = (bigno - 2 * min - max) as usize;
        if colno < live.len() && live[colno] != 0 {
            *extent += 1;
            live[colno] = 0;
        }
    }

    fn test_match(&self, ring: usize, real: &mut [u8], live: &mut [u8], nchar: usize) -> usize {
        let mut nreal = 0usize;
        let mut bit = 1u8;
        let mut realterm = 0usize;

        let mut matchweight = [[[0i64; 4]; MAXRING + 1]; MAXRING + 1];

        // 1. Matchings not incident with last ring edge
        for a in 2..=ring {
            for b in 1..a {
                matchweight[a][b][0] = 2 * (self.power[a] + self.power[b]);
                matchweight[a][b][1] = 2 * (self.power[a] - self.power[b]);
                matchweight[a][b][2] = self.power[a] + self.power[b];
                matchweight[a][b][3] = self.power[a] - self.power[b];
            }
        }

        for a in 2..ring {
            for b in 1..a {
                let mut n = 0usize;
                let mut interval = [0usize; 10];
                let mut weight = [[0i64; 4]; 9];
                weight[1] = matchweight[a][b];

                if b >= 3 {
                    n = 1;
                    interval[1] = 1;
                    interval[2] = b - 1;
                }
                if a >= b + 3 {
                    n += 1;
                    interval[2 * n - 1] = b + 1;
                    interval[2 * n] = a - 1;
                }

                self.augment(
                    n,
                    &interval,
                    1,
                    &mut weight,
                    &matchweight,
                    live,
                    real,
                    &mut nreal,
                    ring,
                    0,
                    0,
                    &mut bit,
                    &mut realterm,
                    nchar,
                );
            }
        }

        // 2. Matchings using an edge incident with ring
        for a in 2..=ring {
            for b in 1..a {
                matchweight[a][b][0] = self.power[a] + self.power[b];
                matchweight[a][b][1] = self.power[a] - self.power[b];
                matchweight[a][b][2] = -self.power[a] - self.power[b];
                matchweight[a][b][3] = -self.power[a] - 2 * self.power[b];
            }
        }

        let basecol = (self.power[ring + 1] - 1) / 2;
        for b in 1..ring {
            let mut n = 0usize;
            let mut interval = [0usize; 10];
            let mut weight = [[0i64; 4]; 9];
            weight[1] = matchweight[ring][b];

            if b >= 3 {
                n = 1;
                interval[1] = 1;
                interval[2] = b - 1;
            }
            if ring >= b + 3 {
                n += 1;
                interval[2 * n - 1] = b + 1;
                interval[2 * n] = ring - 1;
            }

            self.augment(
                n,
                &interval,
                1,
                &mut weight,
                &matchweight,
                live,
                real,
                &mut nreal,
                ring,
                basecol,
                1,
                &mut bit,
                &mut realterm,
                nchar,
            );
        }

        nreal
    }

    #[allow(clippy::too_many_arguments)]
    fn augment(
        &self,
        n: usize,
        interval: &[usize; 10],
        depth: usize,
        weight: &mut [[i64; 4]; 9],
        matchweight: &[[[i64; 4]; MAXRING + 1]; MAXRING + 1],
        live: &mut [u8],
        real: &mut [u8],
        pnreal: &mut usize,
        ring: usize,
        basecol: i64,
        on: usize,
        pbit: &mut u8,
        prealterm: &mut usize,
        nchar: usize,
    ) {
        self.check_reality(
            depth, weight, live, real, pnreal, ring, basecol, on, pbit, prealterm, nchar,
        );

        let next_depth = depth + 1;
        for r in 1..=n {
            let lower = interval[2 * r - 1];
            let upper = interval[2 * r];

            for i in (lower + 1)..=upper {
                for j in lower..i {
                    weight[next_depth] = matchweight[i][j];
                    let mut newinterval = [0usize; 10];
                    let mut h = 1usize;
                    while h < 2 * r - 1 {
                        newinterval[h] = interval[h];
                        h += 1;
                    }
                    let mut newn = r - 1;
                    if j > lower + 1 {
                        newn += 1;
                        newinterval[h] = lower;
                        h += 1;
                        newinterval[h] = j - 1;
                        h += 1;
                    }
                    if i > j + 1 {
                        newn += 1;
                        newinterval[h] = j + 1;
                        h += 1;
                        newinterval[h] = i - 1;
                        // h += 1;
                    }

                    self.augment(
                        newn,
                        &newinterval,
                        next_depth,
                        weight,
                        matchweight,
                        live,
                        real,
                        pnreal,
                        ring,
                        basecol,
                        on,
                        pbit,
                        prealterm,
                        nchar,
                    );
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn check_reality(
        &self,
        depth: usize,
        weight: &[[i64; 4]; 9],
        live: &mut [u8],
        real: &mut [u8],
        pnreal: &mut usize,
        ring: usize,
        basecol: i64,
        on: usize,
        pbit: &mut u8,
        prealterm: &mut usize,
        nchar: usize,
    ) {
        let nbits = 1usize << (depth - 1);
        let mut choice = [0i64; 9];

        for k in 0..nbits {
            if *pbit == 0 {
                *pbit = 1;
                *prealterm += 1;
                if *prealterm > nchar {
                    panic!("More than {} entries in real needed", nchar + 1);
                }
            }

            if (*pbit & real[*prealterm]) == 0 {
                *pbit = pbit.wrapping_shl(1);
                continue;
            }

            let mut col = basecol;
            let mut parity = (ring & 1) as i64;
            let mut left = k;

            for i in 1..depth {
                if (left & 1) != 0 {
                    parity ^= 1;
                    choice[i] = weight[i][1];
                    col += weight[i][3];
                } else {
                    choice[i] = weight[i][0];
                    col += weight[i][2];
                }
                left >>= 1;
            }

            if parity != 0 {
                choice[depth] = weight[depth][1];
                col += weight[depth][3];
            } else {
                choice[depth] = weight[depth][0];
                col += weight[depth][2];
            }

            if !self.still_real(col, &choice, depth, live, on) {
                real[*prealterm] ^= *pbit;
            } else {
                *pnreal += 1;
            }

            *pbit = pbit.wrapping_shl(1);
        }
    }

    fn still_real(
        &self,
        col: i64,
        choice: &[i64; 9],
        depth: usize,
        live: &mut [u8],
        on: usize,
    ) -> bool {
        let mut sum = [0i64; 128];
        let mut twisted = [0usize; 128];
        let mut untwisted = [0usize; 128];
        let mut ntwisted = 0;
        let mut nuntwisted = 0;

        if col < 0 {
            let idx = (-col) as usize;
            if idx >= live.len() || live[idx] == 0 {
                return false;
            }
            twisted[ntwisted] = idx;
            ntwisted += 1;
            sum[0] = col;
        } else {
            let idx = col as usize;
            if idx >= live.len() || live[idx] == 0 {
                return false;
            }
            untwisted[nuntwisted] = idx;
            nuntwisted += 1;
            sum[0] = col;
        }

        let mut twopower = 1;
        let mut mark = 1;
        for i in 2..=depth {
            let c = choice[i];
            for j in 0..twopower {
                let b = sum[j] - c;
                if b < 0 {
                    let idx = (-b) as usize;
                    if idx >= live.len() || live[idx] == 0 {
                        return false;
                    }
                    twisted[ntwisted] = idx;
                    ntwisted += 1;
                    sum[mark] = b;
                } else {
                    let idx = b as usize;
                    if idx >= live.len() || live[idx] == 0 {
                        return false;
                    }
                    untwisted[nuntwisted] = idx;
                    nuntwisted += 1;
                    sum[mark] = b;
                }
                mark += 1;
            }
            twopower <<= 1;
        }

        // Mark theta bits on live entries
        if on != 0 {
            for i in 0..ntwisted {
                live[twisted[i]] |= 8;
            }
            for i in 0..nuntwisted {
                live[untwisted[i]] |= 4;
            }
        } else {
            for i in 0..ntwisted {
                live[twisted[i]] |= 2;
            }
            for i in 0..nuntwisted {
                live[untwisted[i]] |= 2;
            }
        }

        true
    }

    fn update_live(&self, live: &mut [u8], ncodes: usize, prev_nlive: usize) -> (usize, bool) {
        let mut new_nlive = 0usize;
        if live[0] > 1 {
            live[0] = 15;
        }
        for i in 0..ncodes {
            if live[i] != 15 {
                live[i] = 0;
            } else {
                new_nlive += 1;
                live[i] = 1;
            }
        }

        let changed = (new_nlive < prev_nlive) && (new_nlive > 0);
        (new_nlive, changed)
    }

    pub fn check_contract(&self, angles: &Angles, live: &[u8], _nlive: usize) -> bool {
        let n_contracts = angles.contract[0];
        if n_contracts == 0 {
            return false;
        }

        let ring = angles.ring;
        let edges = angles.edges;
        let bigno = (self.power[ring + 1] - 1) / 2;

        let mut start = edges;
        while start > 0 && angles.contract[start] != 0 {
            start -= 1;
        }
        if start <= 1 {
            return false;
        }

        let mut c = [0i64; EDGES + 1];
        let mut forbidden = [0i64; EDGES + 1];

        c[start] = 1;
        let mut j = start;
        j -= 1;
        while j > 0 && angles.contract[j] != 0 {
            j -= 1;
        }
        if j == 0 {
            return false;
        }

        let dm = &angles.diffangle[j];
        let sm = &angles.sameangle[j];
        c[j] = 1;
        let mut u = 4i64;
        for i in 1..=dm[0] {
            u |= c[dm[i]];
        }
        for i in 1..=sm[0] {
            u |= !c[sm[i]];
        }
        forbidden[j] = u;

        loop {
            while (forbidden[j] & c[j]) != 0 {
                c[j] <<= 1;
                while (c[j] & 8) != 0 {
                    j += 1;
                    while j <= EDGES && angles.contract[j] != 0 {
                        j += 1;
                    }
                    if j >= start {
                        return true;
                    }
                    c[j] <<= 1;
                }
            }

            if j == 1 {
                if self.inlive(&c, ring, live, bigno) {
                    return false;
                }
                c[j] <<= 1;
                while (c[j] & 8) != 0 {
                    j += 1;
                    while j <= EDGES && angles.contract[j] != 0 {
                        j += 1;
                    }
                    if j >= start {
                        return true;
                    }
                    c[j] <<= 1;
                }
                continue;
            }

            j -= 1;
            while j > 0 && angles.contract[j] != 0 {
                j -= 1;
            }
            if j == 0 {
                return false;
            }
            let dm = &angles.diffangle[j];
            let sm = &angles.sameangle[j];
            c[j] = 1;
            let mut u = 0i64;
            for i in 1..=dm[0] {
                u |= c[dm[i]];
            }
            for i in 1..=sm[0] {
                u |= !c[sm[i]];
            }
            forbidden[j] = u;
        }
    }

    fn inlive(&self, col: &[i64], ring: usize, live: &[u8], bigno: i64) -> bool {
        let mut weight = [0i64; 5];
        for i in 1..=ring {
            let color = col[i] as usize;
            if color < 5 {
                weight[color] += self.power[i];
            }
        }
        let mut min = weight[4];
        let mut max = weight[4];
        for i in 1..=2 {
            let w = weight[i];
            if w < min {
                min = w;
            } else if w > max {
                max = w;
            }
        }
        let colno = (bigno - 2 * min - max) as usize;
        colno < live.len() && live[colno] != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{find_angles, validate_sparse_contract, Configuration};

    fn make_birkhoff_diamond() -> Configuration {
        let mut conf = Configuration::new(1, 10, 6, 4);
        conf.set_vertex(1, &[2, 7, 10, 6]);
        conf.set_vertex(2, &[3, 8, 7, 1]);
        conf.set_vertex(3, &[4, 8, 2]);
        conf.set_vertex(4, &[5, 9, 8, 3]);
        conf.set_vertex(5, &[6, 10, 9, 4]);
        conf.set_vertex(6, &[1, 10, 5]);
        conf.set_vertex(7, &[2, 8, 9, 10, 1]);
        conf.set_vertex(8, &[2, 3, 4, 9, 7]);
        conf.set_vertex(9, &[8, 4, 5, 10, 7]);
        conf.set_vertex(10, &[9, 5, 6, 1, 7]);
        conf
    }

    fn make_rsst_2_126() -> Configuration {
        let mut conf = Configuration::new(2126, 12, 8, 81);
        conf.name = "2.126".to_string();
        conf.set_max_cons_subset(154);
        conf.contract_edges = vec![(1, 9), (3, 9), (5, 11), (7, 11)];
        conf.set_vertex(1, &[2, 9, 12, 8]);
        conf.set_vertex(2, &[3, 9, 1]);
        conf.set_vertex(3, &[4, 10, 9, 2]);
        conf.set_vertex(4, &[5, 10, 3]);
        conf.set_vertex(5, &[6, 11, 10, 4]);
        conf.set_vertex(6, &[7, 11, 5]);
        conf.set_vertex(7, &[8, 12, 11, 6]);
        conf.set_vertex(8, &[1, 12, 7]);
        conf.set_vertex(9, &[2, 3, 10, 11, 12, 1]);
        conf.set_vertex(10, &[3, 4, 5, 11, 9]);
        conf.set_vertex(11, &[10, 5, 6, 7, 12, 9]);
        conf.set_vertex(12, &[11, 7, 8, 1, 9]);
        conf
    }

    #[test]
    fn test_birkhoff_diamond_d_reducibility() {
        let conf = make_birkhoff_diamond();
        let angles = find_angles(&conf);
        let engine = ReducibilityEngine::new();
        let report = engine.test_configuration(&conf, &angles);

        assert!(report.is_d_reducible);
        assert_eq!(report.reduction_type, ReductionType::DReducible);
        assert!(report.is_reducible());
        assert_eq!(report.steps.last().unwrap().remaining_colorings, 0);
    }

    #[test]
    fn test_rsst_2_126_c_reducibility() {
        let conf = make_rsst_2_126();
        let angles = find_angles(&conf);
        assert!(validate_sparse_contract(&conf, &angles).is_ok());

        let engine = ReducibilityEngine::new();
        let report = engine.test_configuration(&conf, &angles);

        // Conf 2.126 has 81 extending colorings, exactly 154 colorings in maximal consistent subset
        assert_eq!(report.extending_colorings, 81);
        assert!(!report.is_d_reducible);
        assert_eq!(report.steps.last().unwrap().remaining_colorings, 154);

        // But is C-reducible via the 4-edge contract!
        assert!(report.is_c_reducible);
        assert_eq!(report.reduction_type, ReductionType::CReducible);
        assert!(report.is_reducible());
    }

    #[test]
    fn test_rsst_2_126_invalid_contract_rejected() {
        let mut conf = make_rsst_2_126();
        // Replace contract with a false contract edge that violates Kempe boundary
        conf.contract_edges = vec![(2, 9)];
        let angles = find_angles(&conf);

        let engine = ReducibilityEngine::new();
        let report = engine.test_configuration(&conf, &angles);

        assert!(!report.is_d_reducible);
        assert!(!report.is_c_reducible);
        assert_eq!(report.reduction_type, ReductionType::NotReducible);
    }
}
