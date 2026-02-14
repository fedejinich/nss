'use strict';

const MODULE_NAME = 'SoulseekQt';
const LINK_BASE = ptr('0x100000000');
const base = Module.findBaseAddress(MODULE_NAME);

if (base === null) {
  throw new Error('Unable to locate module base for ' + MODULE_NAME);
}

const HOOKS = [
  { id: 'server_prepare_search', abs: '0x1000615c0' },
  { id: 'server_file_search', abs: '0x100060fa0' },
  { id: 'server_send_message', abs: '0x100054dac' },
  { id: 'server_handle_message', abs: '0x10005521c' },
  { id: 'peer_queue_download', abs: '0x1000a4474' },
  { id: 'peer_send_message', abs: '0x1000a0328' },
  { id: 'peer_handle_message', abs: '0x1000964ec' },
  { id: 'transfer_on_queue_download', abs: '0x1000f0478' },
  { id: 'transfer_on_file_request', abs: '0x1000d7114' },
  { id: 'download_read_socket', abs: '0x1000eb9fc' },
  { id: 'upload_write_socket', abs: '0x1000ea3cc' },
];

function runtimeAddress(abs) {
  return base.add(ptr(abs).sub(LINK_BASE));
}

function argList(args, count) {
  const values = [];
  for (let i = 0; i < count; i += 1) {
    values.push(args[i].toString());
  }
  return values;
}

function installHook(spec) {
  const target = runtimeAddress(spec.abs);
  Interceptor.attach(target, {
    onEnter(args) {
      send({
        event: 'enter',
        hook_id: spec.id,
        abs_address: spec.abs,
        runtime_address: target.toString(),
        timestamp_ms: Date.now(),
        thread_id: Process.getCurrentThreadId(),
        args: argList(args, 4),
      });
    },
    onLeave(retval) {
      send({
        event: 'leave',
        hook_id: spec.id,
        abs_address: spec.abs,
        runtime_address: target.toString(),
        timestamp_ms: Date.now(),
        thread_id: Process.getCurrentThreadId(),
        retval: retval.toString(),
      });
    },
  });

  send({
    event: 'hook_installed',
    hook_id: spec.id,
    abs_address: spec.abs,
    runtime_address: target.toString(),
    timestamp_ms: Date.now(),
  });
}

send({
  event: 'session_start',
  module: MODULE_NAME,
  module_base: base.toString(),
  hook_count: HOOKS.length,
  timestamp_ms: Date.now(),
});

for (const spec of HOOKS) {
  try {
    installHook(spec);
  } catch (error) {
    send({
      event: 'hook_error',
      hook_id: spec.id,
      abs_address: spec.abs,
      error: String(error),
      timestamp_ms: Date.now(),
    });
  }
}
