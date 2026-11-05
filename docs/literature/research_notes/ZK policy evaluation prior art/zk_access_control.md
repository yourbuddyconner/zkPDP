# Zero-knowledge proofs for access control and policy compliance: prior art for zk-pdp

Notes current to 2026-11-04. `R<n>` is the relevance rank to zk-pdp. The full ranked index is under the last question. Status marks:

- `read-full`: I read the paper's text.
- `read-abstract`: I read the abstract. Where noted, I also read targeted excerpts of the full text, but not all of it.
- `search-summary-only`: I saw only a search summary, or only a verified metadata record plus another paper's description of it.

Text marked "[background]" comes from my own prior knowledge and was not checked in this run. Treat it as a lead, not a finding. BibTeX fields come from Crossref, arXiv, ePrint or USENIX pages unless the entry's `note` says otherwise.

## ZK attribute-based access control, policy-hiding access control, credentials for authorization, oblivious policy evaluation

### Takeaway

"zk-ABAC" is not an established scheme or term, and ZK-ACE is not ABAC. It proves one fixed statement. The field divides into anonymous credentials (fixed or relation-based predicates over signed attributes) and a small set of papers that attach ZK to a policy language: ZK-TPL, zkGen and Cinderella's templates. I found no paper that applies ZK to Cedar or to Rego.

### Cited Findings

#### ZK attached to a policy language or a policy compiler

