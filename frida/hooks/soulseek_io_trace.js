"use strict";

const MODULE_NAME = "SoulseekQt";
const LINK_BASE = ptr("0x100000000");
const MAX_SAMPLE = 96;
const SEEN = new Set();
const moduleObj = Process.getModuleByName(MODULE_NAME);
const base = moduleObj.base;

function nowIso() {
  return new Date().toISOString();
}

function toInt(value) {
  try {
    return value.toInt32();
  } catch (_err) {
    const parsed = parseInt(value.toString(), 10);
    return Number.isFinite(parsed) ? parsed : 0;
  }
}

function runtimeAddress(absAddress) {
  return base.add(ptr(absAddress).sub(LINK_BASE));
}

function safeReadUtf8(ptrValue) {
  try {
    if (ptrValue.isNull()) {
      return "";
    }
    return Memory.readUtf8String(ptrValue) || "";
  } catch (_err) {
    return "";
  }
}

function bytesToHex(bytes) {
  if (bytes === null) {
    return "";
  }
  const view = new Uint8Array(bytes);
  return Array.prototype.map.call(view, function (b) {
    return ("0" + b.toString(16)).slice(-2);
  }).join("");
}

function safeHex(ptrValue, size) {
  try {
    if (ptrValue.isNull()) {
      return "";
    }
    const len = Math.min(Number(size) || 0, MAX_SAMPLE);
    if (len <= 0) {
      return "";
    }
    const bytes = Memory.readByteArray(ptrValue, len);
    return bytesToHex(bytes);
  } catch (_err) {
    return "";
  }
}

function emit(payload) {
  send({
    type: "io_event",
    ts: nowIso(),
    payload: payload,
  });
}

function isNullAddress(value) {
  if (value === null || value === undefined) {
    return true;
  }
  try {
    if (typeof value.isNull === "function") {
      return value.isNull();
    }
  } catch (_err) {
    // fall through
  }
  const asText = String(value);
  return asText === "0x0" || asText === "0";
}

function safeFindExportByName(moduleName, symbol) {
  try {
    if (typeof Module.findExportByName === "function") {
      return Module.findExportByName(moduleName, symbol);
    }
  } catch (_err) {
    // fall through
  }
  try {
    if (typeof Module.getExportByName === "function") {
      return Module.getExportByName(moduleName, symbol);
    }
  } catch (_err) {
    // fall through
  }
  return null;
}

function enumerateModuleSymbols(mod) {
  try {
    if (mod !== null && mod !== undefined && typeof mod.enumerateSymbols === "function") {
      return mod.enumerateSymbols();
    }
  } catch (_err) {
    // fall through
  }
  try {
    if (mod !== null && mod !== undefined && mod.name && typeof Module.enumerateSymbols === "function") {
      return Module.enumerateSymbols(mod.name);
    }
  } catch (_err) {
    // fall through
  }
  return [];
}

function attachHook(addr, meta, onEnter, onLeave) {
  if (addr === null || addr === undefined || isNullAddress(addr)) {
    return false;
  }
  const key = addr.toString();
  if (SEEN.has(key)) {
    return false;
  }
  SEEN.add(key);

  try {
    Interceptor.attach(addr, {
      onEnter(args) {
        this.__meta = meta;
        if (onEnter) {
          onEnter.call(this, args);
        }
      },
      onLeave(retval) {
        if (onLeave) {
          onLeave.call(this, retval);
        }
      },
    });
  } catch (error) {
    emit({
      event: "hook_error",
      target: meta.target,
      symbol: meta.symbol,
      module: meta.module,
      source: meta.source || "",
      address: key,
      error: String(error),
    });
    return false;
  }

  emit({
    event: "hook_registered",
    target: meta.target,
    symbol: meta.symbol,
    module: meta.module,
    source: meta.source || "",
    address: key,
  });
  return true;
}

function enumerateMatches(pattern) {
  try {
    const resolver = new ApiResolver("module");
    return resolver.enumerateMatches(pattern);
  } catch (_err) {
    return [];
  }
}

function importMatches(tokens) {
  const lowered = tokens.map(function (token) {
    return token.toLowerCase();
  });
  const imports = moduleObj.enumerateImports();
  const matches = [];
  imports.forEach(function (imp) {
    const name = (imp.name || "").toLowerCase();
    if (!name) {
      return;
    }
    if (lowered.every(function (token) { return name.indexOf(token) >= 0; })) {
      const address = imp.address || imp.slot || null;
      if (address !== null) {
        matches.push({
          address: address,
          name: imp.name || "",
          moduleName: imp.module || "import",
        });
      }
    }
  });
  return matches;
}

