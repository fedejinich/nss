from __future__ import annotations

import argparse
import csv
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

CORE_MESSAGES: list[tuple[str, str]] = [
    ("server", "SM_LOGIN"),
    ("server", "SM_SET_WAIT_PORT"),
    ("server", "SM_GET_PEER_ADDRESS"),
    ("server", "SM_CONNECT_TO_PEER"),
    ("server", "SM_FILE_SEARCH"),
    ("server", "SM_SEARCH_ROOM"),
    ("server", "SM_EXACT_FILE_SEARCH"),
    ("server", "SM_SEARCH_USER_FILES"),
    ("server", "SM_MESSAGE_USER"),
    ("server", "SM_MESSAGE_ACKED"),
    ("server", "SM_GET_USER_STATS"),
    ("server", "SM_GET_USER_STATUS"),
    ("server", "SM_SHARED_FOLDERS_FILES"),
    ("server", "SM_DOWNLOAD_SPEED"),
    ("server", "SM_UPLOAD_SPEED"),
    ("peer", "PM_GET_SHARED_FILE_LIST"),
    ("peer", "PM_SHARED_FILE_LIST"),
    ("peer", "PM_FILE_SEARCH_REQUEST"),
    ("peer", "PM_FILE_SEARCH_RESULT"),
    ("peer", "PM_TRANSFER_REQUEST"),
    ("peer", "PM_TRANSFER_RESPONSE"),
    ("peer", "PM_QUEUE_UPLOAD"),
    ("peer", "PM_UPLOAD_PLACE_IN_LINE"),
    ("peer", "PM_UPLOAD_FAILED"),
    ("peer", "PM_UPLOAD_DENIED"),
]

KNOWN_CODES: dict[tuple[str, str], int] = {
    ("server", "SM_LOGIN"): 1,
    ("server", "SM_SET_WAIT_PORT"): 2,
    ("server", "SM_GET_PEER_ADDRESS"): 3,
    ("server", "SM_GET_USER_STATUS"): 7,
    ("server", "SM_ADD_CHATROOM"): 10,
    ("server", "SM_IGNORE_USER"): 11,
    ("server", "SM_UNIGNORE_USER"): 12,
    ("server", "SM_SAY_CHATROOM"): 13,
    ("server", "SM_JOIN_ROOM"): 14,
    ("server", "SM_LEAVE_ROOM"): 15,
    ("server", "SM_USER_JOINED_ROOM"): 16,
    ("server", "SM_USER_LEFT_ROOM"): 17,
    ("server", "SM_CONNECT_TO_PEER"): 18,
    ("server", "SM_MESSAGE_USER"): 22,
    ("server", "SM_MESSAGE_ACKED"): 23,
    ("server", "SM_FILE_SEARCH"): 26,
    ("server", "SM_SET_STATUS"): 28,
    ("server", "SM_HEARTBEAT"): 32,
    ("server", "SM_ROOM_LIST"): 64,
    ("server", "SM_DOWNLOAD_SPEED"): 34,
    ("server", "SM_SHARED_FOLDERS_FILES"): 35,
    ("server", "SM_GET_USER_STATS"): 36,
    ("server", "SM_SEARCH_USER_FILES"): 42,
    ("server", "SM_GET_SIMILAR_TERMS"): 50,
    ("server", "SM_ADD_LIKE_TERM"): 51,
    ("server", "SM_REMOVE_LIKE_TERM"): 52,
    ("server", "SM_GET_RECOMMENDATIONS"): 54,
    ("server", "SM_GET_MY_RECOMMENDATIONS"): 55,
    ("server", "SM_GET_GLOBAL_RECOMMENDATIONS"): 56,
    ("server", "SM_GET_USER_RECOMMENDATIONS"): 57,
    ("server", "SM_COMMAND"): 58,
    ("server", "SM_ROOM_ADDED"): 62,
    ("server", "SM_ROOM_REMOVED"): 63,
    ("server", "SM_EXACT_FILE_SEARCH"): 65,
    ("server", "SM_ADMIN_MESSAGE"): 66,
    ("server", "SM_PEER_MESSAGE"): 68,
    ("server", "SM_GET_OWN_PRIVILEGES_STATUS"): 92,
    ("server", "SM_SET_PARENT_INACTIVITY_BEFORE_DISCONNECT"): 86,
    ("server", "SM_SET_SERVER_INACTIVITY_BEFORE_DISCONNECT"): 87,
    ("server", "SM_NODES_IN_CACHE_BEFORE_DISCONNECT"): 88,
    ("server", "SM_SET_SECONDS_BEFORE_PING_CHILDREN"): 90,
    ("server", "SM_CAN_PARENT"): 100,
    ("server", "SM_ADD_HATE_TERM"): 117,
    ("server", "SM_REMOVE_HATE_TERM"): 118,
    ("server", "SM_SEARCH_ROOM"): 120,
    ("server", "SM_GET_USER_PRIVILEGES_STATUS"): 122,
    ("server", "SM_GIVE_PRIVILEGE"): 123,
    ("server", "SM_INFORM_USER_OF_PRIVILEGES"): 124,
    ("server", "SM_INFORM_USER_OF_PRIVILEGES_ACK"): 125,
    ("server", "SM_DNET_LEVEL"): 126,
    ("server", "SM_DNET_GROUP_LEADER"): 127,
    ("server", "SM_DNET_DELIVERY_REPORT"): 128,
    ("server", "SM_DNET_CHILD_DEPTH"): 129,
    ("server", "SM_SET_PARENT_MIN_SPEED"): 83,
    ("server", "SM_SET_PARENT_SPEED_CONNECTION_RATIO"): 84,
    ("server", "SM_GET_ROOM_TICKER"): 113,
    ("server", "SM_UPLOAD_SPEED"): 121,
    ("server", "SM_FLOOD"): 131,
    ("server", "SM_ADD_ROOM_MEMBER"): 134,
    ("server", "SM_REMOVE_ROOM_MEMBER"): 135,
    ("server", "SM_REMOVE_OWN_ROOM_MEMBERSHIP"): 136,
    ("server", "SM_GIVE_UP_ROOM"): 137,
    ("server", "SM_ADD_ROOM_MEMBERSHIP"): 139,
    ("server", "SM_REMOVE_ROOM_MEMBERSHIP"): 140,
    ("server", "SM_ADD_ROOM_OPERATOR"): 143,
    ("server", "SM_REMOVE_ROOM_OPERATOR"): 144,
    ("server", "SM_ADD_ROOM_OPERATORSHIP"): 145,
    ("server", "SM_REMOVE_ROOM_OPERATORSHIP"): 146,
    ("server", "SM_REMOVE_OWN_ROOM_OPERATORSHIP"): 147,
    ("server", "SM_ROOM_MEMBERS"): 133,
    ("server", "SM_ROOM_OPERATORS"): 148,
    ("server", "SM_MESSAGE_USERS"): 149,
    ("server", "SM_JOIN_GLOBAL_ROOM"): 150,
    ("server", "SM_LEAVE_GLOBAL_ROOM"): 151,
    ("server", "SM_SAY_GLOBAL_ROOM"): 152,
    ("server", "SM_SEARCH_CORRELATIONS"): 153,
    ("server", "SM_DNET_RESET"): 130,
    ("peer", "PM_GET_SHARED_FILE_LIST"): 4,
    ("peer", "PM_SHARED_FILE_LIST"): 5,
    ("peer", "PM_FILE_SEARCH_REQUEST"): 8,
    ("peer", "PM_FILE_SEARCH_RESULT"): 9,
    ("peer", "PM_USER_INFO_REQUEST"): 15,
    ("peer", "PM_USER_INFO_REPLY"): 16,
    ("peer", "PM_GET_SHARED_FILES_IN_FOLDER"): 36,
    ("peer", "PM_SHARED_FILES_IN_FOLDER"): 37,
    ("peer", "PM_TRANSFER_REQUEST"): 40,
    ("peer", "PM_TRANSFER_RESPONSE"): 41,
    ("peer", "PM_QUEUE_UPLOAD"): 43,
    ("peer", "PM_UPLOAD_PLACE_IN_LINE"): 44,
    ("peer", "PM_EXACT_FILE_SEARCH_REQUEST"): 47,
    ("peer", "PM_INDIRECT_FILE_SEARCH_REQUEST"): 49,
    ("peer", "PM_UPLOAD_FAILED"): 46,
    ("peer", "PM_UPLOAD_DENIED"): 50,
    ("peer", "PM_UPLOAD_PLACE_IN_LINE_REQUEST"): 51,
}

