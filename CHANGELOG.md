# Changelog

## Unreleased

### Added

- Jetson APE/I2S2 audio frontend support. `genie-audio.service` now runs
  `/opt/geniepod/bin/genie-audio-init` at boot to configure the Tegra AHUB
  route (`ADMAIF1 Mux = I2S2`, I2S2 codec master mode, framing, channel,
  and bit-format controls) so an external I2S source on the Jetson 40-pin
  header — e.g. ESP32-LyraT V4.3 via JP4 — is surfaced through ALSA as
  `plughw:APE,0`. The script is idempotent, waits up to 30 s for the APE
  card to enumerate, and exits cleanly on hosts without the I2S2 overlay.
- `detect-audio-device.sh` now prefers `plughw:APE,0` when `ADMAIF1 Mux` is
  routed to `I2S2`, falling back to USB audio and then card 0.
- `genie-core::detect_audio_device` delegates to the deploy script when
  installed, so `audio_device = "auto"` works for both LyraT and USB users
  without touching `/etc/geniepod/geniepod.toml`.
- `doc/lyrat-jetson-audio.md` — GenieClaw-side install slice for the
  LyraT-on-Jetson audio frontend. Hardware bring-up (firmware, wiring,
  Jetson-IO overlay, byte-exact verification) lives in the
  `ai-hardware-engineer-roadmap` LyraT-Jetson guide; this page covers only
  the genie-claw integration, reboot persistence, and known limitations.
- `setup-jetson.sh` now audits voice-runtime prerequisites (`whisper-cli`,
  whisper model, `piper`, piper voice + `.onnx.json` sidecar) against the
  paths in `[core]` config. Voice prereqs are not auto-downloaded — too
  large and license-sensitive — but the install script now surfaces what
  is missing with concrete install pointers instead of letting the first
  voice-loop invocation fail mysteriously. The `geniepod.target` symlink
  is also created so every `WantedBy=geniepod.target` service auto-starts
  on boot.

### Changed

- `genie-core` now binds to `127.0.0.1` by default through
  `[core].bind_host`, reducing accidental LAN exposure of chat, memory, tool,
  and actuation APIs.
- First-party dashboard and CLI chat requests now send `X-Genie-Origin`; chat
  requests without an origin header are treated as `api` instead of
  `dashboard`.
- Voice speaker identity now receives the captured WAV before cleanup, keeping
  the local biometric recognizer boundary viable for the next alpha.
- Local speaker identification now supports offline WAV-derived profile
  enrollment and matching through `genie-ctl speaker`.
- Speaker profile management now supports live microphone enrollment, WAV
  recording, and profile removal from `genie-ctl`.

## 1.0.0-alpha.4 - 2026-04-25

Alpha 4 is a control-plane hardening release. It moves GenieClaw closer to a
safe local physical agent by making runtime state, tool use, actuation, and
native skills observable and policy-controlled.

### Added

- Runtime contract endpoint and boot log for prompt, tool, policy, and
  hydration fingerprints.
- Optional runtime contract drift detection through
  `[core].expected_runtime_contract_hash`.
- `genie-ctl support-bundle` for local field diagnostics.
- Privacy-preserving tool audit log at `<data_dir>/runtime/tool-audit.jsonl`.
- Actuation channel allowlist and per-origin physical-action rate limits.
- Origin-aware tool policy through `[core.tool_policy]`.
- Native skill sidecar manifest audit metadata.
- Configurable native skill load policy through `[core.skill_policy]`.
- Support-bundle tails for runtime contract, tool audit, and actuation audit logs.

### Changed

- Skill listing now reports manifest status, permissions, capabilities, review
  identity, and signing-material presence.
- Runtime policy status now exposes tool policy, tool audit status, actuation
  limits, skill policy, and loaded skill manifest metadata.
- Documentation now separates current implementation from later work such as
  cryptographic skill signatures and stronger native skill sandboxing.

### Notes

- Skill signature checking is presence-only in this alpha; cryptographic
  verification is still future signed-skill-platform work.
- Tool audit intentionally records argument keys and output length, not argument
  values or outputs.
- Defaults preserve current behavior unless an operator enables stricter
  `skill_policy`, `tool_policy`, or actuation origin/rate settings.
