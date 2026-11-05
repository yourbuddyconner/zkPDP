# S28: Zero-Knowledge VMs: Security and Usability in Practice

Jiang, J., Chu, J., Xue, P., Peng, X., Qin, H., Luo, X. ASE 2026. DOI 10.1145/3832783.3837500. 13 pages.
PDF supplied by the human on 2026-11-04. Not in this repository. Trust: primary. Findings and the discussion of research gaps read.

## What it says

- An empirical study of 314 audit findings, 11,854 commits and 1,150 usability issues across production zkVMs.
- Underconstrained or soundness errors are the most prevalent category, 71 instances, in every zkVM examined.
- Missing range checks (33 findings) and binding failures persist despite formal verification of constraint soundness. The authors' survey of tools found none that reasons about value ranges or binding completeness.
- More than 15 of the surveyed tools target Circom and R1CS.

## What we take

- Not a performance paper. It has no same-workload numbers, so the same-workload gap stands.
- Supports stating, in the threat model, that a zkVM's soundness is an assumption with a poor track record, and supports the earlier point that testing with honest witnesses does not find these bugs.
