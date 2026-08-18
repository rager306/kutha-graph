# RocksDB dossier (Wave 1)

CBM: `root-vendor-source-rocksdb`. Storage engine, not a graph product.

## Extracted

| card | confidence | note |
|------|------------|------|
| rocksdb-wal-recovery | code | WALRecoveryMode + PIT default |

## Not extracted

Compaction filters, BlobDB, transactions/2PC — Wave 2 if event-log design needs them. Do not treat LSM as graph SoT.