function findExport(name) {
  try {
    const direct = safeFindExportByName(null, name);
    if (!isNullAddress(direct)) {
      return direct;
    }
  } catch (_err) {
    // fall through
  }

  try {
    const modules = Process.enumerateModules();
    for (let i = 0; i < modules.length; i += 1) {
      const symbols = enumerateModuleSymbols(modules[i]);
      for (let j = 0; j < symbols.length; j += 1) {
        const candidate = symbols[j].address;
        if ((symbols[j].name || "") === name && !isNullAddress(candidate)) {
          return candidate;
        }
      }
    }
  } catch (_err) {
    return null;
  }
  return null;
}

function hookLibcPaths() {
  const libcSpecs = [
    { names: ["open", "_open"], event: "libc_open", pathArgIndex: 0 },
    { names: ["open$NOCANCEL", "_open$NOCANCEL"], event: "libc_open_nocancel", pathArgIndex: 0 },
    { names: ["openat", "_openat"], event: "libc_openat", pathArgIndex: 1 },
    { names: ["fopen", "_fopen"], event: "libc_fopen", pathArgIndex: 0 },
    { names: ["rename", "_rename"], event: "libc_rename", pathArgIndex: 0, pathArgIndex2: 1 },
    { names: ["unlink", "_unlink"], event: "libc_unlink", pathArgIndex: 0 },
  ];

  libcSpecs.forEach(function (spec) {
    let addr = null;
    let matchedName = "";
    spec.names.some(function (candidate) {
      const resolved = findExport(candidate);
      if (resolved) {
        addr = resolved;
        matchedName = candidate;
        return true;
      }
      return false;
    });
    if (!addr) {
      emit({
        event: "hook_missing",
        target: spec.event,
        names: spec.names,
      });
      return;
    }
    attachHook(
      addr,
      {
        target: spec.event,
        symbol: matchedName,
        module: "libc",
        source: "export",
      },
      function (args) {
        const payload = {
          event: spec.event,
          path: safeReadUtf8(args[spec.pathArgIndex || 0]),
        };
        if (spec.pathArgIndex2 !== undefined) {
          payload.path2 = safeReadUtf8(args[spec.pathArgIndex2]);
        }
        emit(payload);
      }
    );
  });
}

const DYNAMIC_SPECS = [
  {
    target: "qsettings_setvalue",
    importTokens: ["QSettings", "setValue"],
    patterns: ["exports:*!QSettings*setValue*"],
    onEnter(args) {
      emit({
        event: "qsettings_setvalue",
        qsettings_ptr: args[0].toString(),
        key_ptr: args[1].toString(),
        value_ptr: args[2].toString(),
      });
    },
  },
  {
    target: "qsettings_value",
    importTokens: ["QSettings", "value"],
    patterns: ["exports:*!QSettings*value*"],
    onEnter(args) {
      emit({
        event: "qsettings_value",
        qsettings_ptr: args[0].toString(),
        key_ptr: args[1].toString(),
      });
    },
  },
  {
    target: "qfile_open",
    importTokens: ["QFile", "open"],
    patterns: ["exports:*!QFile*open*"],
    onEnter(args) {
      emit({
        event: "qfile_open",
        qfile_ptr: args[0].toString(),
        mode_arg: args[1].toString(),
      });
    },
  },
  {
    target: "qdatastream_writebytes",
    importTokens: ["QDataStream", "writeBytes"],
    patterns: ["exports:*!QDataStream*writeBytes*"],
    onEnter(args) {
      const size = toInt(args[2]);
      emit({
        event: "qdatastream_writebytes",
        stream_ptr: args[0].toString(),
        size: size,
        sample_hex: safeHex(args[1], size),
      });
    },
  },
  {
    target: "qdatastream_readrawdata",
    importTokens: ["QDataStream", "readRawData"],
    patterns: ["exports:*!QDataStream*readRawData*"],
    onEnter(args) {
      this.buf = args[1];
      this.len = toInt(args[2]);
      emit({
        event: "qdatastream_readrawdata_enter",
        stream_ptr: args[0].toString(),
        requested: this.len,
      });
    },
    onLeave(retval) {
      const readLen = toInt(retval);
      emit({
        event: "qdatastream_readrawdata_leave",
        read_len: readLen,
        sample_hex: safeHex(this.buf, readLen),
      });
    },
  },
];

