# S29: SoK: Anonymous Credentials for Digital Identity Wallets

Bormann, C. and Lehmann, A. ePrint 2026/330. 23 pages. Venue to be confirmed from the bibliography.
PDF supplied by the human on 2026-11-04. Not in this repository. Trust: primary. Abstract, general-purpose ZKP sections and conclusion read.

## What it says

- Two architectures: multi-message signatures with efficient proofs (BBS and relatives), and general-purpose ZKPs over legacy ECDSA.
- Schnorr-based presentations are essentially limited to equality and and/or statements over attributes.
- General-purpose ZKPs can in theory express any logic or policy over attributes. The authors say practical systems do not fully achieve this, because they get their efficiency by hard-coding a particular policy, credential format and signature scheme.
- It reports BBS-based systems as the most mature and efficient, and notes high-severity implementation findings in the general-purpose route.

## What we take

The clearest independent statement of the gap the private mode would address: an expressive policy that is not hard-coded. Cite it for the problem statement. It does not show that a policy compiler is efficient enough, and zk-pdp's private mode with signature verification is not yet built or measured.
