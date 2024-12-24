use std::{collections::{HashMap, HashSet}, rc::Rc, str::FromStr};

use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 24)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let (start_values, gates) = puzzle.input_as_str().split_once("\n\n").unwrap();
        let gates = gates
            .lines()
            .map(|l| Rc::new(l.parse::<Gate>().unwrap()))
            .map(|g| (g.output.clone(), g))
            .collect::<HashMap<_, _>>();
        let mut states: HashMap<String, bool> = start_values
            .lines()
            .map(|l| {
                let parts = l.split_once(": ").unwrap();
                (parts.0.to_string(), parts.1 == "1")
            })
            .collect::<HashMap<_, _>>();
        let mut z_states = vec![];
        for gate in gates.values() {
            if gate.output.starts_with("z") {
                let out = resolve_state(&gates, &mut states, gate.output.clone());
                z_states.push((gate.output.clone(), out));
            }
        }
        z_states.sort_unstable_by_key(|(k, _)| k.to_string());
        let number =
            z_states.iter().rev().fold(
                0_usize,
                |acc, (_, v)| {
                    if *v {
                        (acc << 1) | 1
                    } else {
                        acc << 1
                    }
                },
            );
        number.into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let (_, gates) = puzzle.input_as_str().split_once("\n\n").unwrap();
        let gates = gates
            .lines()
            .map(|l| Rc::new(l.parse::<Gate>().unwrap()))
            .map(|g| (g.output.clone(), g))
            .collect::<HashMap<_, _>>();
        let input_count = gates
            .values()
            .filter_map(|g| {
                if g.left.starts_with("x") {
                    Some(g.left.trim_start_matches('x').parse::<usize>().unwrap())
                } else if g.right.starts_with("x") {
                    Some(g.right.trim_start_matches('x').parse::<usize>().unwrap())
                } else {
                    None
                }
            })
            .max()
            .unwrap();
        /*
         * Each set of gates should be modeled like;
         *     X'    Y'    N1 N2
         *   /   \ /   \   |  |
         *   \   / \   /   |  |
         *    AND   XOR     OR      // AND = and1, XOR = xor1, OR = or1
         *     |    |   \ /   |
         *     |    |   / \   |
         *     |    AND     XOR     // AND = and2, XOR = xor2
         *     |    |        |
         *    N1    N2       Z'
         */
        let mut problems = HashSet::new();
        for d in 1..input_count {
            let and_xor_gates = gates
                .values()
                .filter(|g| g.takes_input(&format!("x{:02}", d)) || g.takes_input(&format!("y{:02}", d)))
                .collect::<Vec<_>>();
            if and_xor_gates.len() != 2 {
                // This cannot be... even with the issues, we should be able to swap gates
                panic!("Invalid number of gates: {} for {}", and_xor_gates.len(), d);
            }
            let and1 = and_xor_gates.iter().find(|g| g.operator == Operator::And);
            let xor1 = and_xor_gates.iter().find(|g| g.operator == Operator::Xor);
            let linked_to_z = and_xor_gates.iter().find(|g| g.output.starts_with("z"));
            if let Some(gate) = linked_to_z {
                // x/y directly linked to z
                problems.insert(gate.output.clone());
                // If x/y is directly linked to z, to other side is probably linked to opposite gate
                let chained_to = if gate.operator == Operator::Xor {
                    &and1.expect("and1 gate").output
                } else {
                    &xor1.expect("xor1 gate").output  
                };
                let xor2 = gates.values().find(|g| g.operator == Operator::Xor && g.takes_input(chained_to)).expect("xor2 gate");
                if xor2.output.starts_with("z") {
                    // Expected this to be our other pair of the problem...
                    panic!("Expected this gate not be linked to z: {}", xor2.output);
                } else {
                    problems.insert(xor2.output.clone());
                }
            } else if and1.is_none() || xor1.is_none() {
                // x/y not linked to and1 and xor1, this does not happen in my input
                panic!("Problem found with gate: x{:02}/y{:02}, not linked to and1 and xor1", d, d);
            } else {
                let and1 = and1.unwrap();
                let xor1 = xor1.unwrap();                
                let xor2 = gates.values().find(|g| g.operator == Operator::Xor && g.takes_input(&xor1.output));
                let and2 = gates.values().find(|g| g.operator == Operator::And && g.takes_input(&xor1.output));
                let or1 = gates.values().find(|g| g.operator == Operator::Or && g.takes_input(&and1.output));
                if xor2.is_none() && and2.is_none() && or1.is_none() {
                    // xor1 not linked to xor2 and and2
                    // and1 not linked to or1
                    // they are swapped
                    problems.insert(xor1.output.clone());
                    problems.insert(and1.output.clone());
                } else if xor2.is_none() || and2.is_none() || or1.is_none() {
                    // partial link missing, this doesn't happen in my input
                    panic!("Problem found with gate: x{:02}/y{:02}, partial incorrect link (xor2:{}, and2:{}, or1:{})", d, d, xor2.is_none(), and2.is_none(), or1.is_none());
                } else {
                    // General structure look to be correct, verify expected outputs
                    let xor2 = xor2.unwrap();
                    // let and2 = and2.unwrap();
                    // let or1 = or1.unwrap();
                    if xor2.output != format!("z{:02}", d) {
                        problems.insert(xor2.output.clone());
                        problems.insert(format!("z{:02}", d));
                    }
                }
            }
        }
        let mut problems = problems.into_iter().collect::<Vec<_>>();
        problems.sort_unstable();
        problems.join(",").into()
    }
}