KNOWN_PAYLOADS: dict[tuple[str, str], list[dict[str, str]]] = {
    ("server", "SM_LOGIN"): [
        {"name": "username", "type": "string"},
        {"name": "password", "type": "string"},
        {"name": "client_version", "type": "u32"},
        {"name": "md5hash", "type": "string"},
        {"name": "minor_version", "type": "u32"},
    ],
    ("server", "SM_SET_WAIT_PORT"): [{"name": "listen_port", "type": "u32"}],
    ("server", "SM_GET_PEER_ADDRESS"): [
        {"name": "username", "type": "string"},
        {"name": "ip_address", "type": "ipv4_u32_le"},
        {"name": "port", "type": "u32"},
        {"name": "obfuscation_type", "type": "u32"},
        {"name": "obfuscated_port", "type": "u16"},
    ],
    ("server", "SM_IGNORE_USER"): [{"name": "username", "type": "string"}],
    ("server", "SM_UNIGNORE_USER"): [{"name": "username", "type": "string"}],
    ("server", "SM_ADD_CHATROOM"): [{"name": "room", "type": "string"}],
    ("server", "SM_SAY_CHATROOM"): [
        {"name": "room", "type": "string"},
        {"name": "username", "type": "optional_string"},
        {"name": "message", "type": "string"},
    ],
    ("server", "SM_JOIN_ROOM"): [
        {"name": "room", "type": "string"},
        {"name": "users", "type": "array<string>"},
    ],
    ("server", "SM_LEAVE_ROOM"): [{"name": "room", "type": "string"}],
    ("server", "SM_USER_JOINED_ROOM"): [
        {"name": "room", "type": "string"},
        {"name": "username", "type": "string"},
    ],
    ("server", "SM_USER_LEFT_ROOM"): [
        {"name": "room", "type": "string"},
        {"name": "username", "type": "string"},
    ],
    ("server", "SM_CONNECT_TO_PEER"): [
        {"name": "token", "type": "u32"},
        {"name": "username", "type": "string"},
        {"name": "connection_type", "type": "string"},
        {"name": "ip_address", "type": "ipv4_u32_le"},
        {"name": "port", "type": "u32"},
        {"name": "privileged", "type": "bool_u32"},
        {"name": "obfuscation_type", "type": "u32"},
        {"name": "obfuscated_port", "type": "u32"},
    ],
    ("server", "SM_RELOGGED"): [],
    ("server", "SM_USER_LIST"): [
        {"name": "user_count", "type": "u32"},
        {"name": "entry.username", "type": "string"},
        {"name": "status_count", "type": "u32"},
        {"name": "entry.status", "type": "u32"},
        {"name": "stats_count", "type": "u32"},
        {"name": "entry.avg_speed", "type": "u32"},
        {"name": "entry.upload_num", "type": "u32"},
        {"name": "entry.unknown", "type": "u32"},
        {"name": "entry.files", "type": "u32"},
        {"name": "entry.dirs", "type": "u32"},
        {"name": "slots_count", "type": "u32"},
        {"name": "entry.slots_full", "type": "u32"},
        {"name": "country_count", "type": "u32"},
        {"name": "entry.country", "type": "string"},
    ],
    ("server", "SM_GLOBAL_USER_LIST"): [
        {"name": "user_count", "type": "u32"},
        {"name": "entry.username", "type": "string"},
        {"name": "status_count", "type": "u32"},
        {"name": "entry.status", "type": "u32"},
        {"name": "stats_count", "type": "u32"},
        {"name": "entry.avg_speed", "type": "u32"},
        {"name": "entry.upload_num", "type": "u32"},
        {"name": "entry.unknown", "type": "u32"},
        {"name": "entry.files", "type": "u32"},
        {"name": "entry.dirs", "type": "u32"},
        {"name": "slots_count", "type": "u32"},
        {"name": "entry.slots_full", "type": "u32"},
        {"name": "country_count", "type": "u32"},
        {"name": "entry.country", "type": "string"},
    ],
    ("server", "SM_CONNECT_TO_CLIENT"): [
        {"name": "token", "type": "u32"},
        {"name": "username", "type": "string"},
        {"name": "connection_type", "type": "string"},
        {"name": "extension_reserved_bytes", "type": "bytes_raw"},
    ],
    ("server", "SM_SEND_DISTRIBUTIONS"): [{"name": "no_parent", "type": "bool_u8"}],
    ("server", "SM_NOTE_PARENT"): [{"name": "parent_ip", "type": "ipv4_u32_reversed"}],
    ("server", "SM_CHILD_PARENT_MAP"): [
        {"name": "entry_count", "type": "u32"},
        {"name": "entry.child_username", "type": "string"},
        {"name": "entry.parent_username", "type": "string"},
        {"name": "extension_reserved_bytes", "type": "bytes_raw"},
    ],
    ("server", "SM_DNET_MESSAGE"): [
        {"name": "distrib_code", "type": "u8"},
        {"name": "distrib_payload", "type": "bytes_raw"},
    ],
    ("server", "SM_POSSIBLE_PARENTS"): [
        {"name": "parent_count", "type": "u32"},
        {"name": "entry.username", "type": "string"},
        {"name": "entry.ip_address", "type": "ipv4_u32_reversed"},
        {"name": "entry.port", "type": "u32"},
    ],
    ("server", "SM_ROOM_TICKER_USER_ADDED"): [
        {"name": "room", "type": "string"},
        {"name": "username", "type": "string"},
        {"name": "ticker", "type": "string"},
    ],
    ("server", "SM_ROOM_TICKER_USER_REMOVED"): [
        {"name": "room", "type": "string"},
        {"name": "username", "type": "string"},
    ],
    ("server", "SM_SET_TICKER"): [
        {"name": "room", "type": "string"},
        {"name": "ticker", "type": "string"},
    ],
    ("server", "SM_TRANSFER_ROOM_OWNERSHIP"): [{"name": "room", "type": "string"}],
    ("server", "SM_ENABLE_PRIVATE_ROOM_ADD"): [{"name": "enabled", "type": "bool_u8"}],
    ("server", "SM_CHANGE_PASSWORD"): [{"name": "password", "type": "string"}],
    ("server", "SM_ROOM_LIST"): [
        {"name": "room_count", "type": "u32"},
        {"name": "rooms", "type": "array<string>"},
    ],
    ("server", "SM_FILE_SEARCH"): [
        {"name": "search_token", "type": "u32"},
        {"name": "search_text", "type": "string"},
    ],
    ("server", "SM_SEARCH_ROOM"): [
        {"name": "room", "type": "string"},
        {"name": "search_text", "type": "string"},
    ],
    ("server", "SM_EXACT_FILE_SEARCH"): [{"name": "virtual_path", "type": "string"}],
    ("server", "SM_SEARCH_USER_FILES"): [
        {"name": "username", "type": "string"},
        {"name": "search_text", "type": "string"},
    ],
    ("server", "SM_GET_SIMILAR_TERMS"): [
        {"name": "term", "type": "string"},
        {"name": "recommendation_count", "type": "u32"},
        {"name": "recommendation.term", "type": "string"},
        {"name": "recommendation.score", "type": "i32"},
    ],
    ("server", "SM_ADD_LIKE_TERM"): [{"name": "term", "type": "string"}],
    ("server", "SM_REMOVE_LIKE_TERM"): [{"name": "term", "type": "string"}],
    ("server", "SM_GET_RECOMMENDATIONS"): [
        {"name": "recommendation_count", "type": "u32"},
        {"name": "recommendation.term", "type": "string"},
        {"name": "recommendation.score", "type": "i32"},
        {"name": "unrecommendation_count", "type": "u32"},
        {"name": "unrecommendation.term", "type": "string"},
        {"name": "unrecommendation.score", "type": "i32"},
    ],
    ("server", "SM_GET_MY_RECOMMENDATIONS"): [
        {"name": "recommendation_count", "type": "u32"},
        {"name": "recommendation.term", "type": "string"},
        {"name": "recommendation.score", "type": "i32"},
        {"name": "unrecommendation_count", "type": "u32"},
        {"name": "unrecommendation.term", "type": "string"},
        {"name": "unrecommendation.score", "type": "i32"},
    ],
    ("server", "SM_GET_GLOBAL_RECOMMENDATIONS"): [
        {"name": "recommendation_count", "type": "u32"},
        {"name": "recommendation.term", "type": "string"},
        {"name": "recommendation.score", "type": "i32"},
        {"name": "unrecommendation_count", "type": "u32"},
        {"name": "unrecommendation.term", "type": "string"},
        {"name": "unrecommendation.score", "type": "i32"},
    ],
    ("server", "SM_GET_OWN_PRIVILEGES_STATUS"): [
        {"name": "time_left_seconds", "type": "u32"},
    ],
    ("server", "SM_GET_USER_PRIVILEGES_STATUS"): [
        {"name": "username", "type": "string"},
        {"name": "privileged", "type": "bool_u32"},
    ],
    ("server", "SM_GIVE_PRIVILEGE"): [
        {"name": "username", "type": "string"},
        {"name": "days", "type": "u32"},
    ],
    ("server", "SM_INFORM_USER_OF_PRIVILEGES"): [
        {"name": "token", "type": "u32"},
        {"name": "username", "type": "string"},
    ],
    ("server", "SM_INFORM_USER_OF_PRIVILEGES_ACK"): [
        {"name": "token", "type": "u32"},
    ],
    ("server", "SM_GET_USER_RECOMMENDATIONS"): [
        {"name": "username", "type": "string"},
        {"name": "recommendation_count", "type": "u32"},
        {"name": "recommendation.term", "type": "string"},
        {"name": "recommendation.score", "type": "i32"},
        {"name": "unrecommendation_count", "type": "u32"},
        {"name": "unrecommendation.term", "type": "string"},
        {"name": "unrecommendation.score", "type": "i32"},
    ],
    ("server", "SM_ADD_ROOM_MEMBER"): [
        {"name": "room", "type": "string"},
        {"name": "username", "type": "string"},
    ],
    ("server", "SM_REMOVE_ROOM_MEMBER"): [
        {"name": "room", "type": "string"},
        {"name": "username", "type": "string"},
    ],
    ("server", "SM_ADD_ROOM_OPERATOR"): [
        {"name": "room", "type": "string"},
        {"name": "username", "type": "string"},
    ],
    ("server", "SM_REMOVE_ROOM_OPERATOR"): [
        {"name": "room", "type": "string"},
        {"name": "username", "type": "string"},
    ],
    ("server", "SM_ROOM_MEMBERS"): [
        {"name": "room", "type": "string"},
        {"name": "users", "type": "array<string>"},
    ],
    ("server", "SM_ROOM_OPERATORS"): [
        {"name": "room", "type": "string"},
        {"name": "operators", "type": "array<string>"},
    ],
    ("server", "SM_MESSAGE_USER"): [
        {"name": "message_id", "type": "u32"},
        {"name": "timestamp", "type": "u32"},
        {"name": "username", "type": "string"},
        {"name": "message", "type": "string"},
        {"name": "is_new", "type": "bool_u8"},
    ],
    ("server", "SM_MESSAGE_ACKED"): [{"name": "message_id", "type": "u32"}],
    ("server", "SM_MESSAGE_USERS"): [
        {"name": "username_count", "type": "u32"},
        {"name": "usernames", "type": "array<string>"},
        {"name": "message", "type": "string"},
    ],
    ("server", "SM_PEER_MESSAGE"): [
        {"name": "username", "type": "string"},
        {"name": "token", "type": "u32"},
        {"name": "code", "type": "u32"},
        {"name": "ip_address", "type": "ipv4_u32_le"},
        {"name": "port", "type": "u32"},
        {"name": "message", "type": "string"},
    ],
    ("server", "SM_GET_USER_STATS"): [
        {"name": "username", "type": "string"},
        {"name": "avg_speed", "type": "u32"},
        {"name": "download_num", "type": "u32"},
        {"name": "files", "type": "u32"},
        {"name": "dirs", "type": "u32"},
    ],
    ("server", "SM_GET_USER_STATUS"): [
        {"name": "username", "type": "string"},
        {"name": "status", "type": "u32"},
        {"name": "privileged", "type": "bool_u32"},
    ],
    ("server", "SM_SHARED_FOLDERS_FILES"): [
        {"name": "folder_count", "type": "u32"},
        {"name": "file_count", "type": "u32"},
    ],
    ("server", "SM_DOWNLOAD_SPEED"): [{"name": "bytes_per_sec", "type": "u32"}],
    ("server", "SM_SET_STATUS"): [{"name": "status", "type": "u32"}],
    ("server", "SM_HEARTBEAT"): [{"name": "sequence", "type": "optional_u32"}],
    ("server", "SM_SET_PARENT_MIN_SPEED"): [{"name": "min_speed", "type": "u32"}],
    ("server", "SM_SET_PARENT_SPEED_CONNECTION_RATIO"): [{"name": "ratio", "type": "u32"}],
    ("server", "SM_SET_PARENT_INACTIVITY_BEFORE_DISCONNECT"): [
        {"name": "seconds", "type": "u32"}
    ],
    ("server", "SM_SET_SERVER_INACTIVITY_BEFORE_DISCONNECT"): [
        {"name": "seconds", "type": "u32"}
    ],
    ("server", "SM_NODES_IN_CACHE_BEFORE_DISCONNECT"): [
        {"name": "nodes", "type": "u32"}
    ],
    ("server", "SM_SET_SECONDS_BEFORE_PING_CHILDREN"): [
        {"name": "seconds", "type": "u32"}
    ],
    ("server", "SM_CAN_PARENT"): [{"name": "can_parent", "type": "bool_u32"}],
    ("server", "SM_COMMAND"): [{"name": "command", "type": "string"}],
    ("server", "SM_ROOM_ADDED"): [{"name": "room", "type": "string"}],
    ("server", "SM_ROOM_REMOVED"): [{"name": "room", "type": "string"}],
    ("server", "SM_ADMIN_MESSAGE"): [{"name": "message", "type": "string"}],
    ("server", "SM_ADD_HATE_TERM"): [{"name": "term", "type": "string"}],
    ("server", "SM_REMOVE_HATE_TERM"): [{"name": "term", "type": "string"}],
    ("server", "SM_GET_ROOM_TICKER"): [
        {"name": "room", "type": "string"},
        {"name": "ticker_count", "type": "u32"},
        {"name": "entry.username", "type": "string"},
        {"name": "entry.ticker", "type": "string"},
    ],
    ("server", "SM_DNET_RESET"): [{"name": "reason", "type": "optional_u32"}],
    ("server", "SM_DNET_LEVEL"): [
        {"name": "level", "type": "optional_u32"},
        {"name": "extension_reserved_bytes", "type": "bytes_raw"},
    ],
    ("server", "SM_DNET_GROUP_LEADER"): [
        {"name": "username", "type": "optional_string"},
        {"name": "extension_reserved_bytes", "type": "bytes_raw"},
    ],
    ("server", "SM_DNET_DELIVERY_REPORT"): [
        {"name": "report", "type": "optional_u32"},
        {"name": "extension_reserved_bytes", "type": "bytes_raw"},
    ],
    ("server", "SM_DNET_CHILD_DEPTH"): [
        {"name": "depth", "type": "optional_u32"},
        {"name": "extension_reserved_bytes", "type": "bytes_raw"},
    ],
    ("server", "SM_FLOOD"): [
        {"name": "flood_code", "type": "optional_u32"},
        {"name": "extension_reserved_bytes", "type": "bytes_raw"},
    ],
    ("server", "SM_UPLOAD_SPEED"): [{"name": "bytes_per_sec", "type": "u32"}],
    ("server", "SM_REMOVE_OWN_ROOM_MEMBERSHIP"): [{"name": "room", "type": "string"}],
    ("server", "SM_GIVE_UP_ROOM"): [{"name": "room", "type": "string"}],
    ("server", "SM_ADD_ROOM_MEMBERSHIP"): [{"name": "room", "type": "string"}],
    ("server", "SM_REMOVE_ROOM_MEMBERSHIP"): [{"name": "room", "type": "string"}],
    ("server", "SM_ADD_ROOM_OPERATORSHIP"): [{"name": "room", "type": "string"}],
    ("server", "SM_REMOVE_ROOM_OPERATORSHIP"): [
        {"name": "room", "type": "optional_string"},
        {"name": "extension_reserved_bytes", "type": "bytes_raw"},
    ],
    ("server", "SM_REMOVE_OWN_ROOM_OPERATORSHIP"): [
        {"name": "room", "type": "optional_string"},
        {"name": "extension_reserved_bytes", "type": "bytes_raw"},
    ],
    ("server", "SM_JOIN_GLOBAL_ROOM"): [{"name": "room", "type": "optional_string"}],
    ("server", "SM_LEAVE_GLOBAL_ROOM"): [{"name": "room", "type": "optional_string"}],
    ("server", "SM_SAY_GLOBAL_ROOM"): [{"name": "message", "type": "string"}],
    ("server", "SM_SEARCH_CORRELATIONS"): [{"name": "term", "type": "string"}],
    ("peer", "PM_GET_SHARED_FILE_LIST"): [{"name": "username", "type": "string"}],
    ("peer", "PM_SHARED_FILE_LIST"): [
        {"name": "entries", "type": "array<shared_file_entry>"},
        {"name": "entry.virtual_path", "type": "string"},
        {"name": "entry.size", "type": "u64"},
    ],
    ("peer", "PM_GET_SHARED_FILES_IN_FOLDER"): [
        {"name": "directory", "type": "string"},
    ],
    ("peer", "PM_SHARED_FILES_IN_FOLDER"): [
        {"name": "directory", "type": "string"},
        {"name": "compressed_listing", "type": "bytes_raw"},
        {"name": "decompressed_listing", "type": "bytes_raw"},
        {"name": "listing_format", "type": "enum"},
        {"name": "entry.virtual_path", "type": "string"},
        {"name": "entry.size", "type": "u64"},
    ],
    ("peer", "PM_FILE_SEARCH_REQUEST"): [
        {"name": "token", "type": "u32"},
        {"name": "query", "type": "string"},
    ],
    ("peer", "PM_FILE_SEARCH_RESULT"): [
        {"name": "token", "type": "u32"},
        {"name": "username", "type": "string"},
        {"name": "result_count", "type": "u32"},
    ],
    ("peer", "PM_USER_INFO_REQUEST"): [],
    ("peer", "PM_USER_INFO_REPLY"): [
        {"name": "description", "type": "string"},
        {"name": "has_picture", "type": "bool_u8"},
        {"name": "picture", "type": "bytes_len_prefixed"},
        {"name": "total_uploads", "type": "u32"},
        {"name": "queue_size", "type": "u32"},
        {"name": "slots_free", "type": "bool_u8"},
        {"name": "upload_permissions", "type": "optional_u32"},
    ],
    ("peer", "PM_TRANSFER_REQUEST"): [
        {"name": "direction", "type": "enum_u32"},
        {"name": "token", "type": "u32"},
        {"name": "virtual_path", "type": "string"},
        {"name": "file_size", "type": "u64"},
    ],
    ("peer", "PM_TRANSFER_RESPONSE"): [
        {"name": "token", "type": "u32"},
        {"name": "allowed", "type": "bool_u32"},
        {"name": "queue_or_reason", "type": "string"},
    ],
    ("peer", "PM_QUEUE_UPLOAD"): [
        {"name": "username", "type": "string"},
        {"name": "virtual_path", "type": "string"},
    ],
    ("peer", "PM_UPLOAD_PLACE_IN_LINE"): [
        {"name": "username", "type": "string"},
        {"name": "virtual_path", "type": "string"},
        {"name": "place", "type": "u32"},
    ],
    ("peer", "PM_UPLOAD_FAILED"): [
        {"name": "username", "type": "string"},
        {"name": "virtual_path", "type": "string"},
        {"name": "reason", "type": "string"},
    ],
    ("peer", "PM_UPLOAD_DENIED"): [
        {"name": "username", "type": "string"},
        {"name": "virtual_path", "type": "string"},
        {"name": "reason", "type": "string"},
    ],
    ("peer", "PM_EXACT_FILE_SEARCH_REQUEST"): [
        {"name": "token", "type": "optional_u32"},
        {"name": "query", "type": "string"},
    ],
    ("peer", "PM_INDIRECT_FILE_SEARCH_REQUEST"): [
        {"name": "token", "type": "optional_u32"},
        {"name": "query", "type": "string"},
    ],
    ("peer", "PM_UPLOAD_PLACE_IN_LINE_REQUEST"): [
        {"name": "virtual_path", "type": "string"},
    ],
}