const SYMBOL_HOOKS = [
  {
    target: "qdatastream_writebytes_symbol",
    moduleToken: "qtcore",
    symbols: [
      "_ZN11QDataStream10writeBytesEPKcx",
      "__ZN11QDataStream10writeBytesEPKcx",
    ],
    onEnter(args) {
      const size = toInt(args[2]);
      emit({
        event: "qdatastream_writebytes",
        stream_ptr: args[0].toString(),
        size: size,
        sample_hex: safeHex(args[1], size),
      });
    },
  },
  {
    target: "qdatastream_readrawdata_symbol",
    moduleToken: "qtcore",
    symbols: [
      "_ZN11QDataStream11readRawDataEPcx",
      "__ZN11QDataStream11readRawDataEPcx",
    ],
    onEnter(args) {
      this.buf = args[1];
      this.len = toInt(args[2]);
      emit({
        event: "qdatastream_readrawdata_enter",
        stream_ptr: args[0].toString(),
        requested: this.len,
      });
    },
    onLeave(retval) {
      const readLen = toInt(retval);
      emit({
        event: "qdatastream_readrawdata_leave",
        read_len: readLen,
        sample_hex: safeHex(this.buf, readLen),
      });
    },
  },
  {
    target: "qfile_open_symbol",
    moduleToken: "qtcore",
    symbols: [
      "_ZN5QFile4openE6QFlagsIN13QIODeviceBase12OpenModeFlagEE",
      "__ZN5QFile4openE6QFlagsIN13QIODeviceBase12OpenModeFlagEE",
    ],
    onEnter(args) {
      emit({
        event: "qfile_open",
        qfile_ptr: args[0].toString(),
        mode_arg: args[1].toString(),
      });
    },
  },
  {
    target: "qsettings_setvalue_symbol",
    moduleToken: "qtcore",
    symbols: [
      "_ZN9QSettings8setValueE14QAnyStringViewRK8QVariant",
      "__ZN9QSettings8setValueE14QAnyStringViewRK8QVariant",
    ],
    onEnter(args) {
      emit({
        event: "qsettings_setvalue",
        qsettings_ptr: args[0].toString(),
        key_ptr: args[1].toString(),
        value_ptr: args[2].toString(),
      });
    },
  },
  {
    target: "qsettings_value_symbol",
    moduleToken: "qtcore",
    symbols: [
      "_ZNK9QSettings5valueE14QAnyStringView",
      "__ZNK9QSettings5valueE14QAnyStringView",
    ],
    onEnter(args) {
      emit({
        event: "qsettings_value",
        qsettings_ptr: args[0].toString(),
        key_ptr: args[1].toString(),
      });
    },
  },
  {
    target: "libc_fopen_symbol",
    moduleToken: "libsystem_c",
    symbols: ["fopen", "_fopen"],
    onEnter(args) {
      emit({
        event: "libc_fopen",
        path: safeReadUtf8(args[0]),
        mode: safeReadUtf8(args[1]),
      });
    },
  },
  {
    target: "libc_read_symbol",
    moduleToken: "libsystem_kernel",
    symbols: ["read", "_read"],
    onEnter(args) {
      this.buf = args[1];
      this.requested = toInt(args[2]);
      emit({
        event: "libc_read_enter",
        fd: toInt(args[0]),
        requested: this.requested,
      });
    },
    onLeave(retval) {
      const readLen = toInt(retval);
      emit({
        event: "libc_read_leave",
        read_len: readLen,
        sample_hex: safeHex(this.buf, readLen),
      });
    },
  },
  {
    target: "libc_write_symbol",
    moduleToken: "libsystem_kernel",
    symbols: ["write", "_write"],
    onEnter(args) {
      const size = toInt(args[2]);
      emit({
        event: "libc_write",
        fd: toInt(args[0]),
        size: size,
        sample_hex: safeHex(args[1], size),
      });
    },
  },
  {
    target: "libc_close_symbol",
    moduleToken: "libsystem_kernel",
    symbols: ["close", "_close"],
    onEnter(args) {
      emit({
        event: "libc_close",
        fd: toInt(args[0]),
      });
    },
  },
];

function findExportByModuleToken(moduleToken, symbol) {
  const token = moduleToken.toLowerCase();
  const modules = Process.enumerateModules();
  for (let i = 0; i < modules.length; i += 1) {
    const mod = modules[i];
    const haystack = ((mod.name || "") + " " + (mod.path || "")).toLowerCase();
    if (haystack.indexOf(token) < 0) {
      continue;
    }
    const exported = safeFindExportByName(mod.name, symbol);
    if (!isNullAddress(exported)) {
      return { address: exported, moduleName: mod.name, source: "module_export" };
    }
    try {
      const symbols = enumerateModuleSymbols(mod);
      for (let j = 0; j < symbols.length; j += 1) {
        const candidate = symbols[j].address;
        if ((symbols[j].name || "") === symbol && !isNullAddress(candidate)) {
          return { address: candidate, moduleName: mod.name, source: "module_symbol" };
        }
      }
    } catch (_err) {
      // Keep scanning candidate modules.
    }
  }
  return null;
}

