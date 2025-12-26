# HodlHunt Solana Program

Minimal source bundle for public verification of the on-chain program.

## Program
- Program ID: `B1osUCap5eJ2iJnbRqfCQB87orhJM5EqZqPcGMbjJvXz`

## Toolchain
- Anchor: **0.31.1** (must use this exact version)
- Solana program crate: 2.3.0
- Rust edition: 2021

## Build Instructions

**IMPORTANT:** To build the contract, you must use Anchor version **0.31.1** with the verification flag.

```bash
# Verify that Anchor 0.31.1 is being used
anchor --version  # should output 0.31.1

# Build with verification flag
anchor build --verifiable
```

The `--verifiable` flag creates a binary file ready for on-chain verification.

## Project layout
```
Anchor.toml
Cargo.lock
programs/hodlhunt/Cargo.toml
programs/hodlhunt/src/**   (all source files)
```

## Verify via OtterSec API
Request (async):
```bash
curl -X POST https://verify.osec.io/verify \
  -H "Content-Type: application/json" \
  -d '{
    "repository": "https://github.com/uniwexLab/hodlhunt_pvp",
    "program_id": "B1osUCap5eJ2iJnbRqfCQB87orhJM5EqZqPcGMbjJvXz",
    "commit_hash": "REPLACE_WITH_COMMIT",
    "lib_name": "hodlhunt"
  }'
```

Check status:
```bash
curl https://verify.osec.io/status/B1osUCap5eJ2iJnbRqfCQB87orhJM5EqZqPcGMbjJvXz | jq
```

Build logs:
```bash
curl https://verify.osec.io/logs/B1osUCap5eJ2iJnbRqfCQB87orhJM5EqZqPcGMbjJvXz | jq
```

## Notes
- Keep the repository public and pinned to the exact commit used for verification.
- Do not include private keys, target/ artifacts, or `.so` binaries in the repo.