EXTRA_EVIDENCE: dict[tuple[str, str], list[dict[str, str]]] = {
    ("server", "SM_FILE_SEARCH"): [
        {
            "kind": "ghidra_decompile",
            "source": "evidence/reverse/disasm/server_file_search.txt",
            "note": "Function writes constant 0x1a before serializing search payload.",
        },
        {
            "kind": "ghidra_decompile",
            "source": "evidence/reverse/disasm/server_prepare_search.txt",
            "note": "PrepareSearch normalizes and emits search tokens/strings.",
        },
    ],
    ("server", "SM_GET_PEER_ADDRESS"): [
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 3 defines peer-address request and response payload fields.",
        }
    ],
    ("server", "SM_CONNECT_TO_PEER"): [
        {
            "kind": "ghidra_decompile",
            "source": "evidence/reverse/disasm/server_handle_message.txt",
            "note": "Server handler routes peer connect responses to transfer subsystem.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 18 documents ConnectToPeer request and response payload shape.",
        }
    ],
    ("server", "SM_RELOGGED"): [
        {
            "kind": "spec",
            "source": "https://raw.githubusercontent.com/nicotine-plus/nicotine-plus/master/pynicotine/slskmessages.py",
            "note": "Nicotine+ class Relogged (code 41) documents empty payload semantics.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-s6-batch1-control/official_frames.hex",
            "note": "S6 batch-1 probe includes relogged control code frame for typed decode coverage.",
        },
    ],
    ("server", "SM_USER_LIST"): [
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-s6-batch1-control/official_frames.hex",
            "note": "S6 batch-1 authenticated probe includes code 61 frame with typed user-list schema layout.",
        },
    ],
    ("server", "SM_GLOBAL_USER_LIST"): [
        {
            "kind": "spec",
            "source": "https://raw.githubusercontent.com/nicotine-plus/nicotine-plus/master/pynicotine/slskmessages.py",
            "note": "Nicotine+ class GlobalUserList (code 67) parses UsersMessage list payload layout.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-s6-batch1-control/official_frames.hex",
            "note": "S6 batch-1 authenticated probe includes code 67 request/response activity.",
        },
    ],
    ("server", "SM_CONNECT_TO_CLIENT"): [
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-s6-batch1-control/official_frames.hex",
            "note": "S6 batch-1 authenticated probe includes code 70 frame with token+username+connection_type payload shape.",
        },
    ],
    ("server", "SM_SEND_DISTRIBUTIONS"): [
        {
            "kind": "spec",
            "source": "https://raw.githubusercontent.com/nicotine-plus/nicotine-plus/master/pynicotine/slskmessages.py",
            "note": "Nicotine+ class HaveNoParent (code 71) serializes one bool payload flag.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-s6-batch2-control/official_frames.hex",
            "note": "S6 batch-2 authenticated probe includes code 71 bool payload.",
        },
    ],
    ("server", "SM_NOTE_PARENT"): [
        {
            "kind": "spec",
            "source": "https://raw.githubusercontent.com/nicotine-plus/nicotine-plus/master/pynicotine/slskmessages.py",
            "note": "Nicotine+ class ParentIP (code 73) uses reversed IPv4 u32 payload representation.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-s6-batch2-control/official_frames.hex",
            "note": "S6 batch-2 authenticated probe includes code 73 payload with reversed IPv4 encoding.",
        },
    ],
    ("server", "SM_CHILD_PARENT_MAP"): [
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-s6-batch2-control/official_frames.hex",
            "note": "S6 batch-2 authenticated probe includes code 82 payload with typed child->parent pair entries and raw-tail compatibility.",
        },
    ],
    ("server", "SM_DNET_MESSAGE"): [
        {
            "kind": "spec",
            "source": "https://raw.githubusercontent.com/nicotine-plus/nicotine-plus/master/pynicotine/slskmessages.py",
            "note": "Nicotine+ class EmbeddedMessage (code 93) parses u8 distributed code followed by embedded payload bytes.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-s6-batch2-control/official_frames.hex",
            "note": "S6 batch-2 authenticated probe includes code 93 payload with typed distributed code and bytes.",
        },
    ],
    ("server", "SM_DNET_LEVEL"): [
        {
            "kind": "spec",
            "source": "https://raw.githubusercontent.com/nicotine-plus/nicotine-plus/master/pynicotine/slskmessages.py",
            "note": "Nicotine+ class BranchLevel (code 126) serializes one u32 branch-level value.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-legacy-distributed-control/official_frames.hex",
            "note": "S6E authenticated runtime probe includes code 126 frame with u32 payload and no residual tail.",
        },
    ],
    ("server", "SM_DNET_GROUP_LEADER"): [
        {
            "kind": "spec",
            "source": "https://raw.githubusercontent.com/nicotine-plus/nicotine-plus/master/pynicotine/slskmessages.py",
            "note": "Nicotine+ class BranchRoot (code 127) serializes one root-username string.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-legacy-distributed-control/official_frames.hex",
            "note": "S6E authenticated runtime probe includes code 127 frame with root-username string payload.",
        },
    ],
    ("server", "SM_DNET_DELIVERY_REPORT"): [
        {
            "kind": "static_analysis",
            "source": "evidence/ui_audit/decomp/server_methods.txt",
            "note": "Server::DNetDeliveryReport(int) symbol signature indicates a single int payload argument on the server handler path.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-legacy-distributed-control/official_frames.hex",
            "note": "S6E authenticated runtime probe confirms active code 128 wire path with 4-byte payload.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-legacy-residual-control/official_frames.hex",
            "note": "S6F authenticated runtime probe exercises code 128 with multi-value u32 payload variants (0,1,2), confirming typed optional_u32 + extension-reserved bytes layout.",
        },
    ],
    ("server", "SM_DNET_CHILD_DEPTH"): [
        {
            "kind": "spec",
            "source": "https://raw.githubusercontent.com/nicotine-plus/nicotine-plus/master/pynicotine/slskmessages.py",
            "note": "Nicotine+ class ChildDepth (code 129) serializes one u32 depth value.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-legacy-distributed-control/official_frames.hex",
            "note": "S6E authenticated runtime probe includes code 129 frame with u32 depth payload.",
        },
    ],
    ("server", "SM_FLOOD"): [
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-legacy-distributed-control/official_frames.hex",
            "note": "S6E authenticated runtime probe confirms active code 131 wire path with 4-byte payload.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-legacy-residual-control/official_frames.hex",
            "note": "S6F authenticated runtime probe exercises code 131 with multi-value u32 payload variants (0,1,2), promoting typed optional_u32 + extension-reserved bytes handling.",
        },
    ],
    ("server", "SM_POSSIBLE_PARENTS"): [
        {
            "kind": "spec",
            "source": "https://raw.githubusercontent.com/nicotine-plus/nicotine-plus/master/pynicotine/slskmessages.py",
            "note": "Nicotine+ class PossibleParents (code 102) parses username + ip + port candidate tuples.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-s6-batch2-control/official_frames.hex",
            "note": "S6 batch-2 authenticated probe includes code 102 candidate list payload shape.",
        },
    ],
    ("server", "SM_ROOM_TICKER_USER_ADDED"): [
        {
            "kind": "spec",
            "source": "https://raw.githubusercontent.com/nicotine-plus/nicotine-plus/master/pynicotine/slskmessages.py",
            "note": "Nicotine+ class RoomTickerAdded (code 114) parses room+user+ticker strings.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-s6-batch3-control/official_frames.hex",
            "note": "S6 batch-3 probe includes code 114 payload for typed ticker-added decode.",
        },
    ],
    ("server", "SM_ROOM_TICKER_USER_REMOVED"): [
        {
            "kind": "spec",
            "source": "https://raw.githubusercontent.com/nicotine-plus/nicotine-plus/master/pynicotine/slskmessages.py",
            "note": "Nicotine+ class RoomTickerRemoved (code 115) parses room+user strings.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-s6-batch3-control/official_frames.hex",
            "note": "S6 batch-3 probe includes attempted code 115 outbound payload while server resets were observed.",
        },
    ],
    ("server", "SM_SET_TICKER"): [
        {
            "kind": "spec",
            "source": "https://raw.githubusercontent.com/nicotine-plus/nicotine-plus/master/pynicotine/slskmessages.py",
            "note": "Nicotine+ class SetRoomTicker (code 116) serializes room+ticker strings.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-s6-batch3-control/official_frames.hex",
            "note": "S6 batch-3 probe includes attempted code 116 outbound payload while server resets were observed.",
        },
    ],
    ("server", "SM_TRANSFER_ROOM_OWNERSHIP"): [
        {
            "kind": "spec",
            "source": "https://raw.githubusercontent.com/nicotine-plus/nicotine-plus/master/pynicotine/slskmessages.py",
            "note": "Nicotine+ class RoomSomething (code 138, obsolete) uses room string payload.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-s6-batch3-control/official_frames.hex",
            "note": "S6 batch-3 probe includes attempted code 138 outbound payload while server resets were observed.",
        },
    ],
    ("server", "SM_ENABLE_PRIVATE_ROOM_ADD"): [
        {
            "kind": "spec",
            "source": "https://raw.githubusercontent.com/nicotine-plus/nicotine-plus/master/pynicotine/slskmessages.py",
            "note": "Nicotine+ class EnableRoomInvitations (code 141) serializes a bool flag payload.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-s6-batch3-control/official_frames.hex",
            "note": "S6 batch-3 probe includes attempted code 141 outbound payload while server resets were observed.",
        },
    ],
    ("server", "SM_CHANGE_PASSWORD"): [
        {
            "kind": "spec",
            "source": "https://raw.githubusercontent.com/nicotine-plus/nicotine-plus/master/pynicotine/slskmessages.py",
            "note": "Nicotine+ class ChangePassword (code 142) serializes one password string payload.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-s6-batch3-control/official_frames.hex",
            "note": "S6 batch-3 probe includes attempted code 142 outbound payload while server resets were observed.",
        },
    ],
    ("server", "SM_REMOVE_ROOM_OPERATORSHIP"): [
        {
            "kind": "spec",
            "source": "https://raw.githubusercontent.com/nicotine-plus/nicotine-plus/master/pynicotine/slskmessages.py",
            "note": "Nicotine+ class RoomOperatorshipRevoked (code 146) parses room string payload.",
        },
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-legacy-room-operatorship-control/official_frames.hex",
            "note": "S6E authenticated runtime probe includes code 146 with len-prefixed room string payload.",
        },
    ],
    ("server", "SM_REMOVE_OWN_ROOM_OPERATORSHIP"): [
        {
            "kind": "runtime_capture",
            "source": "captures/redacted/login-legacy-room-operatorship-control/official_frames.hex",
            "note": "S6E authenticated runtime probe includes code 147 with same room-string wire layout used by code 146.",
        },
    ],
    ("server", "SM_MESSAGE_USER"): [
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 22 documents private message request and incoming event fields.",
        }
    ],
    ("server", "SM_MESSAGE_ACKED"): [
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 23 documents private message acknowledgement with message ID.",
        }
    ],
    ("server", "SM_GET_USER_STATUS"): [
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 7 defines user status response fields (status and privilege bit).",
        }
    ],
    ("server", "SM_GET_USER_STATS"): [
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 36 defines user stats response fields (speed/downloads/files/dirs).",
        }
    ],
    ("server", "SM_MESSAGE_USERS"): [
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 149 documents SendMessageUsers payload (users list + message).",
        }
    ],
    ("server", "SM_PEER_MESSAGE"): [
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 68 is documented as PeerMessage/TunneledMessage in protocol references.",
        }
    ],
    ("server", "SM_GET_SIMILAR_TERMS"): [
        {
            "kind": "string",
            "source": "evidence/reverse/message_name_strings.txt",
            "note": "Server string table includes SM_GET_SIMILAR_TERMS.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Protocol list documents similar recommendation request/response message family.",
        },
    ],
    ("server", "SM_GET_RECOMMENDATIONS"): [
        {
            "kind": "string",
            "source": "evidence/reverse/server_messagecodetostring_otool.txt",
            "note": "Server MessageCodeToString includes SM_GET_RECOMMENDATIONS.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Protocol list documents recommendation request/response code mapping.",
        },
    ],
    ("server", "SM_GET_MY_RECOMMENDATIONS"): [
        {
            "kind": "string",
            "source": "evidence/reverse/server_messagecodetostring_otool.txt",
            "note": "Server MessageCodeToString includes SM_GET_MY_RECOMMENDATIONS.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Protocol list documents my-recommendations request code mapping.",
        },
    ],
    ("server", "SM_GET_GLOBAL_RECOMMENDATIONS"): [
        {
            "kind": "string",
            "source": "evidence/reverse/server_messagecodetostring_otool.txt",
            "note": "Server MessageCodeToString includes SM_GET_GLOBAL_RECOMMENDATIONS.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Protocol list documents global recommendations message mapping.",
        },
    ],
    ("server", "SM_GET_USER_RECOMMENDATIONS"): [
        {
            "kind": "string",
            "source": "evidence/reverse/server_messagecodetostring_otool.txt",
            "note": "Server MessageCodeToString includes SM_GET_USER_RECOMMENDATIONS.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Protocol list documents user recommendation/interests message mapping.",
        },
    ],
    ("server", "SM_IGNORE_USER"): [
        {
            "kind": "string",
            "source": "evidence/reverse/server_messagecodetostring_otool.txt",
            "note": "Server MessageCodeToString includes SM_IGNORE_USER.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 11 documents IgnoreUser (obsolete) with username payload.",
        },
    ],
    ("server", "SM_UNIGNORE_USER"): [
        {
            "kind": "string",
            "source": "evidence/reverse/server_messagecodetostring_otool.txt",
            "note": "Server MessageCodeToString includes SM_UNIGNORE_USER.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 12 documents UnignoreUser (obsolete) with username payload.",
        },
    ],
    ("server", "SM_GET_OWN_PRIVILEGES_STATUS"): [
        {
            "kind": "string",
            "source": "evidence/reverse/message_name_strings.txt",
            "note": "Server string table includes SM_GET_OWN_PRIVILEGES_STATUS.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 92 documents CheckPrivileges with response payload indicating remaining seconds.",
        },
    ],
    ("server", "SM_GET_USER_PRIVILEGES_STATUS"): [
        {
            "kind": "string",
            "source": "evidence/reverse/message_name_strings.txt",
            "note": "Server string table includes SM_GET_USER_PRIVILEGES_STATUS.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 122 documents UserPrivileged (deprecated) with username and bool response fields.",
        },
    ],
    ("server", "SM_GIVE_PRIVILEGE"): [
        {
            "kind": "string",
            "source": "evidence/reverse/message_name_strings.txt",
            "note": "Server string table includes SM_GIVE_PRIVILEGE.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 123 documents GivePrivileges with username and number of days.",
        },
    ],
    ("server", "SM_INFORM_USER_OF_PRIVILEGES"): [
        {
            "kind": "string",
            "source": "evidence/reverse/message_name_strings.txt",
            "note": "Server string table includes SM_INFORM_USER_OF_PRIVILEGES.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 124 documents NotifyPrivileges with token and username.",
        },
    ],
    ("server", "SM_INFORM_USER_OF_PRIVILEGES_ACK"): [
        {
            "kind": "string",
            "source": "evidence/reverse/message_name_strings.txt",
            "note": "Server string table includes SM_INFORM_USER_OF_PRIVILEGES_ACK.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 125 documents AckNotifyPrivileges with token payload.",
        },
    ],
    ("server", "SM_ADD_ROOM_MEMBER"): [
        {
            "kind": "string",
            "source": "evidence/reverse/message_name_strings.txt",
            "note": "Server string table includes SM_ADD_ROOM_MEMBER.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 134 documents AddUserToPrivileged operation with room+username fields.",
        },
    ],
    ("server", "SM_REMOVE_ROOM_MEMBER"): [
        {
            "kind": "string",
            "source": "evidence/reverse/message_name_strings.txt",
            "note": "Server string table includes SM_REMOVE_ROOM_MEMBER.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 135 documents RemoveUserFromPrivileged operation with room+username fields.",
        },
    ],
    ("server", "SM_ADD_ROOM_OPERATOR"): [
        {
            "kind": "string",
            "source": "evidence/reverse/message_name_strings.txt",
            "note": "Server string table includes SM_ADD_ROOM_OPERATOR.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 143 documents AddOperatorToPrivileged operation with room+username fields.",
        },
    ],
    ("server", "SM_REMOVE_ROOM_OPERATOR"): [
        {
            "kind": "string",
            "source": "evidence/reverse/message_name_strings.txt",
            "note": "Server string table includes SM_REMOVE_ROOM_OPERATOR.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Server code 144 documents RemoveOperatorFromPrivileged operation with room+username fields.",
        },
    ],
    ("peer", "PM_GET_SHARED_FILES_IN_FOLDER"): [
        {
            "kind": "string",
            "source": "evidence/reverse/peer_messagecodetostring_otool.txt",
            "note": "Peer MessageCodeToString includes PM_GET_SHARED_FILES_IN_FOLDER.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Peer code 36 documents folder contents request with directory string payload.",
        },
    ],
    ("peer", "PM_SHARED_FILES_IN_FOLDER"): [
        {
            "kind": "string",
            "source": "evidence/reverse/peer_messagecodetostring_otool.txt",
            "note": "Peer MessageCodeToString includes PM_SHARED_FILES_IN_FOLDER.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Peer code 37 documents compressed folder listing response payload.",
        },
    ],
    ("peer", "PM_USER_INFO_REQUEST"): [
        {
            "kind": "string",
            "source": "evidence/reverse/peer_messagecodetostring_otool.txt",
            "note": "Peer MessageCodeToString includes PM_USER_INFO_REQUEST.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Peer code 15 documents user info request as empty payload.",
        },
    ],
    ("peer", "PM_USER_INFO_REPLY"): [
        {
            "kind": "string",
            "source": "evidence/reverse/peer_messagecodetostring_otool.txt",
            "note": "Peer MessageCodeToString includes PM_USER_INFO_REPLY.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Peer code 16 documents user info reply payload fields.",
        },
    ],
    ("peer", "PM_EXACT_FILE_SEARCH_REQUEST"): [
        {
            "kind": "string",
            "source": "evidence/reverse/peer_messagecodetostring_otool.txt",
            "note": "Peer MessageCodeToString includes PM_EXACT_FILE_SEARCH_REQUEST.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Peer code list includes code 47 for ExactFileSearchRequest (legacy/obsolete family).",
        },
    ],
    ("peer", "PM_INDIRECT_FILE_SEARCH_REQUEST"): [
        {
            "kind": "string",
            "source": "evidence/reverse/peer_messagecodetostring_otool.txt",
            "note": "Peer MessageCodeToString includes PM_INDIRECT_FILE_SEARCH_REQUEST.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Peer code list includes code 49 for IndirectFileSearchRequest (legacy/obsolete family).",
        },
    ],
    ("peer", "PM_UPLOAD_PLACE_IN_LINE_REQUEST"): [
        {
            "kind": "string",
            "source": "evidence/reverse/peer_messagecodetostring_otool.txt",
            "note": "Peer MessageCodeToString includes PM_UPLOAD_PLACE_IN_LINE_REQUEST.",
        },
        {
            "kind": "spec",
            "source": "https://nicotine-plus.org/doc/SLSKPROTOCOL.html",
            "note": "Peer code 51 documents place-in-line request carrying filename/path.",
        },
    ],
    ("peer", "PM_TRANSFER_REQUEST"): [
        {
            "kind": "ghidra_decompile",
            "source": "evidence/reverse/disasm/transfer_on_file_request.txt",
            "note": "Transfer queue dispatcher handles transfer request negotiation path.",
        }
    ],
    ("peer", "PM_TRANSFER_RESPONSE"): [
        {
            "kind": "ghidra_decompile",
            "source": "evidence/reverse/disasm/transfer_on_file_request.txt",
            "note": "Transfer queue dispatcher handles transfer response negotiation path.",
        }
    ],
    ("peer", "PM_QUEUE_UPLOAD"): [
        {
            "kind": "ghidra_decompile",
            "source": "evidence/reverse/disasm/transfer_on_queue_download.txt",
            "note": "Queue manager records upload queueing for pending peers.",
        }
    ],
    ("peer", "PM_UPLOAD_FAILED"): [
        {
            "kind": "ghidra_decompile",
            "source": "evidence/reverse/disasm/upload_write_socket.txt",
            "note": "Upload send path emits failure branch when transfer cannot continue.",
        }
    ],
    ("peer", "PM_UPLOAD_DENIED"): [
        {
            "kind": "ghidra_decompile",
            "source": "evidence/reverse/disasm/upload_write_socket.txt",
            "note": "Upload send path emits deny branch for rejected requests.",
        }
    ],
}