fn resolve_state(
    gates: &HashMap<String, Rc<Gate>>,
    states: &mut HashMap<String, bool>,
    output: String,
) -> bool {
    if let Some(state) = states.get(&output) {
        return *state;
    }
    let gate = gates.get(&output).unwrap();
    let left = resolve_state(gates, states, gate.left.clone());
    let right = resolve_state(gates, states, gate.right.clone());
    let result = match gate.operator {
        Operator::And => left & right,
        Operator::Or => left | right,
        Operator::Xor => left ^ right,
    };
    states.insert(gate.output.clone(), result);
    result
}

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    And,
    Or,
    Xor,
}
impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Operator::And => "AND",
            Operator::Or => "OR",
            Operator::Xor => "XOR",
        };
        write!(f, "{}", s)
    }
}

struct Gate {
    left: String,
    right: String,
    output: String,
    operator: Operator,
}

impl Gate {
    fn takes_input(&self, input: &str) -> bool {
        self.left == input || self.right == input
    }
}

impl FromStr for Gate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        let left = parts[0].to_string();
        let operator = match parts[1] {
            "AND" => Operator::And,
            "OR" => Operator::Or,
            "XOR" => Operator::Xor,
            _ => panic!("Invalid operator"),
        };
        let right = parts[2].to_string();
        let output = parts[4].to_string();
        Ok(Gate {
            left,
            right,
            output,
            operator,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#,
        )
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 2024.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 0.into());
    }
}

