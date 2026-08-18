# CRP-SpMM (scalable-matrix/CRP-SpMM)

User-supplied 2026-08-18. Communication-reduced parallel sparse×dense matrix multiply (MPI + OpenMP + MKL).

| item | value |
|------|--------|
| Repo | https://github.com/scalable-matrix/CRP-SpMM |
| Disk / CBM | cloned `/tmp/user-url-scout/crp-spmm` for this scout; not vendored; not indexed |
| Default confidence | `code` only for files actually read |
| Read | README; `src/para2d_spmm.h` / `para2d_spmm.c`; `src/rowpara_spmm.h`; `deprecated/SC23_AD/readme.md` |
| Paper | SC23: Communication-Reduced Sparse-Dense Matrix Multiplication with Adaptive Parallelization |
| Closed card | `crp-spmm-comm-reduced` |
| Do not | make MPI/MKL a Kutha quantum; replace leapfrog with SpMM; treat as GPU WCOJ |

Kutha mapping: Data/Query **multi-node GraphBLAS lease** (SpMM schedule). Distinct from single-node Samyama CSR, Falkor GraphBLAS API (`spec`), SciRS in-process CSR, ULTRA relational `rspmm`.