def now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def _normalize_confidence(raw: str) -> str:
    value = raw.strip().lower()
    if value in {"high", "medium", "low"}:
        return value
    return "medium"


def _source_kind(source: str) -> str:
    lowered = source.lower()
    if "/disasm/" in lowered or lowered.endswith(".asm"):
        return "ghidra_decompile"
    if lowered.endswith(".pcap"):
        return "pcap"
    if "frida" in lowered and lowered.endswith((".json", ".jsonl")):
        return "frida_hook"
    if lowered.endswith((".json", ".jsonl", ".hex")):
        return "runtime_capture"
    if "strings" in lowered or "messagecode" in lowered:
        return "string"
    return "manual_note"


def _read_message_map(path: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    if not path.exists():
        return rows

    with path.open("r", encoding="utf-8") as fh:
        reader = csv.DictReader(fh)
        for row in reader:
            scope = (row.get("scope") or "").strip()
            name = (row.get("name") or "").strip()
            if not scope or not name:
                continue

            code_raw = (row.get("code") or "").strip()
            try:
                code = int(code_raw)
            except ValueError:
                code = KNOWN_CODES.get((scope, name))

            rows.append(
                {
                    "scope": scope,
                    "name": name,
                    "code": code,
                    "confidence": _normalize_confidence((row.get("confidence") or "").strip()),
                    "source": (row.get("source") or "").strip(),
                    "status": (row.get("status") or "").strip(),
                    "notes": (row.get("notes") or "").strip(),
                }
            )

    return rows


def _dedupe_evidence(items: list[dict[str, str]]) -> list[dict[str, str]]:
    dedup: list[dict[str, str]] = []
    seen: set[tuple[str, str, str]] = set()

    for item in items:
        key = (item.get("kind", ""), item.get("source", ""), item.get("note", ""))
        if key in seen:
            continue
        seen.add(key)
        dedup.append(item)

    return dedup


def _entry(row: dict[str, Any]) -> dict[str, Any]:
    scope = row["scope"]
    name = row["name"]
    code = row.get("code")

    evidence: list[dict[str, str]] = []
    source = row.get("source")
    if source:
        evidence.append(
            {
                "kind": _source_kind(source),
                "source": source,
                "note": row.get("notes", "").strip() or "Message mapping source",
            }
        )

    evidence.extend(EXTRA_EVIDENCE.get((scope, name), []))
    evidence = _dedupe_evidence(evidence)

    confidence = _normalize_confidence(row.get("confidence", "medium"))
    if code is None:
        confidence = "medium"

    return {
        "scope": scope,
        "code": code,
        "name": name,
        "payload": KNOWN_PAYLOADS.get((scope, name), []),
        "confidence": confidence,
        "evidence": evidence,
    }


def _bootstrap_missing_entry(scope: str, name: str) -> dict[str, Any]:
    return {
        "scope": scope,
        "code": KNOWN_CODES[(scope, name)],
        "name": name,
        "payload": KNOWN_PAYLOADS.get((scope, name), []),
        "confidence": "medium",
        "evidence": [
            {
                "kind": "manual_note",
                "source": "docs/re/static/search-download-flow.md",
                "note": "Bootstrapped from static flow extraction; runtime evidence promotion pending.",
            }
        ],
    }


def build_schema(message_rows: list[dict[str, Any]]) -> dict[str, Any]:
    dedup: dict[tuple[str, str], dict[str, Any]] = {}
    for row in message_rows:
        key = (row["scope"], row["name"])
        dedup[key] = _entry(row)

    for scope, name in CORE_MESSAGES:
        key = (scope, name)
        if key not in dedup:
            dedup[key] = _bootstrap_missing_entry(scope, name)
            continue

        if dedup[key]["code"] is None:
            dedup[key]["code"] = KNOWN_CODES[key]

    entries = sorted(
        dedup.values(),
        key=lambda row: (row["scope"], row["code"] if row["code"] is not None else 10**9, row["name"]),
    )

    core_entries = [entry for entry in entries if (entry["scope"], entry["name"]) in set(CORE_MESSAGES)]
    high_count = sum(1 for entry in core_entries if entry["confidence"] == "high")
    medium_count = sum(1 for entry in core_entries if entry["confidence"] == "medium")
    low_count = sum(1 for entry in core_entries if entry["confidence"] == "low")

    return {
        "version": 2,
        "generated_at": now_iso(),
        "framing": {
            "transport": "tcp",
            "layout": "<u32 frame_len_le><u32 message_code_le><payload>",
            "confidence": "medium",
            "evidence": [
                {
                    "kind": "ghidra_decompile",
                    "source": "evidence/reverse/disasm/server_send_message.txt",
                    "note": "Server send path serializes integer fields through MemStream before socket write.",
                },
                {
                    "kind": "ghidra_decompile",
                    "source": "evidence/reverse/disasm/peer_send_message.txt",
                    "note": "Peer send path mirrors frame serialization through MemStream.",
                },
            ],
        },
        "coverage_contract": {
            "target_core_messages": 25,
            "high_min": 18,
            "medium_max": 7,
            "low_max": 0,
            "current": {
                "high": high_count,
                "medium": medium_count,
                "low": low_count,
            },
        },
        "messages": entries,
    }


def write_markdown(schema: dict[str, Any], out_path: Path) -> None:
    lines: list[str] = []
    lines.append("# Message Schema")
    lines.append("")
    lines.append(f"- Generated: `{schema['generated_at']}`")
    lines.append(f"- Framing: `{schema['framing']['layout']}`")
    lines.append(f"- Framing confidence: `{schema['framing']['confidence']}`")
    lines.append(
        "- Coverage contract: "
        f"`high >= {schema['coverage_contract']['high_min']}` "
        f"`medium <= {schema['coverage_contract']['medium_max']}` "
        f"`low <= {schema['coverage_contract']['low_max']}`"
    )
    lines.append(
        "- Current coverage: "
        f"`high={schema['coverage_contract']['current']['high']}` "
        f"`medium={schema['coverage_contract']['current']['medium']}` "
        f"`low={schema['coverage_contract']['current']['low']}`"
    )
    lines.append("")
    lines.append("## Messages")
    lines.append("")

    for entry in schema["messages"]:
        code = entry["code"] if entry["code"] is not None else "unknown"
        lines.append(f"### `{entry['scope']}` `{entry['name']}` (code `{code}`)")
        lines.append(f"- Confidence: `{entry['confidence']}`")
        if entry["payload"]:
            lines.append("- Payload fields:")
            for field in entry["payload"]:
                lines.append(f"  - `{field['name']}`: `{field['type']}`")
        else:
            lines.append("- Payload fields: pending derivation")
        lines.append("- Evidence:")
        for ev in entry["evidence"]:
            lines.append(f"  - `{ev['kind']}`: `{ev['source']}` ({ev.get('note', '').strip()})")
        lines.append("")

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description="Derive protocol message schema from KB evidence")
    parser.add_argument("--message-map", default="analysis/ghidra/maps/message_map.csv")
    parser.add_argument("--out-json", default="analysis/protocol/message_schema.json")
    parser.add_argument("--out-md", default="docs/re/static/message-schema.md")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parent.parent.parent
    message_rows = _read_message_map(repo_root / args.message_map)
    schema = build_schema(message_rows)

    out_json = repo_root / args.out_json
    out_json.parent.mkdir(parents=True, exist_ok=True)
    out_json.write_text(json.dumps(schema, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")

    write_markdown(schema, repo_root / args.out_md)

    print(json.dumps({"messages": len(schema["messages"]), "out_json": str(out_json)}, ensure_ascii=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