/*

flowchart TD
      fqm --> btr[AND btr]
   frm --> btr[AND btr]
   y01 --> pvw[AND pvw]
   x01 --> pvw[AND pvw]
   pcq --> vgc[OR vgc]
   mtb --> vgc[OR vgc]
   y27 --> ckj[XOR ckj]
   x27 --> ckj[XOR ckj]
   qsp --> wjr[AND wjr]
   vgc --> wjr[AND wjr]
   ddm --> z26[XOR z26]
   sgs --> z26[XOR z26]
   pcp --> pmj[OR pmj]
   ssb --> pmj[OR pmj]
   y34 --> mvg[AND mvg]
   x34 --> mvg[AND mvg]
   jqm --> swq[AND swq]
   skv --> swq[AND swq]
   wjw --> brf[OR brf]
   nnm --> brf[OR brf]
   wrc --> z05[XOR z05]
   wqb --> z05[XOR z05]
   x22 --> bqp[AND bqp]
   y22 --> bqp[AND bqp]
   x43 --> dgr[XOR dgr]
   y43 --> dgr[XOR dgr]
   x44 --> tpf[XOR tpf]
   y44 --> tpf[XOR tpf]
   ght --> vdm[OR vdm]
   tgp --> vdm[OR vdm]
   y36 --> bgm[AND bgm]
   x36 --> bgm[AND bgm]
   rmj --> mtt[OR mtt]
   vrs --> mtt[OR mtt]
   x34 --> hnp[XOR hnp]
   y34 --> hnp[XOR hnp]
   x17 --> pnt[AND pnt]
   y17 --> pnt[AND pnt]
   nqj --> z15[XOR z15]
   krv --> z15[XOR z15]
   psm --> htr[AND htr]
   qht --> htr[AND htr]
   y00 --> gwq[AND gwq]
   x00 --> gwq[AND gwq]
   nbp --> vhh[AND vhh]
   vtp --> vhh[AND vhh]
   x31 --> fvh[AND fvh]
   y31 --> fvh[AND fvh]
   fnd --> kbg[OR kbg]
   hrm --> kbg[OR kbg]
   x42 --> tkp[AND tkp]
   y42 --> tkp[AND tkp]
   x22 --> dbp[XOR dbp]
   y22 --> dbp[XOR dbp]
   y33 --> hwf[AND hwf]
   x33 --> hwf[AND hwf]
   pdg --> pgj[AND pgj]
   tfm --> pgj[AND pgj]
   x27 --> knm[AND knm]
   y27 --> knm[AND knm]
   y16 --> fnd[AND fnd]
   x16 --> fnd[AND fnd]
   y03 --> sgq[AND sgq]
   x03 --> sgq[AND sgq]
   kft --> rwg[AND rwg]
   djn --> rwg[AND rwg]
   kfp --> bqw[AND bqw]
   fcw --> bqw[AND bqw]
   x36 --> cmh[XOR cmh]
   y36 --> cmh[XOR cmh]
   y30 --> fhm[AND fhm]
   x30 --> fhm[AND fhm]

   ckj --> z27[XOR z27]
   bch --> z27[XOR z27]
   ckj --> jcp[AND jcp]
   bch --> jcp[AND jcp]

   mwv --> wss[AND wss]
   qpg --> wss[AND wss]
   fvh --> nqk[OR nqk]
   nvg --> nqk[OR nqk]
   cmh --> qwm[AND qwm]
   mtt --> qwm[AND qwm]
   x33 --> wrr[XOR wrr]
   y33 --> wrr[XOR wrr]
   x13 --> cks[XOR cks]
   y13 --> cks[XOR cks]
   nbp --> z12[XOR z12]
   vtp --> z12[XOR z12]
   kbg --> gvb[AND gvb]
   bfj --> gvb[AND gvb]
   tsw --> nnm[AND nnm]
   mqt --> nnm[AND nnm]
   fhm --> qtt[OR qtt]
   wss --> qtt[OR qtt]
   wcd --> knf[AND knf]
   fkv --> knf[AND knf]
   x16 --> wqn[XOR wqn]
   y16 --> wqn[XOR wqn]
   x23 --> tfm[XOR tfm]
   y23 --> tfm[XOR tfm]
   dpj --> rgr[OR rgr]
   hwf --> rgr[OR rgr]
   rts --> ddm[OR ddm]
   krd --> ddm[OR ddm]
   x21 --> msw[XOR msw]
   y21 --> msw[XOR msw]
   rww --> pbm[AND pbm]
   nwt --> pbm[AND pbm]
   x15 --> nqj[XOR nqj]
   y15 --> nqj[XOR nqj]
   rgr --> z34[XOR z34]
   hnp --> z34[XOR z34]
   fkv --> z04[XOR z04]
   wcd --> z04[XOR z04]
   njp --> kjj[OR kjj]
   pgj --> kjj[OR kjj]
   qwm --> skv[OR skv]
   bgm --> skv[OR skv]
   y02 --> wvk[XOR wvk]
   x02 --> wvk[XOR wvk]
   y40 --> bhr[XOR bhr]
   x40 --> bhr[XOR bhr]
   vsh --> vsm[OR vsm]
   vhh --> vsm[OR vsm]
   pmm --> pfk[AND pfk]
   nqk --> pfk[AND pfk]
   pvk --> qdb[AND qdb]
   fwt --> qdb[AND qdb]
   y37 --> jqm[XOR jqm]
   x37 --> jqm[XOR jqm]
   hbs --> tfc[OR tfc]
   bqw --> tfc[OR tfc]
   qgt --> z01[XOR z01]
   gwq --> z01[XOR z01]
   y29 --> ppp[AND ppp]
   x29 --> ppp[AND ppp]
   pdg --> z23[XOR z23]
   tfm --> z23[XOR z23]
   rht --> phk[AND phk]
   fmc --> phk[AND phk]
   y24 --> tgp[AND tgp]
   x24 --> tgp[AND tgp]
   y26 --> cnr[AND cnr]
   x26 --> cnr[AND cnr]
   cks --> dcb[AND dcb]
   vsm --> dcb[AND dcb]
   x17 --> bfj[XOR bfj]
   y17 --> bfj[XOR bfj]
   mvg --> wmt[OR wmt]
   wgt --> wmt[OR wmt]
   qht --> z06[XOR z06]
   psm --> z06[XOR z06]
   qjd --> dwb[AND dwb]
   bqj --> dwb[AND dwb]
   y05 --> pdt[AND pdt]
   x05 --> pdt[AND pdt]
   qgt --> cgt[AND cgt]
   gwq --> cgt[AND cgt]
   x37 --> nhm[AND nhm]
   y37 --> nhm[AND nhm]
   pfk --> brs[OR brs]
   tmp --> brs[OR brs]
   mqt --> z07[XOR z07]
   tsw --> z07[XOR z07]
   x39 --> pcp[AND pcp]
   y39 --> pcp[AND pcp]
   dcb --> bjr[OR bjr]
   hqk --> bjr[OR bjr]
   y24 --> jbg[XOR jbg]
   x24 --> jbg[XOR jbg]
   prd --> z43[XOR z43]
   dgr --> z43[XOR z43]
   vmg --> z19[XOR z19]
   rfk --> z19[XOR z19]
   tfc --> csk[AND csk]
   gpk --> csk[AND csk]
   x41 --> fbn[AND fbn]
   y41 --> fbn[AND fbn]
   cks --> z13[XOR z13]
   vsm --> z13[XOR z13]
   y18 --> fwt[XOR fwt]
   x18 --> fwt[XOR fwt]
   y14 --> wfw[AND wfw]
   x14 --> wfw[AND wfw]
   y04 --> fkv[XOR fkv]
   x04 --> fkv[XOR fkv]
   x31 --> ndt[XOR ndt]
   y31 --> ndt[XOR ndt]
   nvh --> z39[XOR z39]
   svw --> z39[XOR z39]
   nqj --> jrw[AND jrw]
   krv --> jrw[AND jrw]
   y25 --> ghf[XOR ghf]
   x25 --> ghf[XOR ghf]
   smg --> vgn[AND vgn]
   rpv --> vgn[AND vgn]
   vmg --> bng[AND bng]
   rfk --> bng[AND bng]
   psk --> vhg[AND vhg]
   phj --> vhg[AND vhg]
   y28 --> qjd[XOR qjd]
   x28 --> qjd[XOR qjd]
   pbm --> prd[OR prd]
   tkp --> prd[OR prd]
   jqm --> z37[XOR z37]
   skv --> z37[XOR z37]
   ksq --> tht[OR tht]
   vgn --> tht[OR tht]
   brf --> qsb[AND qsb]
   ndc --> qsb[AND qsb]
   x12 --> vtp[XOR vtp]
   y12 --> vtp[XOR vtp]
   rwg --> nvh[OR nvh]
   krr --> nvh[OR nvh]
   wrr --> z33[XOR z33]
   brs --> z33[XOR z33]
   bvf --> hrm[AND hrm]
   wqn --> hrm[AND hrm]
   y29 --> psk[XOR psk]
   x29 --> psk[XOR psk]
   x19 --> vmg[XOR vmg]
   y19 --> vmg[XOR vmg]
   y43 --> twg[AND twg]
   x43 --> twg[AND twg]
   dcs --> phj[OR phj]
   dwb --> phj[OR phj]
   ddm --> ntq[AND ntq]
   sgs --> ntq[AND ntq]
   fqm --> z11[XOR z11]
   frm --> z11[XOR z11]
   y25 --> krd[AND krd]
   x25 --> krd[AND krd]
   x14 --> btp[XOR btp]
   y14 --> btp[XOR btp]
   y32 --> tmp[AND tmp]
   x32 --> tmp[AND tmp]
   wjr --> wcd[OR wcd]
   sgq --> wcd[OR wcd]
   vdn --> kgp[OR kgp]
   twg --> kgp[OR kgp]
   x02 --> pcq[AND pcq]
   y02 --> pcq[AND pcq]
   x41 --> fmc[XOR fmc]
   y41 --> fmc[XOR fmc]
   bjr --> kmh[AND kmh]
   btp --> kmh[AND kmh]
   jbg --> z24[XOR z24]
   kjj --> z24[XOR z24]
   wvk --> mtb[AND mtb]
   mct --> mtb[AND mtb]
   htr --> mqt[OR mqt]
   ggb --> mqt[OR mqt]
   x28 --> dcs[AND dcs]
   y28 --> dcs[AND dcs]
   kjj --> ght[AND ght]
   jbg --> ght[AND ght]
   phk --> nwt[OR nwt]
   fbn --> nwt[OR nwt]
   jdm --> dcm[OR dcm]
   vvp --> dcm[OR dcm]
   wfw --> krv[OR krv]
   kmh --> krv[OR krv]
   x35 --> vrs[AND vrs]
   y35 --> vrs[AND vrs]
   gvb --> pvk[OR pvk]
   pnt --> pvk[OR pvk]
   krp --> rmj[AND rmj]
   wmt --> rmj[AND rmj]
   x40 --> nqs[AND nqs]
   y40 --> nqs[AND nqs]
   scq --> fcw[OR fcw]
   qsb --> fcw[OR fcw]
   rpv --> z20[XOR z20]
   smg --> z20[XOR z20]
   y32 --> pmm[XOR pmm]
   x32 --> pmm[XOR pmm]
   ntq --> bch[OR bch]
   cnr --> bch[OR bch]
   x38 --> krr[AND krr]
   y38 --> krr[AND krr]
   y08 --> ndc[XOR ndc]
   x08 --> ndc[XOR ndc]
   x05 --> wqb[XOR wqb]
   y05 --> wqb[XOR wqb]
   y04 --> wkf[AND wkf]
   x04 --> wkf[AND wkf]
   kgp --> z44[XOR z44]
   tpf --> z44[XOR z44]
   x12 --> vsh[AND vsh]
   y12 --> vsh[AND vsh]
   btp --> z14[XOR z14]
   bjr --> z14[XOR z14]
   fmc --> z41[XOR z41]
   rht --> z41[XOR z41]
   x10 --> csm[AND csm]
   y10 --> csm[AND csm]
   x07 --> wjw[AND wjw]
   y07 --> wjw[AND wjw]
   x42 --> rww[XOR rww]
   y42 --> rww[XOR rww]
   x06 --> ggb[AND ggb]
   y06 --> ggb[AND ggb]
   y19 --> mgk[AND mgk]
   x19 --> mgk[AND mgk]
   prd --> vdn[AND vdn]
   dgr --> vdn[AND vdn]
   djn --> z38[XOR z38]
   kft --> z38[XOR z38]
   qsp --> z03[XOR z03]
   vgc --> z03[XOR z03]
   qtt --> nvg[AND nvg]
   ndt --> nvg[AND nvg]
   wmt --> z35[XOR z35]
   krp --> z35[XOR z35]
   x11 --> frm[XOR frm]
   y11 --> frm[XOR frm]
   y20 --> rpv[XOR rpv]
   x20 --> rpv[XOR rpv]
   x00 --> z00[XOR z00]
   y00 --> z00[XOR z00]
   qjd --> z28[XOR z28]
   bqj --> z28[XOR z28]
   nwt --> z42[XOR z42]
   rww --> z42[XOR z42]
   tht --> vvp[AND vvp]
   msw --> vvp[AND vvp]
   dhq --> rfk[OR rfk]
   qdb --> rfk[OR rfk]
   jcp --> bqj[OR bqj]
   knm --> bqj[OR bqj]
   y21 --> jdm[AND jdm]
   x21 --> jdm[AND jdm]
   rgr --> wgt[AND wgt]
   hnp --> wgt[AND wgt]
   y06 --> qht[XOR qht]
   x06 --> qht[XOR qht]
   pdt --> psm[OR psm]
   qft --> psm[OR psm]
   y20 --> ksq[AND ksq]
   x20 --> ksq[AND ksq]
   ndc --> z08[XOR z08]
   brf --> z08[XOR z08]
   nqs --> rht[OR rht]
   nhb --> rht[OR rht]
   gpk --> z10[XOR z10]
   tfc --> z10[XOR z10]
   ghf --> z25[XOR z25]
   vdm --> z25[XOR z25]
   knf --> wrc[OR wrc]
   wkf --> wrc[OR wrc]
   y01 --> qgt[XOR qgt]
   x01 --> qgt[XOR qgt]
   mtt --> z36[XOR z36]
   cmh --> z36[XOR z36]
   y08 --> scq[AND scq]
   x08 --> scq[AND scq]
   y38 --> kft[XOR kft]
   x38 --> kft[XOR kft]
   x39 --> svw[XOR svw]
   y39 --> svw[XOR svw]
   x07 --> tsw[XOR tsw]
   y07 --> tsw[XOR tsw]
   y03 --> qsp[XOR qsp]
   x03 --> qsp[XOR qsp]
   y26 --> sgs[XOR sgs]
   x26 --> sgs[XOR sgs]
   vdm --> rts[AND rts]
   ghf --> rts[AND rts]
   y13 --> hqk[AND hqk]
   x13 --> hqk[AND hqk]
   y10 --> gpk[XOR gpk]
   x10 --> gpk[XOR gpk]
   mgk --> smg[OR smg]
   bng --> smg[OR smg]
   mwv --> z30[XOR z30]
   qpg --> z30[XOR z30]
   psk --> z29[XOR z29]
   phj --> z29[XOR z29]
   csm --> fqm[OR fqm]
   csk --> fqm[OR fqm]
   kfp --> z09[XOR z09]
   fcw --> z09[XOR z09]
   wqb --> qft[AND qft]
   wrc --> qft[AND qft]
   x15 --> jdf[AND jdf]
   y15 --> jdf[AND jdf]
   pvw --> mct[OR mct]
   cgt --> mct[OR mct]
   pmj --> z40[XOR z40]
   bhr --> z40[XOR z40]
   mct --> z02[XOR z02]
   wvk --> z02[XOR z02]
   x30 --> mwv[XOR mwv]
   y30 --> mwv[XOR mwv]

   pvk --> z18[XOR z18]
   fwt --> z18[XOR z18]
   x18 --> dhq[AND dhq]
   y18 --> dhq[AND dhq]

   dcm --> z22[XOR z22]
   dbp --> z22[XOR z22]
   bqp --> pdg[OR pdg]
   gkg --> pdg[OR pdg]

   x35 --> krp[XOR krp]
   y35 --> krp[XOR krp]
   y23 --> njp[AND njp]
   x23 --> njp[AND njp]
   nqk --> z32[XOR z32]
   pmm --> z32[XOR z32]
   ndt --> z31[XOR z31]
   qtt --> z31[XOR z31]
   bhr --> nhb[AND nhb]
   pmj --> nhb[AND nhb]
   dcm --> gkg[AND gkg]
   dbp --> gkg[AND gkg]
   tpf --> jkv[AND jkv]
   kgp --> jkv[AND jkv]
   svw --> ssb[AND ssb]
   nvh --> ssb[AND ssb]
   bvf --> z16[XOR z16]
   wqn --> z16[XOR z16]
   y11 --> mqk[AND mqk]
   x11 --> mqk[AND mqk]
   nhm --> djn[OR djn]
   swq --> djn[OR djn]
   msw --> z21[XOR z21]
   tht --> z21[XOR z21]
   kbg --> z17[XOR z17]
   bfj --> z17[XOR z17]
   mqk --> nbp[OR nbp]
   btr --> nbp[OR nbp]
   jdf --> bvf[OR bvf]
   jrw --> bvf[OR bvf]
   ppp --> qpg[OR qpg]
   vhg --> qpg[OR qpg]
   brs --> dpj[AND dpj]
   wrr --> dpj[AND dpj]

   x09 --> hbs[AND hbs]
   y09 --> hbs[AND hbs]
   y09 --> kfp[XOR kfp]
   x09 --> kfp[XOR kfp]

   y44 --> mqf[AND mqf]
   x44 --> mqf[AND mqf]
   jkv --> z45[OR z45]
   mqf --> z45[OR z45]



*/
