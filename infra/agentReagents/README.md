# agentReagents — bearDog Validation Substrates

Declarative VM/device templates consumed by the canonical
`infra/agentReagents/` image builder for bearDog-specific validation
substrates.

## Templates

| Template | Target | Purpose |
|----------|--------|---------|
| `beardog-android-keystore-validation.yaml` | Pixel 8 / eastGate | Android Keystore + StrongBox HSM validation |
| `beardog-linux-hsm-validation.yaml` | x86_64 / flockGate | Linux SecretService + Software HSM regression |

## How Templates Are Consumed

These templates are referenced by the canonical agentReagents builder
at `infra/agentReagents/`. To build a substrate:

```bash
cd /path/to/ecoPrimals/infra/agentReagents
agent-reagents build \
    --template ../../primals/bearDog/infra/agentReagents/beardog-android-keystore-validation.yaml \
    --output /var/lib/agentReagents/beardog-android-keystore.qcow2
```

Or deploy into a benchScale lab:

```bash
benchscale lab create \
    --topology ecoprimals-tower-2node \
    --reagent ../../primals/bearDog/infra/agentReagents/beardog-android-keystore-validation.yaml
```

## Relationship to Canonical agentReagents

The canonical agentReagents repo provides gate-level templates
(`gate-aarch64-pixelgate.yaml`, `gate-ubuntu24-biomeos.yaml`).

These primal-specific templates are **focused validation substrates** —
scoped to bearDog's HSM backends rather than full gate deployments.
They are designed to be composed with gate templates or used standalone
for targeted testing.