function installNamedSymbolHooks() {
  SYMBOL_HOOKS.forEach(function (spec) {
    let resolved = null;
    let resolvedSymbol = "";
    for (let i = 0; i < spec.symbols.length; i += 1) {
      const symbol = spec.symbols[i];
      const candidate = findExportByModuleToken(spec.moduleToken, symbol);
      if (candidate !== null) {
        resolved = candidate;
        resolvedSymbol = symbol;
        break;
      }
    }

    if (resolved === null) {
      emit({
        event: "hook_missing",
        target: spec.target,
        module_token: spec.moduleToken,
        symbols: spec.symbols,
      });
      return;
    }

    attachHook(
      resolved.address,
      {
        target: spec.target,
        symbol: resolvedSymbol,
        module: resolved.moduleName,
        source: resolved.source || "module_export",
      },
      spec.onEnter,
      spec.onLeave
    );
  });
}

// Offsets validated with:
// nm -arch arm64 -nm /Applications/SoulseekQt.app/Contents/MacOS/SoulseekQt | c++filt
const ABSOLUTE_HOOKS = [
  {
    target: "mainwindow_init",
    abs: "0x10000dbd0",
    onEnter() {
      emit({ event: "mainwindow_init_enter" });
    },
  },
  {
    target: "writestring",
    abs: "0x1000acd90",
    onEnter(args) {
      emit({
        event: "writestring",
        qfile_ref: args[0].toString(),
        value_ptr: args[1].toString(),
      });
    },
  },
  {
    target: "readstring",
    abs: "0x1000ace40",
    onEnter(args) {
      emit({
        event: "readstring",
        qfile_ref: args[0].toString(),
        out_ptr: args[1].toString(),
      });
    },
  },
  {
    target: "qfilestreamer_readbuffer",
    abs: "0x10017e99c",
    onEnter(args) {
      this.buf = args[1];
      this.len = toInt(args[2]);
      emit({
        event: "qfilestreamer_readbuffer_enter",
        streamer_ptr: args[0].toString(),
        requested: this.len,
      });
    },
    onLeave(retval) {
      emit({
        event: "qfilestreamer_readbuffer_leave",
        retval: retval.toString(),
        sample_hex: safeHex(this.buf, this.len),
      });
    },
  },
  {
    target: "qfilestreamer_writebuffer",
    abs: "0x10017e9c8",
    onEnter(args) {
      const size = toInt(args[2]);
      emit({
        event: "qfilestreamer_writebuffer",
        streamer_ptr: args[0].toString(),
        size: size,
        sample_hex: safeHex(args[1], size),
      });
    },
  },
  {
    target: "mainwindow_on_connected",
    abs: "0x1000244cc",
    onEnter() {
      emit({ event: "mainwindow_on_connected_enter" });
    },
  },
  {
    target: "mainwindow_on_connecting",
    abs: "0x1000247f0",
    onEnter() {
      emit({ event: "mainwindow_on_connecting_enter" });
    },
  },
  {
    target: "mainwindow_on_disconnected",
    abs: "0x100024930",
    onEnter() {
      emit({ event: "mainwindow_on_disconnected_enter" });
    },
  },
  {
    target: "mainwindow_on_disconnect_clicked",
    abs: "0x100024afc",
    onEnter() {
      emit({ event: "mainwindow_on_disconnect_clicked_enter" });
    },
  },
  {
    target: "mainwindow_time_to_save_data",
    abs: "0x10002c40c",
    onEnter() {
      emit({ event: "mainwindow_time_to_save_data_enter" });
    },
  },
  {
    target: "mainwindow_on_save_data_every_value_changed",
    abs: "0x100035d20",
    onEnter(args) {
      emit({
        event: "mainwindow_on_save_data_every_value_changed_enter",
        value: args[1] ? args[1].toString() : "",
      });
    },
  },
  {
    target: "mainwindow_save_data",
    abs: "0x10002430c",
    onEnter() {
      emit({ event: "mainwindow_save_data_enter" });
    },
    onLeave(retval) {
      emit({ event: "mainwindow_save_data_leave", retval: retval.toString() });
    },
  },
  {
    target: "mainwindow_import_configuration_data",
    abs: "0x100022c74",
    onEnter() {
      emit({ event: "mainwindow_import_configuration_data_enter" });
    },
    onLeave(retval) {
      emit({ event: "mainwindow_import_configuration_data_leave", retval: retval.toString() });
    },
  },
  {
    target: "mainwindow_on_export_client_data_clicked",
    abs: "0x100035f38",
    onEnter() {
      emit({ event: "mainwindow_on_export_client_data_clicked_enter" });
    },
  },
  {
    target: "mainwindow_on_import_client_data_clicked",
    abs: "0x1000363c4",
    onEnter() {
      emit({ event: "mainwindow_on_import_client_data_clicked_enter" });
    },
  },
  {
    target: "mainwindow_on_import_user_list_clicked",
    abs: "0x10002c410",
    onEnter() {
      emit({ event: "mainwindow_on_import_user_list_clicked_enter" });
    },
  },
  {
    target: "mainwindow_on_clear_search_history_clicked",
    abs: "0x100032848",
    onEnter() {
      emit({ event: "mainwindow_on_clear_search_history_clicked_enter" });
    },
  },
  {
    target: "configuration_importer_import_user_list",
    abs: "0x10017e458",
    onEnter(args) {
      emit({
        event: "configuration_importer_import_user_list_enter",
        path_ptr: args[1] ? args[1].toString() : "",
      });
    },
  },
  {
    target: "datasaver_save",
    abs: "0x1000ad348",
    onEnter(args) {
      emit({ event: "datasaver_save_enter", path_ptr: args[1] ? args[1].toString() : "" });
    },
  },
  {
    target: "datasaver_save_to_file",
    abs: "0x1000ade74",
    onEnter(args) {
      emit({ event: "datasaver_save_to_file_enter", path_ptr: args[1] ? args[1].toString() : "" });
    },
  },
  {
    target: "datasaver_load",
    abs: "0x1000af12c",
    onEnter(args) {
      emit({ event: "datasaver_load_enter", path_ptr: args[1] ? args[1].toString() : "" });
    },
  },
  {
    target: "datasaver_load_from_file",
    abs: "0x1000af6f0",
    onEnter(args) {
      emit({
        event: "datasaver_load_from_file_enter",
        path_ptr: args[1] ? args[1].toString() : "",
        mode: args[2] ? args[2].toString() : "",
      });
    },
  },
  {
    target: "transfer_queue_on_data_loaded",
    abs: "0x1000e2fec",
    onEnter() {
      emit({ event: "transfer_queue_on_data_loaded_enter" });
    },
  },
  {
    target: "transfer_queue_transfers_loaded",
    abs: "0x1001fc144",
    onEnter() {
      emit({ event: "transfer_queue_transfers_loaded_enter" });
    },
  },
  {
    target: "transfer_queue_requeue_download",
    abs: "0x1000d6984",
    onEnter() {
      emit({ event: "transfer_queue_requeue_download_enter" });
    },
  },
];

