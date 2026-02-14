# Detangling Notes

This page tracks approved mappings and pending review candidates for SoulseekQt reverse engineering.

## Approved Function Renames

### `FUN_10006c590` -> `server_message_code_to_string`
- Binary: `SoulseekQt`
- Address: `0x10006c590`
- Confidence: `high`
- Status: `approved`
- Evidence:
  - `ghidra_decompile`: `evidence/reverse/server_messagecodetostring_otool.txt` (Jump table maps integer server message codes to SM_* literals.)
  - `string`: `evidence/reverse/message_name_strings.txt` (SM_* literals present in binary strings.)

## Approved Data Labels

No approved data labels yet.

## Review Queue

Review queue is empty.