- **[R2] zkGen / zkPolicy (Lauinger, Ernstberger, Steinhorst, ICBC 2024)**, `read-full`. The closest prior work to zk-pdp's circuit backend. *Proves:* whatever statement a zkPolicy describes, with gadgets for commitments, encryption, signatures and TLS. *Prover and verifier:* a user against any verifier, including generated Solidity. *Hidden:* arguments marked `s-` are secret, and those marked `p-` are public. *Expressiveness:* JSON relations with comparisons, OR through multiple dashes, AND through multiple constraints, plus `if` and `for`, over strings and integers. *Performance:* it reports transpile time only (1.70 to 2.38 ms) and line counts, with no proving benchmark. *Stated limits:* "the zkPolicy language lacks a formal definition", and verifying generated circuits is future work. *Inconsistency in the paper:* the abstract says 957 to 22 lines, while Table I prints 975 and 22 under headers that look swapped. — [Source](https://tum-esi.github.io/publications-list/PDF/2024-ICBC-zkGen.pdf)
- **[R3] ZK-TPL (More, Ramacher, Alber, Herzl, TrustCom 2022)**, `read-abstract` plus excerpts. The earliest paper I found that extends a general access policy language with ZK. *Proves:* a credential holder satisfies a policy over hidden attributes. *Prover and verifier:* wallet holder to service provider. *Hidden:* attributes used in the new `zkaccept` predicate. Those in `accept` stay revealed. The policy is public. *Expressiveness:* TPL rule language, extended with `zkaccept`. A policy holds only if `accept` and `zkaccept` both hold. *Performance:* the abstract calls communication and verification overhead negligible for regular-sized credentials. It compares against plain TPL verification of 1 to 10 seconds. — [arXiv abstract](https://arxiv.org/abs/2212.02454), [PDF](https://arxiv.org/pdf/2212.02454)
- **[R5] Cinderella (Delignat-Lavaud, Fournet, Kohlweiss, Parno, IEEE S&P 2016)**, `search-summary-only`. A strong precedent for "compile a policy, prove validation, verify cheaply". *Proves:* knowledge of a certificate chain that is valid under a policy. *Prover and verifier:* a TLS client or similar, to a verifier that no longer sees the chain. *Hidden:* certificate contents, with selective disclosure. *Expressiveness:* validation policies written by composing X.509 templates. A template compiler emits C, and the Geppetto compiler turns it into a verifiable computation scheme. Revocation checking is supported. *Performance:* slides give a 288-byte proof. I did not retrieve timings. — [Search summary sources: slides](https://www.ieee-security.org/TC/SP2016/slides/23-3/delignat.pdf), [IEEE](https://ieeexplore.ieee.org/document/7546505)
- **[R18] Kerl, Bodin, Schelén (Cybersecurity 2025)**, `read-abstract`. Despite its title, the abstract says the protocol is ZKP-based, after a review of ABAC, HE and ZKP. *Proves:* private attribute values satisfy ABAC conditions, so the policy need not reveal them. *Setting:* cyber-physical systems such as mines. *Performance:* evaluated, with no figures in the abstract. — [Source](https://doi.org/10.1186/s42400-024-00323-8)
- **[R52] Card requirements language (Camenisch et al., SACMAT 2010)**, `search-summary-only`. I verified only the metadata. [background] A policy language for what anonymous credentials must prove. It predates SNARKs, but is the closest SACMAT paper in spirit. — [Source](https://doi.org/10.1145/1809842.1809863)

#### Policy-hiding designs

- **[R4] zk-Guard-R (Hou et al., Sensors 2026)**, `read-abstract`. The clearest policy-as-witness design. *Proves:* committed attributes satisfy a committed policy, bound to a nonce and a data root. *Hidden:* the policy, through MiMC-Merkle commitments in place of public policy matrices, and requester attributes. *Expressiveness:* the protocol specifies a bounded stack-based policy interpreter inside the circuit. **The measured prototype does not implement it.** It evaluates only the committed policy, committed attribute, range check, data root and nonce mechanisms. *Performance (gnark BN254 Groth16, 128 attributes):* 425,574 R1CS constraints, 3.12 s to prove (a repeat run averaged 2.80 s with 0.54 s standard deviation), 0.73 ms to verify, 641 MB peak heap, 241,942 gas. — [Source](https://doi.org/10.3390/s26165045)
- **[R11] zkAt and zkAt+ (Chalkias et al., AFT 2025)**, `read-abstract`. *Proves:* a transaction is authenticated under a policy. *Hidden:* the authentication policy itself, because the public information is independent of it, using Groth16 compiled to have "equivocable verification keys". zkAt+ also hides policy updates, assuming a trusted authority. *Expressiveness:* claimed "arbitrarily complex" authentication policies, beyond threshold structures, including mixed signature schemes. *Performance:* implemented for one threshold structure, and reported as comparable to threshold signatures. No figures on the abstract page, and the PDF returned HTTP 403. — [Source](https://eprint.iacr.org/2025/921)
- **[R33] Zero-Knowledge Policy-Embedded Access Control (Liu et al., IEEE TCE 2026)**, `search-summary-only`, title and metadata only. — [Source](https://doi.org/10.1109/tce.2026.3713976)
- **Hidden credentials line.** **[R90] Holt et al. 2003**, **[R91] Frikken et al. 2004**, **[R92] Frikken et al. 2006**, all `search-summary-only`. The goal: Alice gets the resource only if she satisfies the policy, Bob learns nothing about her credentials, and she does not learn the policy structure. — [Search summary source](https://www.researchgate.net/publication/3049208_Attribute-Based_Access_Control_with_Hidden_Policies_and_Hidden_Credentials)
- **[R93] OACerts (Li, Li, TDSC 2006)** and **[R94] Policy-hiding access control (Li, Li, PODC 2005)**, `search-summary-only`. Protect both sensitive attribute values and sensitive policies. — [OACerts](https://doi.org/10.1109/tdsc.2006.54), [PODC](https://doi.org/10.1145/1073814.1073819)
- **[R95] Oblivious signature-based envelope (Li, Du, Boneh)**, `search-summary-only`, metadata only. — [Source](https://doi.org/10.1007/s00446-004-0116-1)
- **[R96] Private Policy Negotiation (Kursawe, Neven, Tuyls, FC 2006)** and **[R97] Automated trust negotiation (Winsborough, Seamons, Jones)**, `search-summary-only`, metadata only. [background] Trust negotiation is the older answer to "both sides hold sensitive policies". — [Kursawe](https://doi.org/10.1007/11889663_6), [Winsborough](https://doi.org/10.1109/discex.2000.824965)
- **Oblivious transfer with access control.** **[R98] Camenisch, Dubovitskaya, Neven, CCS 2009** and **[R99] with Zaverucha, PKC 2011**, `search-summary-only`. Users query a database in which each record has a hidden policy. A later paper states the 2011 and 2012 constructions cover only conjunctions of attributes, which is a useful expressiveness data point. — [Follow-up ePrint 2012/348](https://eprint.iacr.org/2012/348), [Guleria and Dutta](https://www.scitepress.org/Papers/2014/50169/)
- **[R100] Hiding the Policy in Cryptographic Access Control (Müller, Katzenbeisser, 2012)**, `search-summary-only`, metadata only. — [Source](https://doi.org/10.1007/978-3-642-29963-6_8)

#### Credentials used for authorization

- **[R12] zk-creds (Rosenberg, White, Garman, Miers, IEEE S&P 2023)**, `read-abstract`. *Proves:* holding a credential from a bulletin board or existing identity document, and that its attributes satisfy composable statements. *Hidden:* which credential is used. *Expressiveness:* general zkSNARK relations, "flexible, composable, and complex identity statements". *Performance:* under 150 ms identity assertions in a passport and age-restricted video test. *Metadata conflict:* ePrint lists DOI 10.1109/SP46215.2023.00108, which Crossref does not resolve (404). Crossref holds 10.1109/sp46215.2023.10179430, pp. 790-808. A CERIAS listing gives other pages, 1882-1900. — [ePrint](https://eprint.iacr.org/2022/878), [Crossref](https://doi.org/10.1109/sp46215.2023.10179430)
- **[R19] Crescent (Paquin, Policharla, Zaverucha, 2024)**, `read-abstract`. *Proves:* possession of an existing JWT or mDL, with selective disclosure and unlinkability, without issuer involvement. *Performance:* after a one-time per-credential setup, proving and verifying take tens of milliseconds. The relevance is the JWT binding, which Prezta found to dominate cost. — [Source](https://eprint.iacr.org/2024/2013)
- **[R20] Anonymous credentials from ECDSA (Frigo, shelat, CIC 2026)**, `read-abstract`. Sumcheck plus Ligero, assuming only SHA-256. *Performance:* about 20 ms for ECDSA proofs, a few hundred ms for a full ISO mDOC presentation on mobile. — [Source](https://eprint.iacr.org/2024/2010). A secondary blog reports 1.17 s prover and 0.68 s verifier on a Pixel 6 for Longfellow, and about 30 s for Crescent; this conflicts with both papers' headline figures, and I did not reconcile it. — [Blog](https://eid-privacy.github.io/wp0/2025/11/28/crescent-longfellow-showdown.html)
- **[R21] zkLogin (Baldimtsi et al., CCS 2024)**, `search-summary-only`, through Prezta's description. Proof generation can be outsourced, and the JWT is bound by putting a hash of the request in the nonce. — [Prezta §6.2](https://arxiv.org/pdf/2607.11466)
- **[R22] Efficient Proofs of Possession for Legacy Signatures (Woo et al., IEEE S&P 2025)**, `search-summary-only`. Prezta names this as the way to bring its RSA cost down. — [Prezta](https://arxiv.org/pdf/2607.11466), [DOI](https://doi.org/10.1109/sp61157.2025.00080)
- **[R24] zk-promises (Shih et al., USENIX Security 2025)**, `read-abstract`. *Proves:* a predicate over stateful client-held credential state, and that recent callbacks were ingested. *Expressiveness:* Turing-complete state machines. *Hidden:* the client's identity. *Performance:* "better-than-state-of-the-art", with no figures in the abstract. Relevant because stateful policy is what Cedar lacks. — [Source](https://eprint.iacr.org/2024/1260)
- **[R53] IRAC (Xie, Song, Cai, 2025)**, `read-abstract` plus excerpts. Issuer-hiding credentials with vector commitments and zk-SNARKs. *Performance:* 375 ms to present and 2 ms to verify for one predicate, 442 ms and 2 ms for another, with the presentation finishing within 1 s. — [Source](https://arxiv.org/pdf/2512.20234)
- **[R54] Issuer-Hiding ABCs (Bobolz et al., CANS 2021)**, `read-abstract`. The verifier defines an ad hoc accepted-issuer set, and the holder proves membership without saying which. Online cost does not depend on the set size. — [Source](https://eprint.iacr.org/2022/213)
- **[R55] ZKlaims (Schanzenbach et al., SECRYPT 2019)**, `search-summary-only`, metadata only. — [Source](https://doi.org/10.5220/0007772900002330)
- **[R56] CanDID (Maram et al., IEEE S&P 2021)** and **[R57] DECO (Zhang et al., CCS 2020)**, `search-summary-only`, metadata only. [background] Both lift data from legacy sources into ZK statements, which bears on input provenance. — [CanDID](https://doi.org/10.1109/sp40001.2021.00038), [DECO](https://doi.org/10.1145/3372297.3417239)
- **[R88] Camenisch, Lysyanskaya (EUROCRYPT 2001)** and **[R89] idemix (Camenisch, Van Herreweghen, CCS 2002)**, `search-summary-only`. Prezta cites CL01 (with Brands) for proving a predicate over credential attributes while remaining anonymous, and describes these custom ZK proofs as relatively expensive to verify and less flexible. I checked idemix only as metadata. — [Prezta §1.1](https://arxiv.org/pdf/2607.11466)

#### ZK-ACE

- **[R32] ZK-ACE (Wang, 2026)**, `read-abstract` plus a model-summarized read of the first 100,000 of 168,939 characters. *Proves:* a secret identity root matches an on-chain commitment and authorized a transaction hash, under five fixed constraints. *Public:* identity commitment, transaction hash, domain, target hash, replay commitment. *Hidden:* identity root, salt, derivation context, nonce. *Expressiveness:* none. The circuit has no configurable policy logic, and wallet policy such as spending limits is out of scope. *Performance:* Circle STARK 14.5 ms prove, 1.1 ms verify, about 107 KB. Groth16 2,155 constraints, 37.3 ms prove, 128 bytes. — [Abstract](https://arxiv.org/abs/2603.07974), [Full text](https://arxiv.org/html/2603.07974v3)

#### BibTeX for this question

```bibtex
@inproceedings{lauinger2024zkgen,
  title = {zkGen: Policy-to-Circuit Transpiler},
  author = {Lauinger, Jan and Ernstberger, Jens and Steinhorst, Sebastian},
  booktitle = {2024 IEEE International Conference on Blockchain and Cryptocurrency (ICBC)},
  pages = {620-624},
  year = {2024},
  doi = {10.1109/icbc59979.2024.10634440},
  url = {https://doi.org/10.1109/icbc59979.2024.10634440}
}
@misc{more2022zktpl,
  title = {Extending Expressive Access Policies with Privacy Features},
  author = {More, Stefan and Ramacher, Sebastian and Alber, Lukas and Herzl, Marco},
  year = {2022},
  eprint = {2212.02454},
  archivePrefix = {arXiv},
  doi = {10.48550/arXiv.2212.02454},
  url = {https://arxiv.org/abs/2212.02454},
  note = {Full version of a paper in the 21st IEEE TrustCom 2022, per the arXiv comment.}
}
@inproceedings{delignatlavaud2016cinderella,
  title = {Cinderella: Turning Shabby X.509 Certificates into Elegant Anonymous Credentials with the Magic of Verifiable Computation},
  author = {Delignat-Lavaud, Antoine and Fournet, Cedric and Kohlweiss, Markulf and Parno, Bryan},
  booktitle = {2016 IEEE Symposium on Security and Privacy (SP)},
  pages = {235-254},
  year = {2016},
  doi = {10.1109/sp.2016.22},
  url = {https://doi.org/10.1109/sp.2016.22}
}
@article{kerl2025privacypre,
  title = {Privacy-preserving attribute-based access control using homomorphic encryption},
  author = {Kerl, Malte and Bodin, Ulf and Schelén, Olov},
  journal = {Cybersecurity},
  volume = {8},
  number = {1},
  year = {2025},
  doi = {10.1186/s42400-024-00323-8},
  url = {https://doi.org/10.1186/s42400-024-00323-8}
}
@inproceedings{camenisch2010a,
  title = {A card requirements language enabling privacy-preserving access control},
  author = {Camenisch, Jan and Mödersheim, Sebastian and Neven, Gregory and Preiss, Franz-Stefan and Sommer, Dieter},
  booktitle = {Proceedings of the 15th ACM symposium on Access control models and technologies},
  pages = {119-128},
  year = {2010},
  doi = {10.1145/1809842.1809863},
  url = {https://doi.org/10.1145/1809842.1809863}
}
@article{hou2026zkguardr,
  title = {zk-Guard-R: Policy-Hidden and Replay-Safe zk-SNARK Access Control for IoT Sensor Data Stored on IPFS},
  author = {Hou, Huiying and Ma, Yucong and Zhao, Zisu and Gan, Xuerui},
  journal = {Sensors},
  volume = {26},
  number = {16},
  pages = {5045},
  year = {2026},
  doi = {10.3390/s26165045},
  url = {https://doi.org/10.3390/s26165045}
}
@misc{chalkias2025zkat,
  title = {Zero-knowledge Authenticator for Blockchain: Policy-private and Obliviously Updateable},
  author = {Chalkias, Kostas Kryptos and Maram, Deepak and Roy, Arnab and Wang, Joy and Yadav, Aayush},
  year = {2025},
  howpublished = {IACR Cryptology ePrint Archive, Paper 2025/921},
  url = {https://eprint.iacr.org/2025/921},
  note = {Listed as appearing at AFT 2025.}
}
@article{liu2026zeroknowle,
  title = {Zero-Knowledge Policy-Embedded Access Control with a Tokenized Secret-Sharing Oracle for Privacy-Preserving Consumer EHRs Sharing},
  author = {Liu, Ningbo and Duan, Jia and Liu, Xi and Ren, Wei},
  journal = {IEEE Transactions on Consumer Electronics},
  pages = {1-1},
  year = {2026},
  doi = {10.1109/tce.2026.3713976},
  url = {https://doi.org/10.1109/tce.2026.3713976}
}
@inproceedings{holt2003hidden,
  title = {Hidden Credentials},
  author = {Holt, Jason E. and Bradshaw, Robert W. and Seamons, Kent E. and Orman, Hilarie},
  booktitle = {Proceedings of the 2003 ACM workshop on Privacy in the electronic society},
  pages = {1-8},
  year = {2003},
  doi = {10.1145/1005140.1005142},
  url = {https://doi.org/10.1145/1005140.1005142}
}
@inproceedings{frikken2004hidden,
  title = {Hidden access control policies with hidden credentials},
  author = {Frikken, Keith and Atallah, Mikhail and Li, Jiangtao},
  booktitle = {Proceedings of the 2004 ACM workshop on Privacy in the electronic society},
  pages = {27-27},
  year = {2004},
  doi = {10.1145/1029179.1029186},
  url = {https://doi.org/10.1145/1029179.1029186}
}
@article{frikken2006attributeb,
  title = {Attribute-Based Access Control with Hidden Policies and Hidden Credentials},
  author = {Frikken, K. and Atallah, M. and Jiangtao Li},
  journal = {IEEE Transactions on Computers},
  volume = {55},
  number = {10},
  pages = {1259-1270},
  year = {2006},
  doi = {10.1109/tc.2006.158},
  url = {https://doi.org/10.1109/tc.2006.158}
}
@article{li2006oacerts,
  title = {OACerts: Oblivious Attribute Certificates},
  author = {Li, Jiangtao and Li, Ninghui},
  journal = {IEEE Transactions on Dependable and Secure Computing},
  volume = {3},
  number = {4},
  pages = {340-352},
  year = {2006},
  doi = {10.1109/tdsc.2006.54},
  url = {https://doi.org/10.1109/tdsc.2006.54}
}
@inproceedings{li2005policyhidi,
  title = {Policy-hiding access control in open environment},
  author = {Li, Jiangtao and Li, Ninghui},
  booktitle = {Proceedings of the twenty-fourth annual ACM symposium on Principles of distributed computing},
  pages = {29-38},
  year = {2005},
  doi = {10.1145/1073814.1073819},
  url = {https://doi.org/10.1145/1073814.1073819}
}
@article{li2005oblivious,
  title = {Oblivious signature-based envelope},
  author = {Li, Ninghui and Du, Wenliang and Boneh, Dan},
  journal = {Distributed Computing},
  volume = {17},
  number = {4},
  pages = {293-302},
  year = {2005},
  doi = {10.1007/s00446-004-0116-1},
  url = {https://doi.org/10.1007/s00446-004-0116-1}
}
@inproceedings{kursawe2006private,
  title = {Private Policy Negotiation},
  author = {Kursawe, Klaus and Neven, Gregory and Tuyls, Pim},
  booktitle = {Lecture Notes in Computer Science},
  pages = {81-95},
  year = {2006},
  doi = {10.1007/11889663_6},
  url = {https://doi.org/10.1007/11889663_6}
}
@inproceedings{winsboroughautomated,
  title = {Automated trust negotiation},
  author = {Winsborough, W.H. and Seamons, K.E. and Jones, V.E.},
  booktitle = {Proceedings DARPA Information Survivability Conference and Exposition. DISCEX'00},
  volume = {1},
  pages = {88-102},
  doi = {10.1109/discex.2000.824965},
  url = {https://doi.org/10.1109/discex.2000.824965},
  note = {Crossref record has no issue year. The container title names DISCEX 2000.}
}
@inproceedings{camenisch2009oblivious,
  title = {Oblivious transfer with access control},
  author = {Camenisch, Jan and Dubovitskaya, Maria and Neven, Gregory},
  booktitle = {Proceedings of the 16th ACM conference on Computer and communications security},
  pages = {131-140},
  year = {2009},
  doi = {10.1145/1653662.1653679},
  url = {https://doi.org/10.1145/1653662.1653679}
}
@inproceedings{camenisch2011oblivious,
  title = {Oblivious Transfer with Hidden Access Control Policies},
  author = {Camenisch, Jan and Dubovitskaya, Maria and Neven, Gregory and Zaverucha, Gregory M.},
  booktitle = {Lecture Notes in Computer Science},
  pages = {192-209},
  year = {2011},
  doi = {10.1007/978-3-642-19379-8_12},
  url = {https://doi.org/10.1007/978-3-642-19379-8_12}
}
@inproceedings{mueller2012hiding,
  title = {Hiding the Policy in Cryptographic Access Control},
  author = {Müller, Sascha and Katzenbeisser, Stefan},
  booktitle = {Lecture Notes in Computer Science},
  pages = {90-105},
  year = {2012},
  doi = {10.1007/978-3-642-29963-6_8},
  url = {https://doi.org/10.1007/978-3-642-29963-6_8}
}
@inproceedings{rosenberg2023zkcreds,
  title = {zk-creds: Flexible Anonymous Credentials from zkSNARKs and Existing Identity Infrastructure},
  author = {Rosenberg, Michael and White, Jacob and Garman, Christina and Miers, Ian},
  booktitle = {2023 IEEE Symposium on Security and Privacy (SP)},
  pages = {790-808},
  year = {2023},
  doi = {10.1109/sp46215.2023.10179430},
  url = {https://doi.org/10.1109/sp46215.2023.10179430},
  note = {IACR ePrint 2022/878, which lists DOI 10.1109/SP46215.2023.00108. Crossref returned 404 for that DOI, so the Crossref record is used here.}
}
@misc{paquin2024crescent,
  title = {Crescent: Stronger Privacy for Existing Credentials},
  author = {Paquin, Christian and Policharla, Guru-Vamsi and Zaverucha, Greg},
  year = {2024},
  howpublished = {Cryptology ePrint Archive, Paper 2024/2013},
  url = {https://eprint.iacr.org/2024/2013}
}
@misc{frigo2024ecdsa,
  title = {Anonymous credentials from {ECDSA}},
  author = {Frigo, Matteo and shelat, abhi},
  year = {2024},
  howpublished = {Cryptology ePrint Archive, Paper 2024/2010},
  url = {https://eprint.iacr.org/2024/2010},
  note = {The ePrint page says this version appears in Communications in Cryptology, Vol. 3, Issue 1.}
}
@inproceedings{baldimtsi2024zklogin,
  title = {zkLogin: Privacy-Preserving Blockchain Authentication with Existing Credentials},
  author = {Baldimtsi, Foteini and Chalkias, Konstantinos Kryptos and Ji, Yan and Lindstrøm, Jonas and Maram, Deepak and Riva, Ben and Roy, Arnab and Sedaghat, Mahdi and Wang, Joy},
  booktitle = {Proceedings of the 2024 on ACM SIGSAC Conference on Computer and Communications Security},
  pages = {3182-3196},
  year = {2024},
  doi = {10.1145/3658644.3690356},
  url = {https://doi.org/10.1145/3658644.3690356}
}
@inproceedings{woo2025efficient,
  title = {Efficient Proofs of Possession for Legacy Signatures},
  author = {Woo, Anna P. Y. and Ozdemir, Alex and Sharp, Chad and Pornin, Thomas and Grubbs, Paul},
  booktitle = {2025 IEEE Symposium on Security and Privacy (SP)},
  pages = {3291-3308},
  year = {2025},
  doi = {10.1109/sp61157.2025.00080},
  url = {https://doi.org/10.1109/sp61157.2025.00080}
}
@inproceedings{shih2025zkpromises,
  title = {zk-promises: Anonymous Moderation, Reputation, and Blocking from Anonymous Credentials with Callbacks},
  author = {Shih, Maurice and Rosenberg, Michael and Kailad, Hari and Miers, Ian},
  booktitle = {34th USENIX Security Symposium (USENIX Security 25)},
  pages = {4995--5014},
  year = {2025},
  month = aug,
  publisher = {USENIX Association},
  isbn = {978-1-939133-52-6},
  url = {https://www.usenix.org/conference/usenixsecurity25/presentation/shih},
  note = {IACR ePrint 2024/1260}
}
@misc{xie2025irac,
  title = {Achieving Flexible and Secure Authentication with Strong Privacy in Decentralized Networks},
  author = {Xie, Bin and Song, Rui and Cai, Xuyuan},
  year = {2025},
  eprint = {2512.20234},
  archivePrefix = {arXiv},
  url = {https://arxiv.org/abs/2512.20234},
  note = {Authors as printed on the PDF title page.}
}
@inproceedings{bobolz2021issuerhidi,
  title = {Issuer-Hiding Attribute-Based Credentials},
  author = {Bobolz, Jan and Eidens, Fabian and Krenn, Stephan and Ramacher, Sebastian and Samelin, Kai},
  booktitle = {Lecture Notes in Computer Science},
  pages = {158-178},
  year = {2021},
  doi = {10.1007/978-3-030-92548-2_9},
  url = {https://doi.org/10.1007/978-3-030-92548-2_9}
}
@inproceedings{schanzenbach2019zklaims,
  title = {ZKlaims: Privacy-preserving Attribute-based Credentials using Non-interactive Zero-knowledge Techniques},
  author = {Schanzenbach, Martin and Kilian, Thomas and Schütte, Julian and Banse, Christian},
  booktitle = {Proceedings of the 16th International Joint Conference on e-Business and Telecommunications},
  pages = {325-332},
  year = {2019},
  doi = {10.5220/0007772900002330},
  url = {https://doi.org/10.5220/0007772900002330}
}
@inproceedings{maram2021candid,
  title = {CanDID: Can-Do Decentralized Identity with Legacy Compatibility, Sybil-Resistance, and Accountability},
  author = {Maram, Deepak and Malvai, Harjasleen and Zhang, Fan and Jean-Louis, Nerla and Frolov, Alexander and Kell, Tyler and Lobban, Tyrone and Moy, Christine and Juels, Ari and Miller, Andrew},
  booktitle = {2021 IEEE Symposium on Security and Privacy (SP)},
  pages = {1348-1366},
  year = {2021},
  doi = {10.1109/sp40001.2021.00038},
  url = {https://doi.org/10.1109/sp40001.2021.00038}
}
@inproceedings{zhang2020deco,
  title = {DECO: Liberating Web Data Using Decentralized Oracles for TLS},
  author = {Zhang, Fan and Maram, Deepak and Malvai, Harjasleen and Goldfeder, Steven and Juels, Ari},
  booktitle = {Proceedings of the 2020 ACM SIGSAC Conference on Computer and Communications Security},
  pages = {1919-1938},
  year = {2020},
  doi = {10.1145/3372297.3417239},
  url = {https://doi.org/10.1145/3372297.3417239}
}
@inproceedings{camenisch2001an,
  title = {An Efficient System for Non-transferable Anonymous Credentials with Optional Anonymity Revocation},
  author = {Camenisch, Jan and Lysyanskaya, Anna},
  booktitle = {Lecture Notes in Computer Science},
  pages = {93-118},
  year = {2001},
  doi = {10.1007/3-540-44987-6_7},
  url = {https://doi.org/10.1007/3-540-44987-6_7}
}
@inproceedings{camenisch2002design,
  title = {Design and implementation of the idemix anonymous credential system},
  author = {Camenisch, Jan and Van Herreweghen, Els},
  booktitle = {Proceedings of the 9th ACM conference on Computer and communications security},
  pages = {21-30},
  year = {2002},
  doi = {10.1145/586110.586114},
  url = {https://doi.org/10.1145/586110.586114}
}
@misc{wang2026zkace,
  title = {{ZK-ACE}: Identity-Centric Zero-Knowledge Authorization for Post-Quantum Blockchain Systems},
  author = {Wang, Jian Sheng},
  year = {2026},
  eprint = {2603.07974},
  archivePrefix = {arXiv},
  doi = {10.48550/arXiv.2603.07974},
  url = {https://arxiv.org/abs/2603.07974}
}
```

### Inferences

- zk-pdp's combination appears unclaimed: a standardized general language, policy as data, and committed facts. ZK-TPL and zkGen each use a policy language, but neither has a verified semantics or a committed attribute store for relationship data.
- zkGen's stated gap, no formal definition and no verification of generated circuits, is what zk-pdp's start from Cedar's verified compiler would address.
- zk-Guard-R's bounded interpreter is a third design point between a zkVM and a compiled circuit: one circuit, policy as a witness. Its unimplemented status means no measurement of that point exists there.
- Policy-private designs and zk-pdp's published policy fingerprint want opposite things. A verifier checking "this known policy was applied" must know the policy identity, while zkAt deliberately makes public information independent of it.

### Gaps

- I found no standard or widely used construction called "zk-ABAC". The closest uses of the phrase are research prototypes and a ResearchGate paper I did not read.
- SACMAT was not fully enumerated. DBLP's API returned an Anubis bot-check page, and I did not try to get around it. A Crossref title search restricted to SACMAT and CODASPY surfaced no paper with "zero-knowledge" in its title. That search returns ranked matches rather than a complete list, so this is weak evidence of absence. Related SACMAT items it did surface: APETEEt (2023), FE[r]Chain (2024), Calypso (2026), and the 2007 and 2010 policy language papers.
- zkAt's numbers, because the PDF returned HTTP 403.

## ZK proofs of policy compliance: middleboxes, network and firewall policy, KYC, Privacy Pools, regulation, audit

### Takeaway

Zero-knowledge middleboxes are the most rigorous compliance line, and they show a recurring cost pattern: verification is cheap and client proving is the bottleneck. Published work on proving firewall or router configuration compliance in ZK is almost absent. KYC and Privacy Pools work proves fixed predicates and set membership, not general policy.

### Cited Findings

#### Network policy

- **[R8] Zero-Knowledge Middleboxes (Grubbs et al., USENIX Security 2022)**, `read-abstract` plus excerpts. *Proves:* the plaintext under TLS 1.3 traffic complies with policy. *Prover and verifier:* client to middlebox, with servers unaware. *Hidden:* traffic contents. The policy is normally public, and the paper covers keeping a blocklist from the client (§7.5). *Expressiveness:* hand-tailored parsing per policy. Case studies are firewalling non-HTTPS traffic and domain blocklists through Merkle non-membership proofs. *Performance (xJsnark, Groth16):* 128-byte proofs, 2 to 5 ms to verify, roughly 14 s for a proof at connection start, 3 to 8 s for later packets, and hundreds of MB of client storage. — [Abstract](https://eprint.iacr.org/2021/1022), [Paper](https://www.usenix.org/system/files/sec22-grubbs.pdf)
- **[R9] Zombie (Zhang et al., NSDI 2024)**, `read-abstract` plus excerpts. *Backend:* Spartan, with no trusted setup. *Technique:* precomputation, asynchrony and batching. *Expressiveness:* designed to extend to "any read-based public policy", with new regex encodings for data loss prevention. *Performance:* under 300 ms critical-path overhead for DNS filtering, or 0 asynchronously, 3.5x total computation speedup, 30 KB proofs, 380 packets of 255 bytes per second, and 1 to 2 ms per byte for regex policies, which the authors call uncomfortably large. It does not stop clients circumventing policy outside the proof path. — [Abstract](https://eprint.iacr.org/2023/1022), [Paper](https://www.usenix.org/system/files/nsdi24-zhang-collin.pdf)
- **[R37] FaaS policy validation with ZKMBs (Andreotti, Kubler, Verticale, 2026 SSRN preprint)**, `read-abstract`. *Prover and verifier:* client to middlebox, with the application provider writing the policy as a validation algorithm. *Finding:* proof generation is the bottleneck, "not yet suitable for real-time blocking". HTTP/3 parsing does not dominate circuit size. Not peer reviewed as far as I could see. — [Source](https://doi.org/10.2139/ssrn.7388917)
- **[R38] Andreotti, Verticale (NetSoft 2024)**, `search-summary-only`, metadata only. — [Source](https://doi.org/10.1109/netsoft60951.2024.10588930)
- **[R30] Reef (Angel et al., USENIX Security 2024)**, `read-abstract`. *Proves:* a committed document does or does not match a regular expression, with PCRE features including lookarounds and negation. *Performance:* 32-million-character documents, with verification under 1 s. Prezta compares its regex work against Reef in its Appendix E.1. — [Source](https://www.usenix.org/conference/usenixsecurity24/presentation/angel)
- **[R31] Coral (Angel et al., IEEE S&P 2026)**, `search-summary-only`, title and metadata only. Context-free grammar proofs. Possibly relevant for parsing policy text inside a proof, which zk-pdp found to dominate VM cost. — [Source](https://doi.org/10.1109/sp63933.2026.00059)
- **[R58] NOPE (DeStefano et al., SOSP 2024)**, `search-summary-only`, metadata only. — [Source](https://doi.org/10.1145/3694715.3695962)
- **[R59] Session-Bound ZK Authorization for SRv6 (Zhou et al., IEEE Access 2026)**, `search-summary-only`, title only. — [Source](https://doi.org/10.1109/access.2026.3708994)
- **[R61] REMOTEGATE (Aggarwal, Zamani, Christodorescu, 2017)**, `read-abstract` plus a search summary. Firewall settings are private. The authors considered a zkSNARK and chose a simpler scheme because of gateway and server burden. A rare direct mention of ZK for firewall configuration, and it declines ZK. — [Source](https://arxiv.org/abs/1709.04569)
- **[R60] SPIDeR, Private and Verifiable Interdomain Routing Decisions (Zhao et al., SIGCOMM CCR 2012)**, `search-summary-only`, metadata only. [background] Verifies routing promises with commitments rather than SNARKs. — [Source](https://doi.org/10.1145/2377677.2377755)
- **[R62] Oblivious and distributed firewall policies (Allami et al., Computers & Security 2025)**, `search-summary-only`, metadata only. It cites ZKMB. — [Source](https://doi.org/10.1016/j.cose.2024.104201)

#### KYC, Privacy Pools, regulated payments

- **[R40] Privacy Pools (Buterin et al., Blockchain: Research and Applications 2024)**, `search-summary-only`. *Proves:* funds do or do not come from a custom association set. *Prover and verifier:* withdrawing user to a smart contract. *Hidden:* the transaction graph. *Expressiveness:* set membership and exclusion. Policy lives in who builds the association set, and critics note this adds centralization risk. — [Record](https://libmast.utm.my/Record/doaj.art-a92b923f2dbb489d86a1fed5f7eccfe8), [Basel summary](https://wwz.unibas.ch/de/newsdetails/blockchain-privacy-and-regulatory-compliance-towards-a-practical-equilibrium/)
- **[R41] zkKYC (Pauwels, 2021)**, `read-abstract`. A solution concept using self-sovereign identity and ZK, where a designated authority can still identify a customer. No performance. — [Source](https://eprint.iacr.org/2021/907)
- **[R42] REGKYC (Xiong, Huth, Knottenbelt, 2025)**, `read-abstract`. A privacy-preserving ABAC model for KYC attributes with authorized deanonymization. No numbers on the abstract page. — [Source](https://eprint.iacr.org/2025/579)
- **[R66] zkMixer (Constantinides, Cartlidge, DAPPS 2025)**, `read-abstract`. Group consensus screens deposits. Its text notes a proof of innocence can be bypassed by using the mixer itself to clean funds first. — [Source](https://arxiv.org/abs/2503.14729)
- **[R67] Private Smart Wallet with Probabilistic Compliance (Rizzini et al., 2025)**, `read-abstract`. Proof of innocence plus Bloom filter tracking. Transfers finish in seconds on a laptop. — [Source](https://arxiv.org/abs/2506.04853)
- **[R68] Auditable cross-chain DeFi (Li et al., 2026)**, `read-abstract`. Proves properties such as non-negative amounts and valid signatures, with threshold view keys for legal audit. — [Source](https://arxiv.org/abs/2608.15276)
- **[R63] Accountable Privacy for Decentralized Anonymous Payments (Garman, Green, Miers, FC 2016 proceedings, LNCS 2017)**, **[R64] Platypus (Wüst et al., CCS 2022)**, **[R65] PEReDi (Kiayias et al., CCS 2022)**, `search-summary-only`, metadata only. [background] These enforce regulatory rules such as limits within private payment proofs. — [Garman](https://doi.org/10.1007/978-3-662-54970-4_5), [Platypus](https://doi.org/10.1145/3548606.3560617), [PEReDi](https://doi.org/10.1145/3548606.3560707)
- **[R70] zkPoC (Wu, Otsuka, DLT 2026)**, `read-abstract`. Proves credit capacity from off-chain reports and on-chain history. A fixed predicate. — [Source](https://doi.org/10.1145/3842755)

#### Regulatory compliance and verifiable audit

- **[R10] AUDIT, Practical Accountability of Secret Processes (Frankle et al., USENIX Security 2018)**, `search-summary-only`. Judges, agencies and companies commit to surveillance actions, and SNARKs show consistency without revealing details. SNARK statements can be tuned to control what is released, and aggregate statistics use MPC. Reported as efficient at federal judiciary scale. The closest prior framing of zk-pdp's "someone audits later" case. — [Source](https://www.dci.mit.edu/posts/practical-accountability-of-secret-processes)
- **[R25] SAMPL (Panwar et al., CCS 2019)**, `search-summary-only`. Its critique: proving knowledge of a court order is not proof of complying with it, and logging was trusted to honest parties. It adds an independent auditor. — [Patent text summarized in search](https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/11323489), [DOI](https://doi.org/10.1145/3319535.3354219)
- **[R43] zkLedger (Narula, Vasquez, Virza, NSDI 2018)**, `read-abstract`. *Prover and verifier:* banks to an auditor. *Hidden:* transaction details. *Expressiveness:* auditor queries such as outstanding balance of an asset. *Performance:* Schnorr-style proofs with no trusted setup, answering over 100,000 transactions in under 10 ms. — [Source](https://www.usenix.org/conference/nsdi18/presentation/narula)
- **[R16] zkSAS (Purbasha et al., DySPAN 2026)**, `read-abstract`. Circuits check whether spectrum channel assignments and incumbent protections follow regulatory rules, without revealing user data. Proving grows with users, and verification is constant. A close analog of a verifiable decider outside authorization. The prover role is my reading, since the abstract does not name it. — [Source](https://arxiv.org/abs/2609.28699)
- **[R47] zk-MCP (Jing, Qi, 2025)**, `read-abstract`. Agents prove statistics of their traffic with Circom proofs, and a third party checks and records them. No numbers in the abstract. — [Source](https://arxiv.org/abs/2512.14737), [Search summary](https://arxiv.org/pdf/2512.14737)
- **[R71] Emission Impossible (Man et al., LOCO 2024)**, `read-abstract`. zk-SNARK checks on emissions claims across a supply chain. No numbers in the abstract. — [Source](https://arxiv.org/abs/2506.16347)
- **[R72] Podda et al. (Internet Policy Review 2025)**, `search-summary-only`. Legal analysis arguing GDPR data minimisation calls for ZK in identity wallets. — [Source](https://policyreview.info/node/2019)
- **[R73] Punniyamoorthy et al. (IJERT 2025)**, `read-abstract`. Lists zero-knowledge compliance proofs as one component. A search summary says they cover access-control decisions against HIPAA and GDPR constraints, but the abstract has no such measurement. Low confidence. — [Source](https://arxiv.org/abs/2512.10341)
- **[R74] Larch (Dauterman et al., OSDI 2023)**, `search-summary-only`. Prezta cites it for encrypting audit log entries, alongside Certificate Transparency for an append-only log. — [Prezta §6.3](https://arxiv.org/pdf/2607.11466)

#### BibTeX for this question

```bibtex
@inproceedings{grubbs2022zkmb,
  title = {Zero-Knowledge Middleboxes},
  author = {Grubbs, Paul and Arun, Arasu and Zhang, Ye and Bonneau, Joseph and Walfish, Michael},
  booktitle = {31st USENIX Security Symposium (USENIX Security 22)},
  pages = {4255--4272},
  year = {2022},
  month = aug,
  publisher = {USENIX Association},
  isbn = {978-1-939133-31-1},
  url = {https://www.usenix.org/conference/usenixsecurity22/presentation/grubbs},
  note = {IACR ePrint 2021/1022}
}
@inproceedings{zhang2024zombie,
  title = {Zombie: Middleboxes that Don't Snoop},
  author = {Zhang, Collin and DeStefano, Zachary and Arun, Arasu and Bonneau, Joseph and Grubbs, Paul and Walfish, Michael},
  booktitle = {21st USENIX Symposium on Networked Systems Design and Implementation (NSDI 24)},
  pages = {1917--1936},
  year = {2024},
  month = apr,
  publisher = {USENIX Association},
  isbn = {978-1-939133-39-7},
  url = {https://www.usenix.org/conference/nsdi24/presentation/zhang-collin},
  note = {IACR ePrint 2023/1022}
}
@misc{andreotti2026functionas,
  title = {Function-as-a-Service Policy Validation with Zero-Knowledge Middleboxes over HTTP/1.1 and HTTP/3},
  author = {Andreotti, Davide and Kubler, Filippo and Verticale, Giacomo},
  year = {2026},
  doi = {10.2139/ssrn.7388917},
  url = {https://doi.org/10.2139/ssrn.7388917}
}
@inproceedings{andreotti2024detection,
  title = {Detection of Anomalous e2e Encrypted Function Invocation in FaaS using Zero-Knowledge Proofs},
  author = {Andreotti, Davide and Verticale, Giacomo},
  booktitle = {2024 IEEE 10th International Conference on Network Softwarization (NetSoft)},
  pages = {175-179},
  year = {2024},
  doi = {10.1109/netsoft60951.2024.10588930},
  url = {https://doi.org/10.1109/netsoft60951.2024.10588930}
}
@inproceedings{angel2024reef,
  title = {Reef: Fast Succinct Non-Interactive Zero-Knowledge Regex Proofs},
  author = {Angel, Sebastian and Ioannidis, Eleftherios and Margolin, Elizabeth and Setty, Srinath and Woods, Jess},
  booktitle = {33rd USENIX Security Symposium (USENIX Security 24)},
  pages = {3801--3818},
  year = {2024},
  month = aug,
  publisher = {USENIX Association},
  url = {https://www.usenix.org/conference/usenixsecurity24/presentation/angel}
}
@inproceedings{angel2026coral,
  title = {Coral: Fast Succinct Non-Interactive Zero-Knowledge CFG Proofs},
  author = {Angel, Sebastian and Celi, Sofía and Margolin, Elizabeth and Mishra, Pratyush and Sander, Martin and Woods, Jess},
  booktitle = {2026 IEEE Symposium on Security and Privacy (SP)},
  pages = {1882-1901},
  year = {2026},
  doi = {10.1109/sp63933.2026.00059},
  url = {https://doi.org/10.1109/sp63933.2026.00059}
}
@inproceedings{destefano2024nope,
  title = {NOPE: Strengthening domain authentication with succinct proofs},
  author = {DeStefano, Zachary and Ma, Jeff J. and Bonneau, Joseph and Walfish, Michael},
  booktitle = {Proceedings of the ACM SIGOPS 30th Symposium on Operating Systems Principles},
  pages = {673-692},
  year = {2024},
  doi = {10.1145/3694715.3695962},
  url = {https://doi.org/10.1145/3694715.3695962}
}
@article{zhou2026sessionbou,
  title = {Session-Bound Zero-Knowledge Authorization for SRv6 Programmable Data Planes},
  author = {Zhou, Jinhong and Lu, Hongzhi and Xiong, Jianxin},
  journal = {IEEE Access},
  volume = {14},
  pages = {104824-104833},
  year = {2026},
  doi = {10.1109/access.2026.3708994},
  url = {https://doi.org/10.1109/access.2026.3708994}
}
@misc{aggarwal2017remotegate,
  title = {{REMOTEGATE}: Incentive-Compatible Remote Configuration of Security Gateways},
  author = {Aggarwal, Abhinav and Zamani, Mahdi and Christodorescu, Mihai},
  year = {2017},
  eprint = {1709.04569},
  archivePrefix = {arXiv},
  doi = {10.48550/arXiv.1709.04569},
  url = {https://arxiv.org/abs/1709.04569}
}
@article{zhao2012private,
  title = {Private and verifiable interdomain routing decisions},
  author = {Zhao, Mingchen and Zhou, Wenchao and Gurney, Alexander J.T. and Haeberlen, Andreas and Sherr, Micah and Loo, Boon Thau},
  journal = {ACM SIGCOMM Computer Communication Review},
  volume = {42},
  number = {4},
  pages = {383-394},
  year = {2012},
  doi = {10.1145/2377677.2377755},
  url = {https://doi.org/10.1145/2377677.2377755}
}
@article{allami2025oblivious,
  title = {Oblivious and distributed firewall policies for securing firewalls from malicious attacks},
  author = {Allami, Ali and Nicewarner, Tyler and Goss, Ken and Kundu, Ashish and Jiang, Wei and Lin, Dan},
  journal = {Computers & Security},
  volume = {150},
  pages = {104201},
  year = {2025},
  doi = {10.1016/j.cose.2024.104201},
  url = {https://doi.org/10.1016/j.cose.2024.104201}
}
@article{buterin2024blockchain,
  title = {Blockchain privacy and regulatory compliance: Towards a practical equilibrium},
  author = {Buterin, Vitalik and Illum, Jacob and Nadler, Matthias and Schär, Fabian and Soleimani, Ameen},
  journal = {Blockchain: Research and Applications},
  volume = {5},
  number = {1},
  pages = {100176},
  year = {2024},
  doi = {10.1016/j.bcra.2023.100176},
  url = {https://doi.org/10.1016/j.bcra.2023.100176},
  note = {Also on SSRN, 10.2139/ssrn.4563364}
}
@misc{pauwels2021zkkyc,
  title = {{zkKYC}: A solution concept for {KYC} without knowing your customer, leveraging self-sovereign identity and zero-knowledge proofs},
  author = {Pauwels, Pieter},
  year = {2021},
  howpublished = {Cryptology ePrint Archive, Paper 2021/907},
  url = {https://eprint.iacr.org/2021/907}
}
@misc{xiong2025regkyc,
  title = {{REGKYC}: Supporting Privacy and Compliance Enforcement for {KYC} in Blockchains},
  author = {Xiong, Xihan and Huth, Michael and Knottenbelt, William},
  year = {2025},
  howpublished = {IACR Cryptology ePrint Archive, Paper 2025/579},
  url = {https://eprint.iacr.org/2025/579}
}
@inproceedings{constantinides2025zkmixer,
  title = {{zkMixer}: A Configurable Zero-Knowledge Mixer with Anti-Money Laundering Consensus Protocols},
  author = {Constantinides, Theodoros and Cartlidge, John},
  booktitle = {DAPPS 2025, 7th IEEE International Conference on Decentralized Applications and Infrastructures},
  pages = {111--120},
  year = {2025},
  doi = {10.1109/DAPPS65174.2025.00022},
  eprint = {2503.14729},
  archivePrefix = {arXiv},
  url = {https://arxiv.org/abs/2503.14729}
}
@misc{rizzini2025wallet,
  title = {A Private Smart Wallet with Probabilistic Compliance},
  author = {Rizzini, Andrea and Esposito, Marco and Bruschi, Francesco and Sciuto, Donatella},
  year = {2025},
  doi = {10.1109/DAPPS65174.2025.00021},
  eprint = {2506.04853},
  archivePrefix = {arXiv},
  url = {https://arxiv.org/abs/2506.04853}
}
@misc{li2026defi,
  title = {Balancing Privacy and Compliance in {DeFi}: A Zero-Knowledge-Based Auditable Cross-Chain Framework},
  author = {Li, Huiheng and Feng, Kainuo and Ding, Jiahao and Ma, Ziqi},
  year = {2026},
  eprint = {2608.15276},
  archivePrefix = {arXiv},
  doi = {10.48550/arXiv.2608.15276},
  url = {https://arxiv.org/abs/2608.15276}
}
@inproceedings{garman2017accountabl,
  title = {Accountable Privacy for Decentralized Anonymous Payments},
  author = {Garman, Christina and Green, Matthew and Miers, Ian},
  booktitle = {Lecture Notes in Computer Science},
  pages = {81-98},
  year = {2017},
  doi = {10.1007/978-3-662-54970-4_5},
  url = {https://doi.org/10.1007/978-3-662-54970-4_5}
}
@inproceedings{wuest2022platypus,
  title = {Platypus: A Central Bank Digital Currency with Unlinkable Transactions and Privacy-Preserving Regulation},
  author = {Wüst, Karl and Kostiainen, Kari and Delius, Noah and Capkun, Srdjan},
  booktitle = {Proceedings of the 2022 ACM SIGSAC Conference on Computer and Communications Security},
  pages = {2947-2960},
  year = {2022},
  doi = {10.1145/3548606.3560617},
  url = {https://doi.org/10.1145/3548606.3560617}
}
@inproceedings{kiayias2022peredi,
  title = {PEReDi: Privacy-Enhanced, Regulated and Distributed Central Bank Digital Currencies},
  author = {Kiayias, Aggelos and Kohlweiss, Markulf and Sarencheh, Amirreza},
  booktitle = {Proceedings of the 2022 ACM SIGSAC Conference on Computer and Communications Security},
  pages = {1739-1752},
  year = {2022},
  doi = {10.1145/3548606.3560707},
  url = {https://doi.org/10.1145/3548606.3560707}
}
@article{wu2026zkpoc,
  title = {zkPoC: A Zero-Knowledge Proof-of-Credit Protocol for Decentralized Lending},
  author = {Wu, Yihsuan and Otsuka, Takumi},
  journal = {Distributed Ledger Technologies: Research and Practice},
  year = {2026},
  doi = {10.1145/3842755},
  url = {https://doi.org/10.1145/3842755}
}
@inproceedings{frankle2018audit,
  title = {Practical Accountability of Secret Processes},
  author = {Frankle, Jonathan and Park, Sunoo and Shaar, Daniel and Goldwasser, Shafi and Weitzner, Daniel J.},
  booktitle = {27th USENIX Security Symposium (USENIX Security 2018)},
  year = {2018},
  url = {https://www.dci.mit.edu/posts/practical-accountability-of-secret-processes},
  note = {Pages not verified in this run.}
}
@inproceedings{panwar2019sampl,
  title = {SAMPL: Scalable Auditability of Monitoring Processes using Public Ledgers},
  author = {Panwar, Gaurav and Vishwanathan, Roopa and Misra, Satyajayant and Bos, Austin},
  booktitle = {Proceedings of the 2019 ACM SIGSAC Conference on Computer and Communications Security},
  pages = {2249-2266},
  year = {2019},
  doi = {10.1145/3319535.3354219},
  url = {https://doi.org/10.1145/3319535.3354219}
}
@inproceedings{narula2018zkledger,
  title = {zkLedger: Privacy-Preserving Auditing for Distributed Ledgers},
  author = {Narula, Neha and Vasquez, Willy and Virza, Madars},
  booktitle = {15th USENIX Symposium on Networked Systems Design and Implementation (NSDI 18)},
  pages = {65--80},
  year = {2018},
  month = apr,
  publisher = {USENIX Association},
  url = {https://www.usenix.org/conference/nsdi18/presentation/narula},
  note = {IACR ePrint 2018/241}
}
@inproceedings{purbasha2026zksas,
  title = {zkSAS: Practical Zero-Knowledge Proofs for Verifiable Spectrum Access Management},
  author = {Purbasha, Nishat F. and Alom, Ifteher and Burger, Eric W. and Hou, Y. Thomas and Lou, Wenjing and Xiao, Yang},
  booktitle = {2026 IEEE International Symposium on Spectrum Innovation (DySPAN)},
  pages = {1-10},
  year = {2026},
  doi = {10.1109/dyspan69846.2026.11571163},
  url = {https://doi.org/10.1109/dyspan69846.2026.11571163}
}
@misc{jing2025zkmcp,
  title = {Zero-Knowledge Audit for Internet of Agents: Privacy-Preserving Communication Verification with Model Context Protocol},
  author = {Jing, Guanlin and Qi, Huayi},
  year = {2025},
  eprint = {2512.14737},
  archivePrefix = {arXiv},
  doi = {10.48550/arXiv.2512.14737},
  url = {https://arxiv.org/abs/2512.14737}
}
@misc{man2025emission,
  title = {Emission Impossible: privacy-preserving carbon emissions claims},
  author = {Man, Jessica and Jaffer, Sadiq and Ferris, Patrick and Kleppmann, Martin and Madhavapeddy, Anil},
  year = {2025},
  eprint = {2506.16347},
  archivePrefix = {arXiv},
  doi = {10.48550/arXiv.2506.16347},
  url = {https://arxiv.org/abs/2506.16347},
  note = {Proceedings of the 1st International Workshop on Low Carbon Computing (LOCO), Glasgow, 3 December 2024}
}
@article{podda2025the,
  title = {The impact of zero-knowledge proofs on data minimisation compliance of digital identity wallets},
  author = {Podda, Emanuela and Hölzmer, Pol and Amard, Alexandre and Sedlmeir, Johannes and Fridgen, Gilbert},
  journal = {Internet Policy Review},
  volume = {14},
  number = {3},
  year = {2025},
  doi = {10.14763/2025.3.2019},
  url = {https://doi.org/10.14763/2025.3.2019}
}
@misc{punniyamoorthy2025cloud,
  title = {A Privacy-Preserving Cloud Architecture for Distributed Machine Learning at Scale},
  author = {Punniyamoorthy, Vinoth and Parthi, Ashok Gadi and Palanigounder, Mayilsamy and Kodali, Ravi Kiran and Kumar, Bikesh and Kannan, Kabilan},
  year = {2025},
  eprint = {2512.10341},
  archivePrefix = {arXiv},
  doi = {10.48550/arXiv.2512.10341},
  url = {https://arxiv.org/abs/2512.10341},
  note = {International Journal of Engineering Research and Technology, 2025; related DOI 10.17577/IJERTV14IS110277}
}
@inproceedings{dauterman2023larch,
  title = {Accountable authentication with privacy protection: The larch system for universal login},
  author = {Dauterman, E. and Lin, D. and Corrigan-Gibbs, H. and Mazi{\`e}res, D.},
  booktitle = {17th USENIX Symposium on Operating Systems Design and Implementation (OSDI 23)},
  pages = {81--98},
  year = {2023},
  note = {Metadata copied from the Prezta bibliography; not checked against the publisher.}
}
```

### Inferences

- zk-pdp's verify numbers (20 ms, 14.7 KB) sit in the same range as these systems, so cost alone won't separate it. Client proving latency is the shared problem.
- ZKMB's per-policy hand-written parsing is the problem a general Cedar compiler removes, at the cost of a fixed language.
- SAMPL's critique applies directly: a proof that "this policy on these committed facts gives this answer" does not show that the committed facts are true, or that the enforcer acted on the answer. Three papers state this separately: SAMPL, the CVA paper's runtime binding, and the agent gateway's source-integrity gap.

### Gaps

- I found no peer-reviewed paper that proves in ZK that a firewall ruleset or router configuration satisfies a compliance rule. Searches returned a think-tank proposal, patents and REMOTEGATE. Treat this as "not found", not "does not exist".
- I found no paper encoding specific GDPR or HIPAA rules as circuits with measured proving costs. Results were vendor posts and system proposals.
- Several SSRN preprints surfaced by title and were not read or included: "Algorithmic Audit Trails for Cloud Banking Compliance" (10.2139/ssrn.6996099), "Cryptographic State-Transition Anchoring" (10.2139/ssrn.6546258), "Blockchain-Based Regulatory Compliance Layer for U.S. Payments" (10.2139/ssrn.5842282), "Cryptographic AI Governance" (10.2139/ssrn.5369425), and "Transaction Binding Security for Policy-Bound Authorization Tokens" (10.2139/ssrn.6486738).

## Blockchain and smart-contract ZK access control, including POLARIS and the ZK-EHR health-data paper

### Takeaway

**POLARIS uses no zero-knowledge proofs.** It relies on salted hash selective disclosure, and its policy decision point is still trusted. The ZK-EHR paper does use Groth16, but its published circuit is a fixed check of three self-asserted values. Most on-chain work is fixed-predicate, with cost measured in gas.

### Cited Findings

- **[R15] POLARIS (Zhang et al., IEEE TrustCom 2025)**, `read-full` (the arXiv text, though figures and Table II did not extract cleanly). *Proves:* nothing in zero knowledge. The disclosure check is that `Hash(Ai‖vi‖si)` matches a signed `hClaims` entry from an accepted issuer. *Prover and verifier:* a wallet builds a verifiable presentation, the resource owner verifies authenticity, and an Authorization Server holding the PDP evaluates the policy and returns the result. *Hidden:* undisclosed claims, by salted hash. The paper concedes "weaker formal unlinkability than zero-knowledge counterparts". *Trust:* the decision is not verifiable by anyone else. The word "verifiable" refers to the credentials and the signed policy, not the decision. *Expressiveness:* VPPL, a JSON language aligned with XACML. Rules hold atomic predicates such as `==`, `!=`, `<=`, `>=` with issuer constraints, combined by algorithms such as permit-overrides. Nesting is deliberately excluded. *Performance (Hyperledger Fabric, dual Xeon 4310):* authorization 88.92 ms and 6.008 KB, authentication 62.188 ms, Register DID 2,466 ms. Reads scale near-linearly to 600 rps with latency under 10 ms, saturating around 800 rps. Writes scale to about 180 rps. — [arXiv](https://arxiv.org/abs/2511.22017), [PDF](https://arxiv.org/pdf/2511.22017)
- **[R14] ZK-EHR (Albalwy, Systems 2026)**, `read-abstract`, **not read in full**. The publisher returned HTTP 403, see Gaps. I read the Crossref abstract and the code in the repository the paper cites. *Claims:* zk-SNARK proofs verify policy compliance without revealing roles, affiliations or credentials. *Prover and verifier:* Patient, Doctor or Researcher to a Solidity verifier. Records are encrypted on IPFS. *Performance:* about 390 ms on-chain verification latency and 216,631 gas per verification. Revocation and replay prevention are "design proposals". — [Abstract record](https://doi.org/10.3390/systems14040385), [Crossref API](https://api.crossref.org/works/10.3390/systems14040385)
  - *What the published circuit does.* `access.circom` takes `role`, `department` and `authorized` as private inputs, and outputs the product of three equality checks against hard-coded constants 1, 2 and 1. No signature, commitment or Merkle check binds these inputs to an issuer. — [Circuit](https://raw.githubusercontent.com/FbalwyTU/ZK-EHR/main/circuits/access.circom), [Input](https://raw.githubusercontent.com/FbalwyTU/ZK-EHR/main/circuits/input.json)
  - The repository's own logs show 85 to 468 ms proving and 373 to 392 ms verification. — [Logs](https://raw.githubusercontent.com/FbalwyTU/ZK-EHR/main/evaluation/scenario1_logs.csv)
- **[R7] Privacy-Preserving On-chain Permissioning (Piper, Wolf, Heiss, 2025)**, `read-abstract` plus excerpts. The best-measured on-chain ABAC. *Proves:* credential attributes satisfy equality, range, membership or time-dependent conditions. *Prover and verifier:* holder to an EVM contract. It maps XACML components (PRP, PDP, PEP, PAP) onto the system. *Hidden:* attributes. *Expressiveness:* those four fixed proof types, up to 16 conditions in the experiments. *Performance (ZoKrates, i7-11800H):* at 16 conditions, about 8.11 M gas for membership, 2.72 M for equality, 3.02 M for range. Moving credential authenticity outside the circuit by commit-and-prove cuts costs 50% for range and equality, and 25% for membership. — [Abstract](https://arxiv.org/abs/2510.05807), [PDF](https://arxiv.org/pdf/2510.05807)
- **[R23] ZKP-CapBAC (Chen et al., INFOCOM 2025)**, `search-summary-only`, title and metadata only. Cross-domain capability delegation with a hidden delegation tree, by its title. Delegation chains are relevant to agents. — [Source](https://doi.org/10.1109/infocom55648.2025.11044772)
- **[R28] Selective Disclosure Authorization Schemes (Khadka, Das, SECRYPT 2026)** and **[R69] its poster**, `read-abstract`. *Proves:* an attribute condition, with the proof bound to the sender to stop mempool front-running and reuse. Sender binding takes a 14-constraint Circom circuit. *Performance:* under 200 ms browser proving, 240,512 gas on Sepolia. The lesson for zk-pdp is request binding. — [Paper](https://arxiv.org/abs/2606.20760), [Poster](https://arxiv.org/abs/2602.18539)
- **[R29] Health Insurance Eligibility (Werapun, Werapun, Cryptography 2026)**, `read-abstract`. *Proves:* an authority-signed record satisfies policy predicates (covered diagnosis, lab range, claim ceiling). *Performance:* 10,050 R1CS constraints, of which eligibility predicates are only 2.1%, and a 256-byte proof. It independently supports the finding that policy logic is small next to credential machinery. It cites ZK-EHR. — [Source](https://doi.org/10.3390/cryptography10050066), [Citation list](https://api.openalex.org/works?filter=cites:W7147397566)
- **[R39] ZK-rollup access control (Lin et al., Sensors 2023)**, `read-abstract`. Batches many authorization requests into one proof, reporting 86% lower authorization time under high traffic. — [Source](https://doi.org/10.3390/s23073443)
- **[R84] Wu, Matsubara, Kasahara (Electronics 2025)**, `read-abstract`. ACL plus capability control with hidden accounts. 7 s proof generation and 10 s authorization. — [Source](https://doi.org/10.3390/electronics14142772)
- **[R85] Maalla, Bezzateev (2024)**, **[R86] Hu et al. (IEEE Network 2023)**, **[R87] SSIBAC (Belchior et al., TrustCom 2020)**, `search-summary-only`, metadata only. ZK-TPL's text describes SSIBAC as using XACML, and POLARIS groups it with work integrating verifiable credentials and ZK into ABAC. — [Maalla](https://doi.org/10.17586/2226-1494-2024-24-5-797-805), [Hu](https://doi.org/10.1109/mnet.126.2200334), [ZK-TPL](https://arxiv.org/pdf/2212.02454), [POLARIS §II](https://arxiv.org/pdf/2511.22017)
- **[R26] ZEXE (Bowe et al., IEEE S&P 2020)** and **[R27] zkay (Steffen et al., CCS 2019)**, `search-summary-only`, metadata only. [background] ZEXE proves that arbitrary birth and death predicates held. zkay compiles privacy-annotated contracts to ZK. Both show general predicates under ZK in a ledger model. — [ZEXE](https://doi.org/10.1109/sp40000.2020.00050), [zkay](https://doi.org/10.1145/3319535.3363222)

#### BibTeX for this question

```bibtex
@inproceedings{zhang2025polaris,
  title = {POLARIS: Cross-Domain Access Control via Verifiable Identity and Policy-Based Authorization},
  author = {Zhang, Aiyao and Lee, Xiaodong and Zhuang, Zhixian and Wei, Jiuqi and Fu, Yufan and Peng, Botao},
  booktitle = {2025 IEEE 24th International Conference on Trust, Security and Privacy in Computing and Communications (TrustCom)},
  pages = {1489-1498},
  year = {2025},
  doi = {10.1109/trustcom66490.2025.00173},
  url = {https://doi.org/10.1109/trustcom66490.2025.00173},
  note = {arXiv:2511.22017}
}
@article{albalwy2026zkehr,
  title = {Zero-Knowledge-Based Policy Enforcement for Privacy-Preserving Cross-Institutional Health Data Sharing on Blockchain},
  author = {Albalwy, Faisal},
  journal = {Systems},
  volume = {14},
  number = {4},
  pages = {385},
  year = {2026},
  doi = {10.3390/systems14040385},
  url = {https://doi.org/10.3390/systems14040385}
}
@misc{piper2025kyc,
  title = {Privacy-Preserving On-chain Permissioning for {KYC}-Compliant Decentralized Applications},
  author = {Piper, Fabian and Wolf, Karl and Heiss, Jonathan},
  year = {2025},
  eprint = {2510.05807},
  archivePrefix = {arXiv},
  doi = {10.48550/arXiv.2510.05807},
  url = {https://arxiv.org/abs/2510.05807}
}
@inproceedings{chen2025zkpcapbac,
  title = {ZKP-CapBAC: Capability-Based Access Control via On-Chain Zero-Knowledge Proofs for Cross-Domain Hiding Delegation Tree},
  author = {Chen, Yanru and Zhang, Yuanyuan and Lin, Xin and Xu, Tongzhou and Pan, Zhiwen and Chen, Liangyin},
  booktitle = {IEEE INFOCOM 2025 - IEEE Conference on Computer Communications},
  pages = {1-10},
  year = {2025},
  doi = {10.1109/infocom55648.2025.11044772},
  url = {https://doi.org/10.1109/infocom55648.2025.11044772}
}
@inproceedings{khadka2026privacypre,
  title = {Privacy-Preserving Compliance on Public Ledgers via Selective Disclosure Authorization Schemes},
  author = {Khadka, Supriya and Das, Sanchari},
  booktitle = {Proceedings of the 23rd International Conference on Security and Cryptography},
  pages = {571-578},
  year = {2026},
  doi = {10.5220/0015159500004103},
  url = {https://doi.org/10.5220/0015159500004103},
  note = {arXiv:2606.20760}
}
@misc{khadka2026poster,
  title = {Poster: Privacy-Preserving Compliance Checks on {Ethereum} via Selective Disclosure},
  author = {Khadka, Supriya and Goswami, Dhiman and Das, Sanchari},
  year = {2026},
  eprint = {2602.18539},
  archivePrefix = {arXiv},
  url = {https://arxiv.org/abs/2602.18539}
}
@article{werapun2026privacypre,
  title = {Privacy-Preserving Health Insurance Eligibility Verification on Blockchain Using zk-SNARKs},
  author = {Werapun, Warit and Werapun, Warodom},
  journal = {Cryptography},
  volume = {10},
  number = {5},
  pages = {66},
  year = {2026},
  doi = {10.3390/cryptography10050066},
  url = {https://doi.org/10.3390/cryptography10050066}
}
@article{lin2023an,
  title = {An Access Control System Based on Blockchain with Zero-Knowledge Rollups in High-Traffic IoT Environments},
  author = {Lin, Xin and Zhang, Yuanyuan and Huang, Changhai and Xing, Bin and Chen, Liangyin and Hu, Dasha and Chen, Yanru},
  journal = {Sensors},
  volume = {23},
  number = {7},
  pages = {3443},
  year = {2023},
  doi = {10.3390/s23073443},
  url = {https://doi.org/10.3390/s23073443}
}
@article{wu2025enhancing,
  title = {Enhancing Account Information Anonymity in Blockchain-Based IoT Access Control Using Zero-Knowledge Proofs},
  author = {Wu, Yuxiao and Matsubara, Yutaka and Kasahara, Shoji},
  journal = {Electronics},
  volume = {14},
  number = {14},
  pages = {2772},
  year = {2025},
  doi = {10.3390/electronics14142772},
  url = {https://doi.org/10.3390/electronics14142772}
}
@article{maalla2024enhancing,
  title = {Enhancing attribute-based access control with Ethereum and ZK-SNARK technologies},
  author = {Maalla, Maher and Bezzateev, S.V.},
  journal = {Scientific and Technical Journal of Information Technologies, Mechanics and Optics},
  volume = {24},
  number = {5},
  pages = {797-805},
  year = {2024},
  doi = {10.17586/2226-1494-2024-24-5-797-805},
  url = {https://doi.org/10.17586/2226-1494-2024-24-5-797-805}
}
@article{hu2023enhancing,
  title = {Enhancing Account Privacy in Blockchain-Based IoT Access Control via Zero Knowledge Proof},
  author = {Hu, Qiwei and Dai, Yueyue and Li, Shuo and Jiang, Tao},
  journal = {IEEE Network},
  volume = {37},
  number = {6},
  pages = {117-123},
  year = {2023},
  doi = {10.1109/mnet.126.2200334},
  url = {https://doi.org/10.1109/mnet.126.2200334}
}
@inproceedings{belchior2020ssibac,
  title = {SSIBAC: Self-Sovereign Identity Based Access Control},
  author = {Belchior, Rafael and Putz, Benedikt and Pernul, Guenther and Correia, Miguel and Vasconcelos, André and Guerreiro, Sérgio},
  booktitle = {2020 IEEE 19th International Conference on Trust, Security and Privacy in Computing and Communications (TrustCom)},
  pages = {1935-1943},
  year = {2020},
  doi = {10.1109/trustcom50675.2020.00264},
  url = {https://doi.org/10.1109/trustcom50675.2020.00264}
}
@inproceedings{bowe2020zexe,
  title = {ZEXE: Enabling Decentralized Private Computation},
  author = {Bowe, Sean and Chiesa, Alessandro and Green, Matthew and Miers, Ian and Mishra, Pratyush and Wu, Howard},
  booktitle = {2020 IEEE Symposium on Security and Privacy (SP)},
  pages = {947-964},
  year = {2020},
  doi = {10.1109/sp40000.2020.00050},
  url = {https://doi.org/10.1109/sp40000.2020.00050}
}
@inproceedings{steffen2019zkay,
  title = {zkay: Specifying and Enforcing Data Privacy in Smart Contracts},
  author = {Steffen, Samuel and Bichsel, Benjamin and Gersbach, Mario and Melchior, Noa and Tsankov, Petar and Vechev, Martin},
  booktitle = {Proceedings of the 2019 ACM SIGSAC Conference on Computer and Communications Security},
  pages = {1759-1776},
  year = {2019},
  doi = {10.1145/3319535.3363222},
  url = {https://doi.org/10.1145/3319535.3363222}
}
```

### Inferences

- **`paper/related-work.md` lists POLARIS under "Zero-knowledge proofs for access control", which is a misclassification.** It belongs with verifiable-credential systems. The paper positions itself as a lighter alternative to ZKP-heavy approaches.
- **ZK-EHR's circuit, as published, does not establish that a requester holds a role.** Any prover can pick `role=1, department=2, authorized=1` and produce a valid proof. Its equality template `1-(a-b)^2` also does not constrain its output to be boolean, and because the output is a product, mismatched inputs can reach 1 for suitable field values. This is my reading of the code and I did not run an exploit. The paper text may describe more than the repository contains, and I could not read it. The project should not cite ZK-EHR as evidence that credential-bound ZK policy enforcement works at those costs.
- ZK-EHR's 216,631 gas and zk-Guard-R's 241,942 gas both come from Groth16 verifiers, so those figures reflect the proof system and say little about policy size.
- Commit-and-prove (Piper) and Werapun's 2.1% figure both point the same way as Prezta and zk-pdp's H3: credential and input handling cost more than the decision.

### Gaps

- **The ZK-EHR full text is unread, which falls short of the brief's request to read it.** MDPI returned HTTP 403 to the fetch tool. I then tried a request with a browser user-agent, which the session's permission classifier blocked as bot-check circumvention, and I did not retry. Someone with browser access should read the PDF and check whether the circuit there matches the repository.
- I did not get ZKP-CapBAC's abstract, and Crossref holds none.

## Verifiable authorization in zero-trust, OT and IoT edge settings, and the Prezta citation graph

### Takeaway

Prezta is the only paper I found that evaluates a standard policy language inside a zkVM for a zero-trust decision, and it is the closest prior work by a wide margin. No indexed paper cites it yet. The rest of the IoT literature is fixed-predicate, and agent-authorization papers are early, with little measurement.

### Cited Findings

#### Prezta

- **[R1] Prezta (Wei, Ameer, Zhang, Borisov, USENIX Security 2026, pp. 5811-5830)**, `read-full` of the arXiv v2 body. I did not read the appendices closely, including the Reef comparison. — [USENIX](https://www.usenix.org/conference/usenixsecurity26/presentation/wei-zhongjing), [arXiv PDF](https://arxiv.org/pdf/2607.11466)
  - *Proves:* the relation `P(att, Req, info)=1 ∧ signature.Verify(att, σ_att, pk_IdP)=1`, knowledge soundness of the SNARK plus unforgeability of signatures.
  - *Prover and verifier:* the untrusted subject proves, and the edge device's PEP verifies. Proving may be outsourced without trusting the prover service, apart from care with bearer JWTs.
  - *Hidden:* the paper uses soundness and succinctness and "not zero-knowledge". The JWT is a private input. Public inputs (subject IP, resource ID, timestamp, JWT public key) must be checked by the device against the network stack, request context and system clock. Unlinkability is left as future work.
  - *Expressiveness:* XACML 3.0 compiled to Rust to RISC-V. All 323 evaluated policies pass, out of 389 testable and 397 in the suite. "83%" means 323/389. The other 66 use unsupported features such as XPath, RFC 822 names or duration arithmetic, and 8 more were excluded earlier for missing request files or for policy references. All standard combining algorithms are supported. Output is binary: NotApplicable, Deny and Indeterminate all map to false.
  - *Policy binding:* the policy is the image. The authority signs the IMAGE_ID, the device stores it, and each new policy means a new image.
  - *Performance (AWS c7i.4xlarge, 16 vCPU, 32 GB, RISC Zero v3.0.3):* about 14 s or 28 s to prove, stepping with power-of-two padding at 2^17 and 2^18 cycles. 238 to 250 KB proofs, compressible to 256 bytes through Groth16 at extra prover cost. 14 to 15.5 ms to verify. An empty program takes 3.8 s. Removing RSA gives about 7 s for 88.6% of policies.
  - *Where the time goes:* the heaviest policy went from 1,301.7 s naive to 27.3 s (47.7x), with 10,521,257 user cycles before optimization. RSA was 45% and regex 47% of the final user cycles. Most policies use under 20K user cycles, less than the zkVM's own padding.
  - *Also discussed:* policy updates with expiry and an OCSP-style status input, proof composition for modular policies (future work), audit through log inclusion proofs, a post-quantum route, and Rego or NGAC through new compilers.
  - *Artifact:* [Zenodo 10.5281/zenodo.20303462](https://doi.org/10.5281/zenodo.20303462), [ZK_Zero_Trust](https://github.com/walotta/ZK_Zero_Trust), [xacml-to-rust](https://github.com/osaidameer/xacml-to-rust), marked unaudited research code.

#### Backward citations from Prezta, relevant entries

Taken from Prezta's related work and bibliography. Each is `search-summary-only` unless it has an entry elsewhere in these notes.

- **[R17] Privacy-Preserving eIDAS Compliance via zkVM (Moser et al., PerCom Workshops 2025).** zkVM verifies digital signatures to authorize smart contract operations. Prezta says systems like this take a minute or longer to prove. — [Prezta §1.1](https://arxiv.org/pdf/2607.11466), [DOI](https://doi.org/10.1109/percomworkshops65533.2025.00037)
- **[R75] Zero-Knowledge Address Abstraction (Park et al., SAC 2025).** SNARK-bootstrapped credentials for blockchain. — [Prezta §1.1](https://arxiv.org/pdf/2607.11466), [DOI](https://doi.org/10.1145/3672608.3707839)
- **[R76] Caplets (Bakir et al., SEC 2021)**, **[R77] Active certificates (Borisov, Brewer, NDSS 2002)**, **[R78] Stateful least privilege (Cao et al., USENIX Security 2024).** Access restrictions through a classic, non-verifiable VM, focused on least privilege in delegation. Prezta distinguishes them because they must run at the endpoint or a trusted server. — [Prezta §1.1](https://arxiv.org/pdf/2607.11466)
- **[R79] Collaborative zk-SNARKs (Ozdemir, Boneh, USENIX Security 2022)** and **[R80] zkSaaS (Garg et al., USENIX Security 2023).** Named as routes to full access privacy when proving is outsourced to a committee. — [Prezta §6.2](https://arxiv.org/pdf/2607.11466)
- Also cited: Reef, zkLogin, zk-creds, CL01, Woo et al., Larch. Non-academic: Aptos Keyless, Bonsai Pay, Ethproofs, AuthzForce conformance tests. xJsnark and Poseidon are proof-system work, outside this note.

#### Forward citations

- OpenAlex lists 0 citing works for Prezta. It also lists 0 for ZK-ACE, 4 for POLARIS, 4 for ZKP-CapBAC, 5 for ZK-EHR, 1 for zkGen and 15 for ZKMB. — [Prezta record](https://api.openalex.org/works/doi:10.48550/arXiv.2607.11466), [POLARIS record](https://api.openalex.org/works/doi:10.1109/trustcom66490.2025.00173), [ZKMB citing search](https://api.openalex.org/works?filter=cites:W3202998329)
- Three of POLARIS's four OpenAlex citers are vector search papers, probably from a shared author, so these counts are noisy. The fourth is a zero-trust multi-cloud literature review (10.7250/csimq.2026-48.05). — [POLARIS record](https://api.openalex.org/works/doi:10.1109/trustcom66490.2025.00173)
- Citing leads seen only as titles, not read, no BibTeX: ZKP-CapBAC is cited by "Proof-at-the-Edge: zk-SNARKs for Privacy-Preserving Wearable IoT Health Monitoring" (10.1109/iemcon67450.2025.11381186) and "Operator-Blind Secret Mediation for AI Agents" (10.3390/math14132434). zkGen is cited by CoBBL (10.1109/sp61157.2025.00200). ZKMB is cited by HydraProofs (10.1109/sp61157.2025.00204), FreeAuth (10.1109/acsac63791.2024.00040), "Beyond the Circuit" (10.62056/an-4c3c2h), "Anonymous Domain Ownership" (10.1109/icbc56567.2023.10174991) and "SoK: Content Moderation for End-to-End Encryption" (10.56553/popets-2023-0060).

#### Agents under policy

- **[R6] Cryptographically Verifiable Agent Authorization (Llambí-Morillas, Fernández-Fernández, Frontiers in CS 2026)**, `read-abstract` plus excerpts. *Proves:* a request by an agent principal in a context satisfies a policy, with the policy named by a versioned hash `pid`. *Hidden:* authorization attributes. *Expressiveness:* the proof-of-concept encodes only the policy logic fixed at circuit generation, and the authors leave representable policy classes as an open question. *Performance (Groth16, Circom, WASM):* 531 constraints, 351.9 ms proving, 309.4 ms verifying, 805 bytes, with Node startup included in the timings. *Key claim:* identity binding, request binding and runtime execution binding are separate. A proof does not show the request was executed. — [arXiv](https://arxiv.org/abs/2607.21325), [PDF](https://arxiv.org/pdf/2607.21325), [Journal](https://doi.org/10.3389/fcomp.2026.1966725)
- **[R13] ZK Predicate Proofs Between AI Agents (Gopalakrishna, 2026)**, `read-abstract`. *Proves:* a governance-defined threshold predicate over private data, through a proof slot on MCP and Agent2Agent. *Performance:* 6.2 ms prove, 1.0 ms verify, 608-byte Bulletproofs for a 32-bit threshold. *Key claim:* a predicate proof binds to a committed value and not to the system of record, so it adds enclave attestation, tested only against a mock authority. — [Source](https://arxiv.org/abs/2608.30083)
- **[R48] Aegis Protocol (Adapala, Alugubelly, 2025)**, `read-abstract`. Halo2 policy compliance proofs in a simulation, 2.79 s median proving. Not a measured system. — [Source](https://arxiv.org/abs/2508.19267)

#### IoT and OT

- **[R34] Bilateral Access Control with ZK Pre-verification for IIoT (Chen et al., IEEE IoT Journal 2026)**, **[R35] Anonymous Command Authorization for IoT (Mahdi et al., DCOSS-IoT 2026)**, **[R36] zk-IoT (Ramezan, Meamari, ICBC 2024)**, **[R81] Cross-Domain AC for Industrial Internet (Lu et al., 2024)**, **[R82] G-TOK (Zhong et al., BCCA 2024)**, **[R83] Lightweight ZKP Multi Access Control (Lalouani, Emokpae, 2024)**, `search-summary-only`, title and metadata only. Worth reading for edge verifier measurements. — [Chen](https://doi.org/10.1109/jiot.2026.3718035), [Mahdi](https://doi.org/10.1109/dcoss-iot69657.2026.00013), [Ramezan](https://doi.org/10.1109/icbc59979.2024.10634342), [Lu](https://doi.org/10.1109/aiim64537.2024.10934570), [Zhong](https://doi.org/10.1109/bcca62388.2024.10844465), [Lalouani](https://doi.org/10.1109/smartnets61466.2024.10577736)
- **[R120] Madtls (Wagner et al., AsiaCCS 2024)**, `search-summary-only`, metadata only. An industrial middlebox design surfaced by the ZKMB searches. OpenAlex lists it as citing ZKMB. — [Source](https://doi.org/10.1145/3634737.3637640)
- **[R121] Calypso (Niroula et al., SACMAT 2026)**, `search-summary-only`, title only. Zero-trust access control at the venue, and no ZK in the title. — [Source](https://doi.org/10.1145/3750555.3811897)
- **[R122] Regulatory-Gated Zero-Trust Access Control (Qi, Chen, Electronics 2026)**, `read-abstract`. A signed PERMIT plus attestations and a permissioned ledger, with no ZK named in the abstract. About 47 tx/s at 50 offered. A non-ZK comparator. — [Source](https://doi.org/10.3390/electronics15204743)

#### BibTeX for this question

```bibtex
@inproceedings{wei2026prezta,
  title = {Prezta: Provable Remote Execution of {Zero-Trust} Authorization using {SNARKs}},
  author = {Wei, Zhongjing and Ameer, Osaid Muhammad and Zhang, Yupeng and Borisov, Nikita},
  booktitle = {35th USENIX Security Symposium (USENIX Security 26)},
  pages = {5811--5830},
  year = {2026},
  month = aug,
  publisher = {USENIX Association},
  isbn = {978-1-939133-58-8},
  eprint = {2607.11466},
  archivePrefix = {arXiv},
  url = {https://www.usenix.org/conference/usenixsecurity26/presentation/wei-zhongjing},
  note = {Author order as on the USENIX page. arXiv lists Wei, Ameer, Borisov, Zhang.}
}
@inproceedings{moser2025privacypre,
  title = {Privacy-Preserving eIDAS Compliance in Blockchain Wallets via zkVM},
  author = {Moser, Paolo and Esposito, Marco and Bruschi, Francesco and Sciuto, Donatella},
  booktitle = {2025 IEEE International Conference on Pervasive Computing and Communications Workshops and other Affiliated Events (PerCom Workshops)},
  pages = {31-37},
  year = {2025},
  doi = {10.1109/percomworkshops65533.2025.00037},
  url = {https://doi.org/10.1109/percomworkshops65533.2025.00037}
}
@inproceedings{park2025beyond,
  title = {Beyond the Blockchain Address: Zero-Knowledge Address Abstraction},
  author = {Park, Sanghyeon and Lee, Jeonghyuk and Lee, Seunghwa and Chun, Jung Hyun and Cho, Hyeonmyeong and Kim, Mingi and Cho, Hyun Ki and Moon, Soo-Mook},
  booktitle = {Proceedings of the 40th ACM/SIGAPP Symposium on Applied Computing},
  pages = {366-374},
  year = {2025},
  doi = {10.1145/3672608.3707839},
  url = {https://doi.org/10.1145/3672608.3707839}
}
@inproceedings{bakir2021caplets,
  title = {Caplets: Resource aware, capability-based access control for IoT},
  author = {Bakir, F. and Krintz, C. and Wolski, R.},
  booktitle = {2021 IEEE/ACM Symposium on Edge Computing (SEC)},
  pages = {106--120},
  year = {2021},
  note = {Metadata copied from the Prezta bibliography; not checked against the publisher.}
}
@inproceedings{borisov2002active,
  title = {Active certificates: A framework for delegation},
  author = {Borisov, N. and Brewer, E.},
  booktitle = {Network and Distributed Systems Security Symposium},
  year = {2002},
  note = {Metadata copied from the Prezta bibliography; not checked against the publisher.}
}
@inproceedings{cao2024stateful,
  title = {Stateful least privilege authorization for the cloud},
  author = {Cao, L. and Meng, L. and Stefan, D. and Fernandes, E.},
  booktitle = {33rd USENIX Security Symposium (USENIX Security 24)},
  pages = {3477--3494},
  year = {2024},
  note = {Metadata copied from the Prezta bibliography; not checked against the publisher.}
}
@inproceedings{ozdemir2022collab,
  title = {Experimenting with collaborative zk-SNARKs: Zero-Knowledge proofs for distributed secrets},
  author = {Ozdemir, A. and Boneh, D.},
  booktitle = {31st USENIX Security Symposium (USENIX Security 22)},
  pages = {4291--4308},
  year = {2022},
  note = {Metadata copied from the Prezta bibliography; not checked against the publisher.}
}
@inproceedings{garg2023zksaas,
  title = {zkSaaS: Zero-Knowledge SNARKs as a service},
  author = {Garg, S. and Goel, A. and Jain, A. and Policharla, G.-V. and Sekar, S.},
  booktitle = {32nd USENIX Security Symposium (USENIX Security 23)},
  pages = {4427--4444},
  year = {2023},
  note = {Metadata copied from the Prezta bibliography; not checked against the publisher.}
}
@article{llambmorillas2026cryptograp,
  title = {Cryptographically verifiable authorization for autonomous AI agents: a falsifiable hypothesis and proof of concept},
  author = {Llambí-Morillas, Mar and Fernández-Fernández, Daniel},
  journal = {Frontiers in Computer Science},
  volume = {8},
  year = {2026},
  doi = {10.3389/fcomp.2026.1966725},
  url = {https://doi.org/10.3389/fcomp.2026.1966725},
  note = {arXiv:2607.21325}
}
@misc{gopalakrishna2026predicate,
  title = {Zero-Knowledge Predicate Proofs Between {AI} Agents: A Measured, Cross-Protocol Gateway and the Source-Integrity Gap},
  author = {Gopalakrishna, Ashok Subbabhatta},
  year = {2026},
  eprint = {2608.30083},
  archivePrefix = {arXiv},
  doi = {10.48550/arXiv.2608.30083},
  url = {https://arxiv.org/abs/2608.30083}
}
@misc{adapala2025aegis,
  title = {The Aegis Protocol: A Foundational Security Framework for Autonomous {AI} Agents},
  author = {Adapala, Sai Teja Reddy and Alugubelly, Yashwanth Reddy},
  year = {2025},
  eprint = {2508.19267},
  archivePrefix = {arXiv},
  doi = {10.48550/arXiv.2508.19267},
  url = {https://arxiv.org/abs/2508.19267}
}
@article{chen2026privacypre,
  title = {Privacy-Preserving Attribute-Based Bilateral Access Control With Zero-Knowledge Pre-verification for IIoT},
  author = {Chen, Wenqi and Fu, Yusun and Wang, Zihao and Lin, Xin},
  journal = {IEEE Internet of Things Journal},
  volume = {13},
  number = {19},
  pages = {45151-45169},
  year = {2026},
  doi = {10.1109/jiot.2026.3718035},
  url = {https://doi.org/10.1109/jiot.2026.3718035}
}
@inproceedings{mahdi2026anonymous,
  title = {Anonymous Command Authorization for IoT Devices Using Zero-Knowledge Proofs},
  author = {Mahdi, Mohammad and Alqattan, Ahmad and Mohammed, Ameer},
  booktitle = {2026 22nd International Conference on Distributed Computing in Smart Systems and the Internet of Things (DCOSS-IoT)},
  pages = {46-55},
  year = {2026},
  doi = {10.1109/dcoss-iot69657.2026.00013},
  url = {https://doi.org/10.1109/dcoss-iot69657.2026.00013}
}
@inproceedings{ramezan2024zkiot,
  title = {zk-IoT: Securing the Internet of Things with Zero-Knowledge Proofs on Blockchain Platforms},
  author = {Ramezan, Gholamreza and Meamari, Ehsan},
  booktitle = {2024 IEEE International Conference on Blockchain and Cryptocurrency (ICBC)},
  pages = {1-7},
  year = {2024},
  doi = {10.1109/icbc59979.2024.10634342},
  url = {https://doi.org/10.1109/icbc59979.2024.10634342}
}
@inproceedings{lu2024secure,
  title = {Secure and Efficient Cross-Domain Access Control for Industrial Internet Alliance Chains Based on Zero-Knowledge Proof},
  author = {Lu, Chunyu and Luo, Jun and Shang, Duo and Li, Haoran and Chen, Tianran and Yu, Kang and Qian, Feng and Hui, Xin},
  booktitle = {2024 4th International Symposium on Artificial Intelligence and Intelligent Manufacturing (AIIM)},
  pages = {426-433},
  year = {2024},
  doi = {10.1109/aiim64537.2024.10934570},
  url = {https://doi.org/10.1109/aiim64537.2024.10934570}
}
@inproceedings{zhong2024gtok,
  title = {G-TOK: Zero-Knowledge Proofs Based Dynamic Verifiable Credentials for Sensor Data Sharing},
  author = {Zhong, Junyi and Abreu, Thiago and Souihi, Sami and Lucas, Françoise S.},
  booktitle = {2024 6th International Conference on Blockchain Computing and Applications (BCCA)},
  pages = {400-405},
  year = {2024},
  doi = {10.1109/bcca62388.2024.10844465},
  url = {https://doi.org/10.1109/bcca62388.2024.10844465}
}
@inproceedings{lalouani2024lightweigh,
  title = {Lightweight Zero Knowledge Proof-Based Multi Access Control Schema for Smart Telehealth System},
  author = {Lalouani, Wassila and Emokpae, Lloyd},
  booktitle = {2024 International Conference on Smart Applications, Communications and Networking (SmartNets)},
  pages = {1-6},
  year = {2024},
  doi = {10.1109/smartnets61466.2024.10577736},
  url = {https://doi.org/10.1109/smartnets61466.2024.10577736}
}
@inproceedings{wagner2024madtls,
  title = {Madtls: Fine-grained Middlebox-aware End-to-end Security for Industrial Communication},
  author = {Wagner, Eric and Heye, David and Serror, Martin and Kunze, Ike and Wehrle, Klaus and Henze, Martin},
  booktitle = {Proceedings of the 19th ACM Asia Conference on Computer and Communications Security},
  pages = {962-976},
  year = {2024},
  doi = {10.1145/3634737.3637640},
  url = {https://doi.org/10.1145/3634737.3637640}
}
@inproceedings{niroula2026calypso,
  title = {Calypso: Fine-Grained Access Control for Zero-Trust Cloud Service Discovery},
  author = {Niroula, Pankaj and Boggs, Peyton and Poudel, Aashutosh and Herwig, Stephen},
  booktitle = {Proceedings of the 31st ACM Symposium on Access Control Models and Technologies},
  pages = {63-74},
  year = {2026},
  doi = {10.1145/3750555.3811897},
  url = {https://doi.org/10.1145/3750555.3811897}
}
@article{qi2026regulatory,
  title = {Regulatory-Gated Zero-Trust Access Control for Cross-Border Data Sharing with Policy-Bound Blockchain Arbitration},
  author = {Qi, He and Chen, Xianjun},
  journal = {Electronics},
  volume = {15},
  number = {20},
  pages = {4743},
  year = {2026},
  doi = {10.3390/electronics15204743},
  url = {https://doi.org/10.3390/electronics15204743}
}
```

### Inferences

- Prezta's naive 1,301.7 s (about 22 minutes) and zk-pdp's over 28 minutes for Cedar in RISC Zero are the same order, though the hardware and policies differ, so the comparison is rough. Prezta's own result shows that this order of magnitude is not a final answer for the VM path: precompiling policy-dependent work and using precompiles took it to 27.3 s.
- zk-pdp differs from Prezta in ways the paper confirms: Prezta's attributes come only from a JWT and public context, so it has no committed fact store and no relationship or hierarchy lookup. It uses policy as image, not policy as data. It has no circuit backend or side-by-side comparison with one, and it collapses Indeterminate into false, so an audit could not tell Deny from error.
- Prezta's padding result matters for any RISC Zero baseline: 14 s and 28 s are padding steps, and a small policy cannot go below the program's minimum segment cost. Comparisons should report cycles before padding as well.
- The three "binding" concerns (CVA's execution binding, the gateway's source integrity, SDAS's sender binding) could go straight into zk-pdp's threat model, as each is a distinct way a sound proof can still authorize the wrong thing.

### Gaps

- Semantic Scholar returned HTTP 429 on every attempt, through both curl and the fetch tool, so forward citations rest on OpenAlex alone. Prezta's arXiv v1 is under four months old, so zero citers is unsurprising and should be rechecked.
- I found no independent reproduction of Prezta's numbers, and did not run its artifact.
- `docs/sources/S01-prezta.md` has open items this note can close (proof sizes, hardware, artifact), but I did not edit project files outside this note.

## Privacy-preserving policy evaluation by other means: MPC, FHE, TEEs, garbled circuits, PSI

### Takeaway

Comparators mostly solve a different problem. MPC and HE protect policies and attributes from each other during evaluation. TEEs add integrity through hardware attestation. Neither gives a third party a small proof without trusting hardware or the participants, which is zk-pdp's distinguishing property. I found no paper that evaluates ABAC policy directly under FHE.

### Cited Findings

#### MPC, HE, garbled circuits

- **[R45] Privacy-preserving policy evaluation in multi-party access control (Sheikhalishahi, Stork, Zannone, JCS 2021)**, `read-abstract`, and **[R46] WPES 2019 precursor**, `search-summary-only`. *Protects:* each user's policy. *Technique:* HE and secure functional evaluation over policies in protected form. *Expressiveness:* standardized ABAC languages such as XACML, per its summary, through a Boolean encoding. A search-surfaced statement notes that XACML's three-valued logic adds challenges under privacy preservation. *Performance:* feasible, with no figures in the abstract. — [JCS](https://doi.org/10.3233/jcs-200007), [Journal page](https://content.iospress.com/articles/journal-of-computer-security/jcs200007), [WPES](https://doi.org/10.1145/3338498.3358643)
- **[R101] Private Access Control for Function Secret Sharing (Servan-Schreiber et al., IEEE S&P 2023)**, `search-summary-only`, metadata only. [background] Servers enforce an access policy on secret-shared queries without learning the query. — [Source](https://doi.org/10.1109/sp46215.2023.10179295)
- **[R102] Zeph (Burkhalter et al., OSDI 2021)**, `read-abstract`. Users set sharing rules, enforced cryptographically on encrypted streams by a hybrid protocol. Compliance by construction, and no proof of a decision. Scales to thousands of sources on Kafka. — [Source](https://www.usenix.org/conference/osdi21/presentation/burkhalter)

#### PSI and secret handshakes

- **[R103] Privacy-Preserving Policy-Based Information Transfer (De Cristofaro et al., PETS 2009)**, **[R104] Private Intersection of Certified Sets (Camenisch, Zaverucha, FC 2009)**, **[R105] Secret Handshakes (Balfanz et al.)**, `search-summary-only`, metadata only. [background] These match certified attributes or sets without disclosure, so their expressiveness is mostly set intersection or membership. — [De Cristofaro](https://doi.org/10.1007/978-3-642-03168-7_10), [Camenisch](https://doi.org/10.1007/978-3-642-03549-4_7), [Balfanz](https://doi.org/10.1109/secpri.2003.1199336)

#### TEEs

- **[R44] SPARTA (Marangone et al., 2025)**, `read-abstract`. User decision logic in DMN and access policies in ALFA become attested trusted applications. The nearest TEE counterpart to a verifiable decision, trusting the hardware. Reports modest overhead. — [Source](https://arxiv.org/abs/2509.02413), [Search summary](https://arxiv.org/pdf/2509.02413)
- **[R106] APETEEt (Godhani, Bharadhwaj, Sural, SACMAT 2023 poster)**, `search-summary-only`. ABAC policies compiled to a tree inside SGX, with evaluation time scaling with depth. — [Slides](https://www.sacmat.org/2023/resource/slides/1_5_3_GodhaniPritkumar.pdf), [DOI](https://doi.org/10.1145/3589608.3595079)
- **[R107] CAT, Confidential Attestation (Liu et al., 2020)**, `read-abstract`. A proof-carrying-code design: an untrusted generator instruments code, and a small in-enclave verifier checks policy compliance. It shows the proof-carrying idea under a hardware root of trust. — [Source](https://arxiv.org/abs/2007.10513)
- **[R108] CP-ABE with SGX (Suryawanshi, Sural, 2024)**, `read-abstract`. Encrypts and decrypts inside an enclave according to policy rules. Small overhead, with no figures in the abstract. — [Source](https://arxiv.org/abs/2409.07149)
- **[R109] PESOS (Krahn et al., EuroSys 2018)**, **[R110] Practical Verifiable In-network Filtering (Gong et al., ICDCS 2019)**, **[R111] LightBox (Duan et al., CCS 2019)**, `search-summary-only`, metadata only. [background] TEE-enforced storage policy, verifiable DDoS filtering, and stateful middleboxes. VIF and LightBox are TEE counterparts to ZKMB. — [PESOS](https://doi.org/10.1145/3190508.3190518), [VIF](https://doi.org/10.1109/icdcs.2019.00118), [LightBox](https://doi.org/10.1145/3319535.3339814)
- **[R112] BlindBox (Sherry et al., SIGCOMM 2015)**, `search-summary-only`, metadata only. [background] Inspection over encrypted traffic by searchable encryption, needing no TEE and no ZK. — [Source](https://doi.org/10.1145/2785956.2787502)
- Vendor note, not academic: Turnkey markets "verifiable policy decisions" through enclaves rather than ZK proofs. — [Turnkey](https://www.turnkey.com/blog/introducing-verifiable-policy-decisions)

#### Enforcement by encryption or signature

- **[R113] CP-ABE (Bethencourt, Sahai, Waters, IEEE S&P 2007)**, **[R114] Partially hidden structures (Nishide et al., ACNS 2008)**, **[R115] Policy-hiding ABAC (Zhang, Zheng, Deng, IoT Journal 2018)**, **[R116] Access Control Encryption (Damgård, Haagh, Orlandi, TCC 2016)**, `search-summary-only`, metadata only. [background] These enforce by who can decrypt. There is no decision to verify, so they are comparators for "the decider need not be trusted" only when the policy fits at encryption time. — [CP-ABE](https://doi.org/10.1109/sp.2007.11), [Nishide](https://doi.org/10.1007/978-3-540-68914-0_7), [Zhang](https://doi.org/10.1109/jiot.2018.2825289), [ACE](https://doi.org/10.1007/978-3-662-53644-5_21)
- **[R50] Policy-Based Signatures (Bellare, Fuchsbauer, PKC 2014)**, `search-summary-only`. Signers can sign only messages satisfying an authority-set policy, and the policy stays private. The idea is a close cousin: a signature that attests compliance. — [ePrint](https://eprint.iacr.org/2013/413)
- **[R51] Policy-Compliant Signatures (Badertscher, Matt, Waldner, TCC 2021)**, `search-summary-only`. Extends this to a sender and designated receiver whose attributes jointly satisfy a global policy, from signatures, NIZK and predicate-only encryption. — [ePrint](https://eprint.iacr.org/2021/1234)
- **[R117] Attribute-Based Signatures (Maji, Prabhakaran, Rosulek, CT-RSA 2011)**, `search-summary-only`, metadata only. — [Source](https://doi.org/10.1007/978-3-642-19074-2_24)
- **[R49] Proof-carrying authentication (Appel, Felten, CCS 1999)**, `search-summary-only`, metadata only. [background] The client builds a logic proof that the server checks. The conceptual ancestor of "the decider need not be trusted", without succinctness or privacy. — [Source](https://doi.org/10.1145/319709.319718)
- **[R118] Privacy preserving assertion based policy language (Squicciarini et al., SACMAT 2007)**, `search-summary-only`, metadata only. — [Source](https://doi.org/10.1145/1266840.1266849)
- **[R119] FE[r]Chain (Nuoskala et al., SACMAT 2024)**, `search-summary-only`, title only. Verifiable functional encryption for fair data exchange. — [Source](https://doi.org/10.1145/3649158.3657049)

#### BibTeX for this question

```bibtex
@article{sheikhalishahi2021privacypre,
  title = {Privacy-preserving policy evaluation in multi-party access control},
  author = {Sheikhalishahi, Mina and Stork, Ischa and Zannone, Nicola},
  journal = {Journal of Computer Security},
  volume = {29},
  number = {6},
  pages = {613-650},
  year = {2021},
  doi = {10.3233/jcs-200007},
  url = {https://doi.org/10.3233/jcs-200007}
}
@inproceedings{sheikhalishahi2019privacypre,
  title = {Privacy-Preserving Multi-Party Access Control},
  author = {Sheikhalishahi, Mina and Tillem, Gamze and Erkin, Zekeriya and Zannone, Nicola},
  booktitle = {Proceedings of the 18th ACM Workshop on Privacy in the Electronic Society},
  pages = {1-13},
  year = {2019},
  doi = {10.1145/3338498.3358643},
  url = {https://doi.org/10.1145/3338498.3358643}
}
@inproceedings{servanschreiber2023private,
  title = {Private Access Control for Function Secret Sharing},
  author = {Servan-Schreiber, Sacha and Beyzerov, Simon and Yablon, Eli and Park, Hyojae},
  booktitle = {2023 IEEE Symposium on Security and Privacy (SP)},
  pages = {809-828},
  year = {2023},
  doi = {10.1109/sp46215.2023.10179295},
  url = {https://doi.org/10.1109/sp46215.2023.10179295}
}
@inproceedings{burkhalter2021zeph,
  title = {Zeph: Cryptographic Enforcement of End-to-End Data Privacy},
  author = {Burkhalter, Lukas and K{\"u}chler, Nicolas and Viand, Alexander and Shafagh, Hossein and Hithnawi, Anwar},
  booktitle = {15th USENIX Symposium on Operating Systems Design and Implementation (OSDI 21)},
  pages = {387--404},
  year = {2021},
  month = jul,
  publisher = {USENIX Association},
  url = {https://www.usenix.org/conference/osdi21/presentation/burkhalter}
}
@inproceedings{decristofaro2009privacypre,
  title = {Privacy-Preserving Policy-Based Information Transfer},
  author = {De Cristofaro, Emiliano and Jarecki, Stanislaw and Kim, Jihye and Tsudik, Gene},
  booktitle = {Lecture Notes in Computer Science},
  pages = {164-184},
  year = {2009},
  doi = {10.1007/978-3-642-03168-7_10},
  url = {https://doi.org/10.1007/978-3-642-03168-7_10}
}
@inproceedings{camenisch2009private,
  title = {Private Intersection of Certified Sets},
  author = {Camenisch, Jan and Zaverucha, Gregory M.},
  booktitle = {Lecture Notes in Computer Science},
  pages = {108-127},
  year = {2009},
  doi = {10.1007/978-3-642-03549-4_7},
  url = {https://doi.org/10.1007/978-3-642-03549-4_7}
}
@inproceedings{balfanzsecret,
  title = {Secret handshakes from pairing-based key agreements},
  author = {Balfanz, D. and Durfee, G. and Shankar, N. and Smetters, D. and Staddon, J. and Hao-Chi Wong},
  pages = {180-196},
  doi = {10.1109/secpri.2003.1199336},
  url = {https://doi.org/10.1109/secpri.2003.1199336},
  note = {Crossref has no issue year, and its container title reads Proceedings 19th International Conference on Data Engineering, which conflicts with the DOI slug secpri.2003. Venue left out as unverified.}
}
@misc{marangone2025sparta,
  title = {A Secure, Confidential, and Verifiable Decision Support System},
  author = {Marangone, Edoardo and Nemmi, Eugenio Nerio and Friolo, Daniele and Ateniese, Giuseppe and Weber, Ingo and Di Ciccio, Claudio},
  year = {2025},
  eprint = {2509.02413},
  archivePrefix = {arXiv},
  doi = {10.48550/arXiv.2509.02413},
  url = {https://arxiv.org/abs/2509.02413}
}
@inproceedings{godhani2023poster,
  title = {Poster: APETEEt -- Secure Enforcement of ABAC Policies using Trusted Execution Environment},
  author = {Godhani, Pritkumar and Bharadhwaj, Rahul and Sural, Shamik},
  booktitle = {Proceedings of the 28th ACM Symposium on Access Control Models and Technologies},
  pages = {47-49},
  year = {2023},
  doi = {10.1145/3589608.3595079},
  url = {https://doi.org/10.1145/3589608.3595079}
}
@misc{liu2020cat,
  title = {Confidential Attestation: Efficient in-Enclave Verification of Privacy Policy Compliance},
  author = {Liu, Weijie and Wang, Wenhao and Wang, Xiaofeng and Meng, Xiaozhu and Lu, Yaosong and Chen, Hongbo and Wang, Xinyu and Shen, Qingtao and Chen, Kai and Tang, Haixu and Chen, Yi and Xing, Luyi},
  year = {2020},
  eprint = {2007.10513},
  archivePrefix = {arXiv},
  doi = {10.48550/arXiv.2007.10513},
  url = {https://arxiv.org/abs/2007.10513}
}
@misc{suryawanshi2024cpabesgx,
  title = {Ciphertext Policy Attribute Based Encryption with {Intel SGX}},
  author = {Suryawanshi, Vivek and Sural, Shamik},
  year = {2024},
  eprint = {2409.07149},
  archivePrefix = {arXiv},
  doi = {10.48550/arXiv.2409.07149},
  url = {https://arxiv.org/abs/2409.07149}
}
@inproceedings{krahn2018p,
  title = {Pesos: policy enhanced secure object store},
  author = {Krahn, Robert and Trach, Bohdan and Vahldiek-Oberwagner, Anjo and Knauth, Thomas and Bhatotia, Pramod and Fetzer, Christof},
  booktitle = {Proceedings of the Thirteenth EuroSys Conference},
  pages = {1-17},
  year = {2018},
  doi = {10.1145/3190508.3190518},
  url = {https://doi.org/10.1145/3190508.3190518},
  note = {Crossref title reads "P esos", a small-caps artifact.}
}
@inproceedings{gong2019practical,
  title = {Practical Verifiable In-network Filtering for DDoS Defense},
  author = {Gong, Deli and Tran, Muoi and Shinde, Shweta and Jin, Hao and Sekar, Vyas and Saxena, Prateek and Kang, Min Suk},
  booktitle = {2019 IEEE 39th International Conference on Distributed Computing Systems (ICDCS)},
  pages = {1161-1174},
  year = {2019},
  doi = {10.1109/icdcs.2019.00118},
  url = {https://doi.org/10.1109/icdcs.2019.00118}
}
@inproceedings{duan2019lightbox,
  title = {LightBox: Full-stack Protected Stateful Middlebox at Lightning Speed},
  author = {Duan, Huayi and Wang, Cong and Yuan, Xingliang and Zhou, Yajin and Wang, Qian and Ren, Kui},
  booktitle = {Proceedings of the 2019 ACM SIGSAC Conference on Computer and Communications Security},
  pages = {2351-2367},
  year = {2019},
  doi = {10.1145/3319535.3339814},
  url = {https://doi.org/10.1145/3319535.3339814}
}
@inproceedings{sherry2015blindbox,
  title = {BlindBox: Deep Packet Inspection over Encrypted Traffic},
  author = {Sherry, Justine and Lan, Chang and Popa, Raluca Ada and Ratnasamy, Sylvia},
  booktitle = {Proceedings of the 2015 ACM Conference on Special Interest Group on Data Communication},
  pages = {213-226},
  year = {2015},
  doi = {10.1145/2785956.2787502},
  url = {https://doi.org/10.1145/2785956.2787502}
}
@inproceedings{bethencourt2007ciphertext,
  title = {Ciphertext-Policy Attribute-Based Encryption},
  author = {Bethencourt, John and Sahai, Amit and Waters, Brent},
  booktitle = {2007 IEEE Symposium on Security and Privacy (SP '07)},
  pages = {321-334},
  year = {2007},
  doi = {10.1109/sp.2007.11},
  url = {https://doi.org/10.1109/sp.2007.11}
}
@inproceedings{nishide2008attributeb,
  title = {Attribute-Based Encryption with Partially Hidden Encryptor-Specified Access Structures},
  author = {Nishide, Takashi and Yoneyama, Kazuki and Ohta, Kazuo},
  booktitle = {Lecture Notes in Computer Science},
  pages = {111-129},
  year = {2008},
  doi = {10.1007/978-3-540-68914-0_7},
  url = {https://doi.org/10.1007/978-3-540-68914-0_7}
}
@article{zhang2018security,
  title = {Security and Privacy in Smart Health: Efficient Policy-Hiding Attribute-Based Access Control},
  author = {Zhang, Yinghui and Zheng, Dong and Deng, Robert H.},
  journal = {IEEE Internet of Things Journal},
  volume = {5},
  number = {3},
  pages = {2130-2145},
  year = {2018},
  doi = {10.1109/jiot.2018.2825289},
  url = {https://doi.org/10.1109/jiot.2018.2825289}
}
@inproceedings{damgaard2016access,
  title = {Access Control Encryption: Enforcing Information Flow with Cryptography},
  author = {Damgård, Ivan and Haagh, Helene and Orlandi, Claudio},
  booktitle = {Lecture Notes in Computer Science},
  pages = {547-576},
  year = {2016},
  doi = {10.1007/978-3-662-53644-5_21},
  url = {https://doi.org/10.1007/978-3-662-53644-5_21}
}
@inproceedings{bellare2014policybase,
  title = {Policy-Based Signatures},
  author = {Bellare, Mihir and Fuchsbauer, Georg},
  booktitle = {Lecture Notes in Computer Science},
  pages = {520-537},
  year = {2014},
  doi = {10.1007/978-3-642-54631-0_30},
  url = {https://doi.org/10.1007/978-3-642-54631-0_30}
}
@inproceedings{badertscher2021policycomp,
  title = {Policy-Compliant Signatures},
  author = {Badertscher, Christian and Matt, Christian and Waldner, Hendrik},
  booktitle = {Lecture Notes in Computer Science},
  pages = {350-381},
  year = {2021},
  doi = {10.1007/978-3-030-90456-2_12},
  url = {https://doi.org/10.1007/978-3-030-90456-2_12}
}
@inproceedings{maji2011attributeb,
  title = {Attribute-Based Signatures},
  author = {Maji, Hemanta K. and Prabhakaran, Manoj and Rosulek, Mike},
  booktitle = {Lecture Notes in Computer Science},
  pages = {376-392},
  year = {2011},
  doi = {10.1007/978-3-642-19074-2_24},
  url = {https://doi.org/10.1007/978-3-642-19074-2_24}
}
@inproceedings{appel1999proofcarry,
  title = {Proof-carrying authentication},
  author = {Appel, Andrew W. and Felten, Edward W.},
  booktitle = {Proceedings of the 6th ACM conference on Computer and communications security},
  pages = {52-62},
  year = {1999},
  doi = {10.1145/319709.319718},
  url = {https://doi.org/10.1145/319709.319718}
}
@inproceedings{squicciarini2007a,
  title = {A privacy preserving assertion based policy language for federation systems},
  author = {Squicciarini, Anna C. and Hintoglu, Ayca Azgin and Bertino, Elisa and Saygin, Yucel},
  booktitle = {Proceedings of the 12th ACM symposium on Access control models and technologies},
  pages = {51-60},
  year = {2007},
  doi = {10.1145/1266840.1266849},
  url = {https://doi.org/10.1145/1266840.1266849}
}
@inproceedings{nuoskala2024fer,
  title = {FE[r]Chain: Enforcing Fairness in Blockchain Data Exchanges Through Verifiable Functional Encryption},
  author = {Nuoskala, Camille and Rabbaninejad, Reyhaneh and Dimitriou, Tassos and Michalas, Antonis},
  booktitle = {Proceedings of the 29th ACM Symposium on Access Control Models and Technologies},
  pages = {183-191},
  year = {2024},
  doi = {10.1145/3649158.3657049},
  url = {https://doi.org/10.1145/3649158.3657049}
}
```

### Inferences

- The useful comparison axis is who a third party must trust after the fact. MPC: enough of the participants. TEE: the chip vendor and attestation chain. ZK: only the proof system and the published fingerprints.
- SPARTA suggests a TEE baseline is worth stating explicitly in zk-pdp's paper, since hardware attestation answers the same question at far lower latency under a stronger trust assumption.
- Three-valued logic is a known complication under privacy, and Prezta sidesteps it by collapsing to binary. Cedar's permit, forbid and evaluation-error rules are a matching concern for the circuit backend.

### Gaps

- I found no paper evaluating ABAC policies directly under FHE. The search returned only general FHE background and FHE access-control lists.
- I found no garbled-circuit XACML paper I could identify with confidence. A search summary mentioned one design but did not tie it to a retrievable source, so it is omitted.
- No study compares ZK, MPC and TEE on one policy workload. Nothing I found gives comparable numbers.

## What is the expressive limit of each: fixed predicates, boolean formulas, general policy language?

### Takeaway

I sort the work into five levels, and most papers sit at the lowest two. Only Prezta reaches a standard general policy language with measured conformance. No paper reaches Cedar, and no paper measures one policy set on both a zkVM and a compiled circuit.

### Cited Findings

| Level | What can be expressed | Examples | Cost shape reported |
|---|---|---|---|
| E0: fixed statement | One relation, no policy | [ZK-ACE](https://arxiv.org/html/2603.07974v3), [ZK-EHR's published circuit](https://raw.githubusercontent.com/FbalwyTU/ZK-EHR/main/circuits/access.circom), [zkPoC](https://doi.org/10.1145/3842755) | Under 1 s where reported |
| E1: predicate families | Equality, range, membership, thresholds, set inclusion | [Piper et al.](https://arxiv.org/abs/2510.05807), [Privacy Pools](https://libmast.utm.my/Record/doaj.art-a92b923f2dbb489d86a1fed5f7eccfe8), [agent gateway](https://arxiv.org/abs/2608.30083), [Werapun](https://doi.org/10.3390/cryptography10050066), [ZKMB blocklists](https://www.usenix.org/system/files/sec22-grubbs.pdf) | ms to seconds, plus credential cost |
| E2: boolean or combining logic | AND, OR, rule combining, thresholds | [POLARIS VPPL, without ZK](https://arxiv.org/pdf/2511.22017), [zkAt, implemented only for a threshold](https://eprint.iacr.org/2025/921), [OT with hidden policies, conjunctions](https://www.scitepress.org/Papers/2014/50169/) | Varies |
| E3: bounded DSL or per-policy compile | Templates, a transpiled policy, a bounded interpreter, regex, grammars | [zkGen](https://tum-esi.github.io/publications-list/PDF/2024-ICBC-zkGen.pdf), [Cinderella](https://www.ieee-security.org/TC/SP2016/slides/23-3/delignat.pdf), [zk-Guard-R, interpreter unimplemented](https://doi.org/10.3390/s26165045), [ZK-TPL](https://arxiv.org/abs/2212.02454), [Reef](https://www.usenix.org/conference/usenixsecurity24/presentation/angel), [Zombie](https://www.usenix.org/system/files/nsdi24-zhang-collin.pdf) | Seconds, with size set by the policy |
| E4: general language or relation | A standard policy language, or arbitrary programs and relations | [Prezta, XACML 323 of 389](https://arxiv.org/pdf/2607.11466), [zk-creds, general relations](https://eprint.iacr.org/2022/878), [zk-promises, Turing-complete state](https://eprint.iacr.org/2024/1260) | Tens of seconds in a zkVM, with a floor set by padding |

Verify costs from the cited papers rarely depend on policy:

- Groth16-based systems: [ZKMB 2 to 5 ms](https://www.usenix.org/system/files/sec22-grubbs.pdf), [zk-Guard-R 0.73 ms](https://doi.org/10.3390/s26165045).
- STARK-based Prezta: [14 to 15.5 ms, 238 to 250 KB](https://arxiv.org/pdf/2607.11466).

### Inferences

- zk-creds and zk-promises reach E4 as relations, but a developer writes the relation, so a policy author has no language. The practical question for zk-pdp is E3 or E4 with a standard language and known semantics.
- zk-pdp's two backends straddle E3 and E4 by design. The prior art has a measured example of each but never on a shared workload, so that comparison looks like the clearest original contribution. This rests on absence from my searches, not on an exhaustive review.
- Cedar features that the surveyed systems have no counterpart to, on the evidence I saw: entity hierarchy `in` over a committed store, and forbid-overrides with error semantics preserved in the proof output. Prezta's collapse to binary is the nearest related choice.

### Gaps

- Papers rarely state their expressive limit formally. Most levels above are my classification from abstracts and excerpts.
- For `search-summary-only` entries I did not classify expressiveness, and they are marked "not classified" below.
- Prezta's fixed padding and zk-Guard-R's scaling are the only prover-versus-policy-size data I retrieved. I did not extract scaling curves from the others.

### Ranked index

Ranked by relevance to zk-pdp: first by direct overlap with verifiable policy decisions, then by what the project could reuse or must answer. The order within a tier is my judgment.

| Rank | Paper | Level | Status |
|---|---|---|---|
| R1 | Prezta | E4 | `read-full` |
| R2 | zkGen / zkPolicy | E3 | `read-full` |
| R3 | ZK-TPL | E3 | `read-abstract + excerpts` |
| R4 | zk-Guard-R | E3 designed, E1 measured | `read-abstract` |
| R5 | Cinderella | E3 | `search-summary-only` |
| R6 | Verifiable Agent Authorization | E0 prototype | `read-abstract + excerpts` |
| R7 | On-chain KYC permissioning | E1 | `read-abstract + excerpts` |
| R8 | Zero-Knowledge Middleboxes | E1 | `read-abstract + excerpts` |
| R9 | Zombie | E3 | `read-abstract + excerpts` |
| R10 | AUDIT, accountability of secret processes | Tunable statements | `search-summary-only` |
| R11 | zkAt / zkAt+ | E2 implemented | `read-abstract` |
| R12 | zk-creds | E4 relation | `read-abstract` |
| R13 | ZK predicate proofs between agents | E1 | `read-abstract` |
| R14 | ZK-EHR (health data) | E0 as published | `read-abstract + repo code` |
| R15 | POLARIS (no ZK) | E2, no proof | `read-full` |
| R16 | zkSAS | Fixed regulatory rules | `read-abstract` |
| R17 | eIDAS compliance via zkVM | Not classified | `search-summary-only` |
| R18 | Kerl et al. ABAC | Not classified | `read-abstract` |
| R19 | Crescent | E1 over JWT and mDL | `read-abstract` |
| R20 | Anonymous credentials from ECDSA | E1 | `read-abstract` |
| R21 | zkLogin | Not classified | `search-summary-only` |
| R22 | Proofs of Possession for Legacy Signatures | Not classified | `search-summary-only` |
| R23 | ZKP-CapBAC | Not classified | `search-summary-only` |
| R24 | zk-promises | E4 state machine | `read-abstract` |
| R25 | SAMPL | Not classified | `search-summary-only` |
| R26 | ZEXE | Not classified | `search-summary-only` |
| R27 | zkay | Not classified | `search-summary-only` |
| R28 | Selective Disclosure Authorization Schemes | E1 | `read-abstract` |
| R29 | Health insurance eligibility | E1 | `read-abstract` |
| R30 | Reef | E3 regex | `read-abstract` |
| R31 | Coral | Not classified | `search-summary-only` |
| R32 | ZK-ACE | E0 | `read-abstract` |
| R33 | ZK Policy-Embedded Access Control | Not classified | `search-summary-only` |
| R34 | Bilateral access control, IIoT | Not classified | `search-summary-only` |
| R35 | Anonymous Command Authorization | Not classified | `search-summary-only` |
| R36 | zk-IoT | Not classified | `search-summary-only` |
| R37 | FaaS policy validation with ZKMBs | E3 | `read-abstract` |
| R38 | FaaS anomalous invocation | Not classified | `search-summary-only` |
| R39 | ZK-rollup access control | Not classified | `read-abstract` |
| R40 | Privacy Pools | E1 | `search-summary-only` |
| R41 | zkKYC | Concept only | `read-abstract` |
| R42 | REGKYC | Not classified | `read-abstract` |
| R43 | zkLedger | Fixed query types | `read-abstract` |
| R44 | SPARTA (TEE) | DMN and ALFA, no ZK | `read-abstract` |
| R45 | Multi-party policy evaluation, JCS | XACML-style, no proof | `read-abstract` |
| R46 | Multi-party access control, WPES | Not classified | `search-summary-only` |
| R47 | zk-MCP audit | Not classified | `read-abstract` |
| R48 | Aegis Protocol | Simulated | `read-abstract` |
| R49 | Proof-carrying authentication | Not classified | `search-summary-only` |
| R50 | Policy-Based Signatures | Not classified | `search-summary-only` |
| R51 | Policy-Compliant Signatures | Not classified | `search-summary-only` |
| R52 | Card requirements language | Not classified | `search-summary-only` |
| R53 | IRAC | E1 | `read-abstract + excerpts` |
| R54 | Issuer-Hiding ABCs | Not classified | `read-abstract` |
| R55 | ZKlaims | Not classified | `search-summary-only` |
| R56 | CanDID | Not classified | `search-summary-only` |
| R57 | DECO | Not classified | `search-summary-only` |
| R58 | NOPE | Not classified | `search-summary-only` |
| R59 | SRv6 ZK authorization | Not classified | `search-summary-only` |
| R60 | SPIDeR | Not classified | `search-summary-only` |
| R61 | REMOTEGATE | No ZK used | `read-abstract` |
| R62 | Oblivious firewall policies | Not classified | `search-summary-only` |
| R63 | Accountable Privacy, payments | Not classified | `search-summary-only` |
| R64 | Platypus | Not classified | `search-summary-only` |
| R65 | PEReDi | Not classified | `search-summary-only` |
| R66 | zkMixer | Not classified | `read-abstract` |
| R67 | Private Smart Wallet | Not classified | `read-abstract` |
| R68 | Auditable cross-chain DeFi | E1 | `read-abstract` |
| R69 | ZK-Compliance poster | E1 | `read-abstract` |
| R70 | zkPoC | E0 | `read-abstract` |
| R71 | Emission Impossible | Not classified | `read-abstract` |
| R72 | Podda et al. legal analysis | Not applicable | `search-summary-only` |
| R73 | Privacy-Preserving Cloud Architecture | Not classified | `read-abstract` |
| R74 | Larch | Not classified | `search-summary-only` |
| R75 | Zero-Knowledge Address Abstraction | Not classified | `search-summary-only` |
| R76 | Caplets | No ZK | `search-summary-only` |
| R77 | Active certificates | No ZK | `search-summary-only` |
| R78 | Stateful least privilege | No ZK | `search-summary-only` |
| R79 | Collaborative zk-SNARKs | Not applicable | `search-summary-only` |
| R80 | zkSaaS | Not applicable | `search-summary-only` |
| R81 | Cross-Domain AC, Industrial Internet | Not classified | `search-summary-only` |
| R82 | G-TOK | Not classified | `search-summary-only` |
| R83 | Lightweight ZKP multi access control | Not classified | `search-summary-only` |
| R84 | Wu et al. IoT access control | Not classified | `read-abstract` |
| R85 | Maalla, Bezzateev | Not classified | `search-summary-only` |
| R86 | Hu et al. | Not classified | `search-summary-only` |
| R87 | SSIBAC | Not classified | `search-summary-only` |
| R88 | Camenisch, Lysyanskaya 2001 | Not classified | `search-summary-only` |
| R89 | idemix | Not classified | `search-summary-only` |
| R90 | Hidden Credentials | Not classified | `search-summary-only` |
| R91 | Hidden policies with hidden credentials | Not classified | `search-summary-only` |
| R92 | ABAC with hidden policies | Not classified | `search-summary-only` |
| R93 | OACerts | Not classified | `search-summary-only` |
| R94 | Policy-hiding access control | Not classified | `search-summary-only` |
| R95 | Oblivious signature-based envelope | Not classified | `search-summary-only` |
| R96 | Private Policy Negotiation | Not classified | `search-summary-only` |
| R97 | Automated trust negotiation | Not classified | `search-summary-only` |
| R98 | OT with access control | Not classified | `search-summary-only` |
| R99 | OT with hidden policies | E2, conjunctions | `search-summary-only` |
| R100 | Hiding the Policy | Not classified | `search-summary-only` |
| R101 | Private Access Control for FSS | Not classified | `search-summary-only` |
| R102 | Zeph | No decision proof | `read-abstract` |
| R103 | Policy-Based Information Transfer | Not classified | `search-summary-only` |
| R104 | Private Intersection of Certified Sets | Not classified | `search-summary-only` |
| R105 | Secret Handshakes | Not classified | `search-summary-only` |
| R106 | APETEEt | ABAC tree in SGX, no ZK | `search-summary-only` |
| R107 | CAT | No ZK | `read-abstract` |
| R108 | CP-ABE with SGX | No ZK | `read-abstract` |
| R109 | PESOS | Not classified | `search-summary-only` |
| R110 | Verifiable In-network Filtering | Not classified | `search-summary-only` |
| R111 | LightBox | Not classified | `search-summary-only` |
| R112 | BlindBox | Not classified | `search-summary-only` |
| R113 | CP-ABE | Not classified | `search-summary-only` |
| R114 | Partially hidden structures | Not classified | `search-summary-only` |
| R115 | Policy-hiding ABAC, smart health | Not classified | `search-summary-only` |
| R116 | Access Control Encryption | Not classified | `search-summary-only` |
| R117 | Attribute-Based Signatures | Not classified | `search-summary-only` |
| R118 | Assertion based policy language | Not classified | `search-summary-only` |
| R119 | FE[r]Chain | Not classified | `search-summary-only` |
| R120 | Madtls | Not classified | `search-summary-only` |
| R121 | Calypso | Not classified | `search-summary-only` |
| R122 | Regulatory-Gated Zero-Trust | No ZK in abstract | `read-abstract` |