function installDynamicHooks() {
  DYNAMIC_SPECS.forEach(function (spec) {
    let installed = 0;
    spec.patterns.forEach(function (pattern) {
      const matches = enumerateMatches(pattern);
      matches.forEach(function (match) {
        const ok = attachHook(
          match.address,
          {
            target: spec.target,
            symbol: match.name,
            module: match.moduleName,
            source: "pattern",
          },
          spec.onEnter,
          spec.onLeave
        );
        if (ok) {
          installed += 1;
        }
      });
    });

    const importHits = importMatches(spec.importTokens || []);
    importHits.forEach(function (match) {
      const ok = attachHook(
        match.address,
        {
          target: spec.target,
          symbol: match.name,
          module: match.moduleName,
          source: "import",
        },
        spec.onEnter,
        spec.onLeave
      );
      if (ok) {
        installed += 1;
      }
    });

    if (installed === 0) {
      emit({
        event: "hook_missing",
        target: spec.target,
        patterns: spec.patterns,
        import_tokens: spec.importTokens || [],
      });
    }
  });
}

function installAbsoluteHooks() {
  ABSOLUTE_HOOKS.forEach(function (spec) {
    const target = runtimeAddress(spec.abs);
    const ok = attachHook(
      target,
      {
        target: spec.target,
        symbol: spec.target,
        module: MODULE_NAME,
        source: "absolute",
      },
      spec.onEnter,
      spec.onLeave
    );
    if (!ok) {
      emit({
        event: "hook_missing",
        target: spec.target,
        abs: spec.abs,
      });
    }
  });
}

emit({
  event: "session_start",
  module: MODULE_NAME,
  module_base: base.toString(),
  pid: Process.id,
  timestamp: nowIso(),
});

hookLibcPaths();
installDynamicHooks();
installNamedSymbolHooks();
installAbsoluteHooks();
