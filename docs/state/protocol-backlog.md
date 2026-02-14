# Protocol Backlog (Post Stage 2)

## Objective

Expandir cobertura hacia mapeo completo del protocolo por dominios funcionales.

## Lote A - Mensajería y Chat

- `SM_SAY_CHATROOM`
- `SM_MESSAGE_USERS`
- `SM_JOIN_ROOM`
- `SM_LEAVE_ROOM`
- `SM_USER_JOINED_ROOM`
- `SM_USER_LEFT_ROOM`

## Lote B - Rooms y Moderación

- `SM_ROOM_LIST`
- `SM_ROOM_MEMBERS`
- `SM_ROOM_OPERATORS`
- `SM_ADD_ROOM_MEMBER`
- `SM_REMOVE_ROOM_MEMBER`

## Lote C - Recomendaciones y Discovery

- `SM_GET_RECOMMENDATIONS`
- `SM_GET_MY_RECOMMENDATIONS`
- `SM_GET_GLOBAL_RECOMMENDATIONS`
- `SM_GET_USER_RECOMMENDATIONS`
- `SM_GET_SIMILAR_TERMS`

## Lote D - Peer avanzado

- `PM_USER_INFO_REQUEST`
- `PM_USER_INFO_REPLY`
- `PM_EXACT_FILE_SEARCH_REQUEST`
- `PM_INDIRECT_FILE_SEARCH_REQUEST`
- `PM_UPLOAD_PLACE_IN_LINE_REQUEST`

## Strategy

1. Añadir mensajes por lote en `analysis/ghidra/maps/message_map.csv`.
2. Registrar evidencia estática + runtime por entrada.
3. Promover solo `high` confidence a rutas core de SDK/CLI.
4. Extender `rust/protocol` + `rust/core` + `rust/cli` + fixtures redacted por lote.
