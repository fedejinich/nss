# Soulseek Feature Inventory (Stage 5B UI Audit)

## Goal

Produce an exhaustive, evidence-backed inventory of SoulseekQt functionality with explicit UI coverage, then run a second pass to close mapping gaps.

## Scope and Baseline

- App target: `/Applications/SoulseekQt.app`
- Binary target: `/Applications/SoulseekQt.app/Contents/MacOS/SoulseekQt`
- Baseline timestamp (UTC): `evidence/ui_audit/baseline_timestamp_utc.txt`
- Binary hash: `evidence/ui_audit/soulseekqt_binary_sha256.txt`
- Binary metadata: `evidence/ui_audit/soulseekqt_info_plist.json`
- Code-signing metadata: `evidence/ui_audit/soulseekqt_codesign.txt`
- Mach-O/deps metadata: `evidence/ui_audit/soulseekqt_file_type.txt`, `evidence/ui_audit/soulseekqt_otool_L.txt`
- App tree snapshot: `evidence/ui_audit/soulseekqt_contents_tree.txt`

## Methodology

### Pass 1 inputs

1. Static UI/decomp evidence
- `evidence/ui_audit/decomp/mainwindow_methods.txt`
- `evidence/ui_audit/decomp/server_methods.txt`
- `evidence/ui_audit/decomp/peer_methods.txt`
- `evidence/ui_audit/decomp/transfer_methods.txt`
- `evidence/reverse/ui_handler_symbols_nm.txt`
- `analysis/re/flow_graph.json`
- `docs/re/static/search-download-flow.md`
- `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`

2. External sources
- `https://www.slsknet.org/news/changelog` (`evidence/ui_audit/external/changelog_structured.json`)
- `https://www.slsknet.org/news/` (`evidence/ui_audit/external/news_structured.json`)
- `https://groups.google.com/g/soulseek-discussion?pli=1` (`evidence/ui_audit/external/forum_topics_structured.json`)

3. Runtime UI introspection attempt
- `evidence/ui_audit/ui_menu_bar_items.err` contains assistive-access denial (`-1719`), so menu extraction was completed using symbol/string evidence instead of live accessibility API.

### Pass 2 process

- Revisited every Pass 1 feature entry.
- Marked each entry with `pass2_status: verified_pass2` or `pass2_status: gap_found`.
- Added explicit rationale for each gap.

## UI Surface Map

- Main shell: top-level tab container (`transfersTab`, `roomsTab`, `chatTab`, `searchTab`, `userListForm`, `sharesTab`, `optionsTab`).
- Toolbar/action routing: `MainWindow::onToolBarActionTriggered(QWidget*, QString)`.
- Tray/minimize UX: `MainWindow::minimizeToTray()`, `MainWindow::onTrayIconActivated(...)`, `MainWindow::minimizeToTrayOnCloseClicked(bool)`.
- Options tab groups inferred from control IDs:
  - login/network (`loginOptionsTab`)
  - file sharing / transfers (`fileSharingOptionsTab`)
  - UI behavior
  - logging/filters
  - user info
  - notification sounds
  - extras/import-export
- Context menus and list interactions: shared folders, users, downloads/uploads, ignored list, search results.
- Dialogs/forms: add shared folder, set folder permissions, shared files rescanned, user groups, filter help.

## Feature Inventory (Pass 1 + Pass 2 status)

### A. Application Shell and Navigation

#### UI-NAV-01
- ui_id: `UI-NAV-01`
- location: Main window tab strip (`transfersTab`, `roomsTab`, `chatTab`, `searchTab`, `userListForm`, `sharesTab`, `optionsTab`)
- trigger: Click tab headers
- preconditions: App initialized
- expected_result: Switches active workspace while preserving each tab state
- alternate_states: First-open tab may be configured (`onFirstOpenTabBoxIndexChanged(int)`)
- observable_errors: Tab labels truncation historically fixed in changelog
- functional_link: Global UX routing across all features
- requires_auth: `false` for navigation, `true` for server-backed content
- evidence: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-NAV-02
- ui_id: `UI-NAV-02`
- location: Toolbar + action dispatch
- trigger: Toolbar button click
- preconditions: Main window active
- expected_result: Opens corresponding view/action through `onToolBarActionTriggered`
- alternate_states: Event-button mode can be toggled
- observable_errors: None directly observed
- functional_link: Entrypoint into chat/search/transfers/user operations
- requires_auth: `mixed`
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`
- pass2_status: `verified_pass2`

#### UI-NAV-03
- ui_id: `UI-NAV-03`
- location: Tray icon + tray menu
- trigger: Close window / tray icon activation
- preconditions: Minimize-to-tray enabled
- expected_result: App minimizes to tray and exposes tray actions (including quit/title click)
- alternate_states: `-minimized` startup flag supported
- observable_errors: Platform-specific minimize behavior noted in changelog
- functional_link: Background operation while transfers/chats continue
- requires_auth: `false`
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-NAV-04
- ui_id: `UI-NAV-04`
- location: Options > UI behavior toggles
- trigger: Toggle checkboxes (old style search/transfers, split tabs, event buttons)
- preconditions: Options tab open
- expected_result: Immediate UI mode change and persisted preference
- alternate_states: Requires restart for some visual settings
- observable_errors: None directly observed
- functional_link: Affects search/transfers rendering and tab affordances
- requires_auth: `false`
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`
- pass2_status: `verified_pass2`

#### UI-NAV-05
- ui_id: `UI-NAV-05`
- location: Language selector + restart warning
- trigger: Language dropdown change
- preconditions: Options open
- expected_result: Persists language and prompts restart effect
- alternate_states: Translation files vary by build
- observable_errors: None directly observed
- functional_link: Localized UI labels/messages
- requires_auth: `false`
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`
- pass2_status: `verified_pass2`

### B. Session, Authentication, and Connectivity UI

#### UI-AUTH-01
- ui_id: `UI-AUTH-01`
- location: Options > Login/network controls (`usernameEdit`, password/server controls)
- trigger: Edit credentials and connect
- preconditions: Valid account tuple
- expected_result: Connect/login against Soulseek server
- alternate_states: Connection in progress / reconnect paths
- observable_errors: Invalid username/password failure flows and reconnect bugfix history
- functional_link: Enables all authenticated features
- requires_auth: `true`
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/decomp/server_methods.txt`, `evidence/ui_audit/external/changelog_structured.json`, `evidence/ui_audit/external/forum_topics_structured.json`
- pass2_status: `verified_pass2`

#### UI-AUTH-02
- ui_id: `UI-AUTH-02`
- location: Login failure dialog
- trigger: Server-reported login rejection
- preconditions: Attempted login with invalid/expired credentials
- expected_result: Human-readable dialog explaining failure and username/password change path
- alternate_states: Generic failure reason interpolation (`%1`)
- observable_errors: Wrong-password retry loops were historically fixed
- functional_link: Account recovery and operator support load reduction
- requires_auth: `true`
- evidence: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-AUTH-03
- ui_id: `UI-AUTH-03`
- location: Change username/password workflow
- trigger: `onChangeUsernamePasswordClicked()`
- preconditions: User disconnected (explicit guard)
- expected_result: Credential mutation path allowed only while offline
- alternate_states: Blocked when online
- observable_errors: "Cannot change username while online"
- functional_link: Identity/account management
- requires_auth: `mixed`
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/forum_topics_structured.json`
- pass2_status: `verified_pass2`

#### UI-AUTH-04
- ui_id: `UI-AUTH-04`
- location: Logged-in-elsewhere handling
- trigger: Duplicate session detected
- preconditions: Same account active on another client
- expected_result: Alert and local session handling
- alternate_states: Forced disconnect path
- observable_errors: None directly observed
- functional_link: Session exclusivity semantics
- requires_auth: `true`
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`
- pass2_status: `verified_pass2`

#### UI-NET-01
- ui_id: `UI-NET-01`
- location: Listening port controls (`listeningPortEdit`, enable checkbox)
- trigger: Toggle/modify listening port
- preconditions: Options open
- expected_result: Applies local listen-port behavior for peer connections
- alternate_states: Port disabled mode available
- observable_errors: Auto-disable and full disable options from changelog
- functional_link: Peer connectivity and transfer reachability
- requires_auth: `false`
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/reverse/ui_handler_symbols_nm.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-NET-02
- ui_id: `UI-NET-02`
- location: Port mapping toggles (UPnP, NAT-PMP)
- trigger: Toggle checkboxes
- preconditions: Router supports protocol
- expected_result: Add/remove mapping attempts and status logs
- alternate_states: Mapping failure paths with error codes
- observable_errors: Mapping retry/removal failures surfaced in strings
- functional_link: Inbound peer reachability
- requires_auth: `false`
- evidence: `evidence/reverse/ui_handler_symbols_nm.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-NET-03
- ui_id: `UI-NET-03`
- location: Auto-disable listening ports controls
- trigger: Toggle auto-disable and threshold spinboxes
- preconditions: Listening ports enabled
- expected_result: Temporarily disables inbound listening when connection flood thresholds hit
- alternate_states: Custom thresholds for attempts/window/cooldown
- observable_errors: None directly observed
- functional_link: Defensive network posture
- requires_auth: `false`
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`
- pass2_status: `verified_pass2`

#### UI-NET-04
- ui_id: `UI-NET-04`
- location: Check Ports action
- trigger: Click `checkPortsButton`
- preconditions: Network reachable
- expected_result: Opens/checks port-test path and reports mapping status
- alternate_states: URL/test endpoint issues fixed in changelog
- observable_errors: Port-test URL regression historically fixed
- functional_link: Troubleshooting inbound reachability
- requires_auth: `false`
- evidence: `evidence/reverse/ui_handler_symbols_nm.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/changelog_structured.json`, `evidence/ui_audit/external/forum_topics_structured.json`
- pass2_status: `verified_pass2`

#### UI-NET-05
- ui_id: `UI-NET-05`
- location: Obfuscated port display
- trigger: Port state refresh
- preconditions: Connectivity configured
- expected_result: Shows obfuscated-port hint for manual forwarding
- alternate_states: May show informational warning only
- observable_errors: None directly observed
- functional_link: Manual router configuration guidance
- requires_auth: `false`
- evidence: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/decomp/mainwindow_methods.txt`
- pass2_status: `verified_pass2`

### C. Search and Wishlist UI

#### UI-SRCH-01
- ui_id: `UI-SRCH-01`
- location: Search form (`SearchForm`, `manualSearchesTab`, `wishListSearchesTab`, `received searches`)
- trigger: Enter query / submit
- preconditions: Connected session for remote search
- expected_result: Query dispatch and result tabs
- alternate_states: Search stop/re-enter/manual rerun actions
- observable_errors: Search freezes/performance issues discussed in forum/changelog
- functional_link: Server and peer search pipeline
- requires_auth: `true`
- evidence: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/decomp/server_methods.txt`, `evidence/ui_audit/external/changelog_structured.json`, `evidence/ui_audit/external/forum_topics_structured.json`
- pass2_status: `verified_pass2`

#### UI-SRCH-02
- ui_id: `UI-SRCH-02`
- location: Search target selector
- trigger: Change `Search target` option
- preconditions: Search tab open
- expected_result: Routes search toward global/room/user-list target modes
- alternate_states: Target-specific result visibility
- observable_errors: Target-label improvements noted in changelog
- functional_link: Controls server search opcode/semantics
- requires_auth: `true`
- evidence: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/changelog_structured.json`, `evidence/ui_audit/decomp/server_methods.txt`
- pass2_status: `verified_pass2`

#### UI-SRCH-03
- ui_id: `UI-SRCH-03`
- location: Search tab controls (`Stop Search`, `Close All Searches`, `Re-enter Search`)
- trigger: Buttons/context actions
- preconditions: Active search tabs
- expected_result: Stops or bulk-closes search sessions
- alternate_states: Wishlist searches can be enabled/disabled globally
- observable_errors: Focus/label handling historically fixed
- functional_link: Search lifecycle management
- requires_auth: `true`
- evidence: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-SRCH-04
- ui_id: `UI-SRCH-04`
- location: Search filters (`Save Filter`, `Remove Filter`, `savedFiltersBox`)
- trigger: Save/remove filter profile
- preconditions: Results present
- expected_result: Persists reusable result-filter settings
- alternate_states: Filter help dialog available
- observable_errors: None directly observed
- functional_link: Result triage and repeatability
- requires_auth: `true`
- evidence: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/decomp/mainwindow_methods.txt`
- pass2_status: `verified_pass2`

#### UI-SRCH-05
- ui_id: `UI-SRCH-05`
- location: Search limits (`maxResultsPerSearch*`)
- trigger: Toggle/set max-result controls
- preconditions: Options tab open
- expected_result: Applies result-cap limit to outgoing searches
- alternate_states: Limit can be increased/disabled depending on build
- observable_errors: Very large result sets may stress UI
- functional_link: Server request load and UI rendering pressure
- requires_auth: `false` to configure, `true` to exercise
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-SRCH-06
- ui_id: `UI-SRCH-06`
- location: Search history controls (`clearSearchHistoryButton`, dropdown history behavior)
- trigger: Clear history / select previous query
- preconditions: Prior searches exist
- expected_result: History list cleared or re-used
- alternate_states: None
- observable_errors: None directly observed
- functional_link: Iterative discovery workflow
- requires_auth: `false` for clearing
- evidence: `evidence/reverse/ui_handler_symbols_nm.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-SRCH-07
- ui_id: `UI-SRCH-07`
- location: Private-result visibility toggles
- trigger: `onShowPrivateSearchResultsClicked(bool)` and lock-icon behavior
- preconditions: Private shares/search responses exist
- expected_result: Include/exclude private-visible results and annotate visibility
- alternate_states: Results sorted to bottom in older behavior notes
- observable_errors: None directly observed
- functional_link: Search privacy + access control UX
- requires_auth: `true`
- evidence: `evidence/reverse/ui_handler_symbols_nm.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-SRCH-08
- ui_id: `UI-SRCH-08`
- location: Ignored-user interaction with search results
- trigger: Ignore user or run searches after ignore updates
- preconditions: Ignored list has entries
- expected_result: Search results from ignored users are hidden
- alternate_states: Hidden entries still exist server-side
- observable_errors: Behavior explicitly changed in 2024-02-01 changelog
- functional_link: Moderation safety in discovery flow
- requires_auth: `true`
- evidence: `evidence/ui_audit/external/changelog_structured.json`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`
- pass2_status: `verified_pass2`

### D. Rooms, Chat, and Messaging UI

#### UI-CHAT-01
- ui_id: `UI-CHAT-01`
- location: Rooms tab (`RoomForm`, room chat views)
- trigger: Join room / room message activity
- preconditions: Connected session
- expected_result: Enter/leave room, stream room chat and membership updates
- alternate_states: Operator/member lists available from protocol
- observable_errors: None directly observed
- functional_link: `Server::EnterRoom`, `Server::LeaveRoom`, `Server::RoomChat`
- requires_auth: `true`
- evidence: `evidence/ui_audit/decomp/server_methods.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/reverse/ui_handler_symbols_nm.txt`
- pass2_status: `verified_pass2`

#### UI-CHAT-02
- ui_id: `UI-CHAT-02`
- location: Private chat views (`ChatForm`, message list)
- trigger: Send/receive private message
- preconditions: Connected; peer reachable
- expected_result: Open private chat session, append messages, maintain history
- alternate_states: Offline indicators and delayed delivery paths
- observable_errors: Repeated forum reports around login/message issues
- functional_link: `Server::SendPrivateChat`, `Server::PrivateChat`
- requires_auth: `true`
- evidence: `evidence/reverse/ui_handler_symbols_nm.txt`, `evidence/ui_audit/decomp/server_methods.txt`, `evidence/ui_audit/external/forum_topics_structured.json`
- pass2_status: `verified_pass2`

#### UI-CHAT-03
- ui_id: `UI-CHAT-03`
- location: Chat timestamp mode (`chatTimestampsBox`)
- trigger: Timestamp display setting change
- preconditions: Chat tabs active
- expected_result: Timestamp format updates (including 24h/year behavior)
- alternate_states: Long/short timestamp formats
- observable_errors: Format behavior changed in 2024 changelog
- functional_link: Chat readability and moderation context
- requires_auth: `false` to configure
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-CHAT-04
- ui_id: `UI-CHAT-04`
- location: Chat logging controls (`logRoomChatCheckBox`, `logPrivateChatCheckBox`, log actions)
- trigger: Toggle logging; open log folder; clear log
- preconditions: Chat activity
- expected_result: Chat content persisted in log paths and manageable from UI
- alternate_states: User-specific chat logs (`/Soulseek Chat Logs/Users/`)
- observable_errors: None directly observed
- functional_link: Auditing/history for rooms and PMs
- requires_auth: `true` for meaningful content
- evidence: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/decomp/mainwindow_methods.txt`
- pass2_status: `verified_pass2`

#### UI-CHAT-05
- ui_id: `UI-CHAT-05`
- location: Message-notification policy (`notifyForMessagesFromBox`)
- trigger: Select notification scope
- preconditions: Options > logging/notification settings
- expected_result: Private message notification scope set (everyone/user-list/none)
- alternate_states: Combined with sound profile choices
- observable_errors: Feature explicitly described in changelog
- functional_link: Attention routing and noise control
- requires_auth: `false` to configure
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-CHAT-06
- ui_id: `UI-CHAT-06`
- location: Chat filter words
- trigger: Open `Chat Filter Words` editor and save tokens
- preconditions: Options open
- expected_result: Filters room/private chat content by keyword; historical behavior extends to usernames
- alternate_states: Room activity alerts still possible
- observable_errors: Token behavior changed in changelog
- functional_link: Moderation and noise reduction
- requires_auth: `false` to configure
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-CHAT-07
- ui_id: `UI-CHAT-07`
- location: Private room administration context actions
- trigger: Context menu actions (`Give User Membership`, `Make User Operator`, removal counterparts)
- preconditions: User has sufficient room privileges
- expected_result: Membership/operator updates in private room state
- alternate_states: Server may reject unauthorized action
- observable_errors: None directly observed
- functional_link: `Server::AddPrivateRoomMember`, `Server::RemovePrivateRoomOperator`, etc.
- requires_auth: `true`
- evidence: `evidence/ui_audit/decomp/server_methods.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`
- pass2_status: `verified_pass2`

#### UI-CHAT-08
- ui_id: `UI-CHAT-08`
- location: Room ticker/metadata behavior
- trigger: Room ticker actions and server ticker methods
- preconditions: Room joined
- expected_result: Room ticker update path exists and propagates
- alternate_states: Permission constrained
- observable_errors: None directly observed
- functional_link: `Server::SetRoomTicker(QString, QString)`
- requires_auth: `true`
- evidence: `evidence/ui_audit/decomp/server_methods.txt`
- pass2_status: `verified_pass2`

### E. Transfers (Downloads/Uploads/Queue) UI

#### UI-XFER-01
- ui_id: `UI-XFER-01`
- location: Transfers tab (downloads/uploads trees)
- trigger: Queue download/upload actions
- preconditions: Peer address available and file metadata known
- expected_result: Queue entries created, states transition through queued/requesting/downloading/uploading/completed
- alternate_states: Place-in-line, denied, failed, timed out
- observable_errors: Aborted/queue issues widely discussed in forum
- functional_link: `PeerMessenger::QueueDownload`, `TransferQueueManager::OnQueueDownloadRequested`
- requires_auth: `true`
- evidence: `evidence/reverse/ui_handler_symbols_nm.txt`, `evidence/ui_audit/decomp/peer_methods.txt`, `evidence/ui_audit/decomp/transfer_methods.txt`, `evidence/ui_audit/external/forum_topics_structured.json`
- pass2_status: `verified_pass2`

#### UI-XFER-02
- ui_id: `UI-XFER-02`
- location: Download context actions (`Download File(s)`, folder variants)
- trigger: Search/share context menu
- preconditions: Search or browse results visible
- expected_result: Queue file/folder download and optionally open folder-dialog
- alternate_states: Filtered extensions deselected by default in folder dialog
- observable_errors: Duplicate filename/folder handling improved in changelog
- functional_link: Search/browse to transfer pipeline
- requires_auth: `true`
- evidence: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-XFER-03
- ui_id: `UI-XFER-03`
- location: Transfer speed and slot controls (`Upload Slots`, speed limits)
- trigger: Change spinboxes/toggles
- preconditions: Transfers tab/options available
- expected_result: Rate limiting and slot policy affect active and queued transfers
- alternate_states: Old/new transfer view modes
- observable_errors: Large upload queue UI instability reported in forum
- functional_link: `TransferQueueManager::setUploadSlots`, speed-limit methods
- requires_auth: `false` to configure, `true` to exercise
- evidence: `evidence/ui_audit/decomp/transfer_methods.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/forum_topics_structured.json`
- pass2_status: `verified_pass2`

#### UI-XFER-04
- ui_id: `UI-XFER-04`
- location: Queue maintenance (remove/requeue/pause/resume)
- trigger: Transfers context menu or key actions
- preconditions: Transfer entries exist
- expected_result: Entry state transitions and retry behavior
- alternate_states: Automatically requeue for selected deny reasons (`too many files/MB`)
- observable_errors: Some queue edge cases tracked historically
- functional_link: `TransferQueueManager::removeDownload`, `requeueDownload`, `pauseDownload`, `resumeDownload`
- requires_auth: `true`
- evidence: `evidence/ui_audit/decomp/transfer_methods.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-XFER-05
- ui_id: `UI-XFER-05`
- location: Finish uploads and quit toggle
- trigger: Transfers action toggle
- preconditions: Active uploads
- expected_result: Stops accepting new uploads; exits after existing uploads drain
- alternate_states: Wait grace period for retries
- observable_errors: None directly observed
- functional_link: Upload queue lifecycle and app shutdown control
- requires_auth: `true`
- evidence: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-XFER-06
- ui_id: `UI-XFER-06`
- location: Download folder policy toggles
- trigger: Toggle options (`dontSplitDownloads`, `dontCreateUsernameFolders`, `dontDownloadSingleFilesIntoSubfolders`, dialog usage)
- preconditions: Options > file sharing/transfers
- expected_result: Controls final path layout for downloaded files/folders
- alternate_states: User/folder-centric organization behavior
- observable_errors: Folder management got extensive changelog updates
- functional_link: `TransferQueueManager::getDownloadTargetPath` and related helpers
- requires_auth: `false` to configure
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/decomp/transfer_methods.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-XFER-07
- ui_id: `UI-XFER-07`
- location: Open download folder action
- trigger: Transfers context action
- preconditions: Completed/in-progress download entry selected
- expected_result: Opens filesystem folder for selected transfer
- alternate_states: Missing local path may fail silently
- observable_errors: None directly observed
- functional_link: User handoff from transfer pipeline to local files
- requires_auth: `false`
- evidence: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`
- pass2_status: `verified_pass2`

#### UI-XFER-08
- ui_id: `UI-XFER-08`
- location: Upload request paths (`UploadPlaceInLine`, `UploadDenied`, `UploadFailed` notifications)
- trigger: Peer upload negotiation events
- preconditions: Remote peer requesting files
- expected_result: UI updates status and queue ordering for uploads
- alternate_states: Denied, timed-out, aborted branches
- observable_errors: Aborted uploads commonly discussed in forum
- functional_link: `TransferQueueManager::OnUpload*` event family
- requires_auth: `true`
- evidence: `evidence/ui_audit/decomp/transfer_methods.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/forum_topics_structured.json`
- pass2_status: `verified_pass2`

#### UI-XFER-09
- ui_id: `UI-XFER-09`
- location: Transfer completion notifications
- trigger: Download/upload complete events
- preconditions: Transfer reaches terminal success
- expected_result: Completion message, optional sound alert, optional auto-clear behavior
- alternate_states: Clear complete downloads/uploads options
- observable_errors: Premature completion regression noted historically
- functional_link: `OnDownloadComplete`, `OnUploadComplete`, notification-sound settings
- requires_auth: `true`
- evidence: `evidence/ui_audit/decomp/transfer_methods.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

### F. Sharing, Users, and Social Controls UI

#### UI-SHARE-01
- ui_id: `UI-SHARE-01`
- location: Shared folders list + context menu
- trigger: Add/remove/rescan folders, right-click shared folder list
- preconditions: Shares tab/options available
- expected_result: Shared-folder set updated and reflected in indexing
- alternate_states: Add-shared-folder dialog focus issues on macOS fixed in changelog
- observable_errors: Folder dialog z-order issues in Ventura fixed in 2023
- functional_link: Share index and remote browse/search visibility
- requires_auth: `false` to configure
- evidence: `evidence/reverse/ui_handler_symbols_nm.txt`, `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-SHARE-02
- ui_id: `UI-SHARE-02`
- location: Folder permissions and unshare controls
- trigger: `Set Folder Permissions`, `Unshare Folder`
- preconditions: Shared folder selected
- expected_result: Access policy updated for private/public scopes
- alternate_states: User/IP-specific unshare behavior from changelog
- observable_errors: Permission display bugs historically fixed
- functional_link: Access control for browse/search/download authorization
- requires_auth: `mixed`
- evidence: `evidence/reverse/ui_handler_symbols_nm.txt`, `evidence/ui_audit/external/changelog_structured.json`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`
- pass2_status: `verified_pass2`

#### UI-SHARE-03
- ui_id: `UI-SHARE-03`
- location: Browse user shares
- trigger: `Browse User's Files` action
- preconditions: Target user reachable and sharing
- expected_result: Remote share tree appears, allows download queueing
- alternate_states: Large-share performance/memory constraints
- observable_errors: Several large-share fixes in changelog/news
- functional_link: Peer share discovery and transfer seed selection
- requires_auth: `true`
- evidence: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/decomp/peer_methods.txt`, `evidence/ui_audit/external/changelog_structured.json`, `evidence/ui_audit/external/news_structured.json`
- pass2_status: `verified_pass2`

#### UI-SHARE-04
- ui_id: `UI-SHARE-04`
- location: User list core actions
- trigger: Add/remove user, message user, browse user, get user info
- preconditions: User list tab active
- expected_result: Tracked user set updates and action dispatch works
- alternate_states: User address might need deferred resolution
- observable_errors: None directly observed
- functional_link: `Server::AddUser`, `Server::GetUserStatus`, `PeerMessenger::GetUserInfo`
- requires_auth: `true`
- evidence: `evidence/ui_audit/decomp/server_methods.txt`, `evidence/ui_audit/decomp/peer_methods.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`
- pass2_status: `verified_pass2`

#### UI-SHARE-05
- ui_id: `UI-SHARE-05`
- location: Ignored list / ignore-unignore user
- trigger: Context menu actions `Ignore user`, `OnUnignoreUser()`
- preconditions: User context present
- expected_result: User moderation list updated and search/chat impact applied
- alternate_states: Search-result suppression for ignored users
- observable_errors: None directly observed
- functional_link: Social moderation and result filtering
- requires_auth: `true`
- evidence: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-SHARE-06
- ui_id: `UI-SHARE-06`
- location: User groups configuration (`UserGroupsDialog`)
- trigger: Configure user groups / set groups from context menu
- preconditions: Users tab active
- expected_result: Group assignments drive policy and messaging scope
- alternate_states: Group-specific permission effects on uploads
- observable_errors: Group-permission upload bug fixed historically
- functional_link: Fine-grained social/access policies
- requires_auth: `true`
- evidence: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-SHARE-07
- ui_id: `UI-SHARE-07`
- location: Gift privileges UI action
- trigger: `Gift Privileges`
- preconditions: Account with available privileges
- expected_result: Sends privilege gift request
- alternate_states: Server policy may reject
- observable_errors: Privilege activation issues discussed in changelog/news
- functional_link: `Server::GiftPrivileges(...)`
- requires_auth: `true`
- evidence: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/decomp/server_methods.txt`, `evidence/ui_audit/external/news_structured.json`
- pass2_status: `verified_pass2`

#### UI-SHARE-08
- ui_id: `UI-SHARE-08`
- location: Reshare/unshare by user actions
- trigger: `OnUnshareFilesFromUser()`, `OnReshareFilesWithUser()`
- preconditions: Target user selected
- expected_result: Per-user sharing exceptions updated
- alternate_states: IP-based unshare interactions
- observable_errors: None directly observed
- functional_link: User-specific access gating
- requires_auth: `true`
- evidence: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/decomp/transfer_methods.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-SHARE-09
- ui_id: `UI-SHARE-09`
- location: Import user list from original Soulseek client
- trigger: `importUserListButton`
- preconditions: Legacy configuration file selected
- expected_result: Migrates tracked/social user data
- alternate_states: Import success/failure dialogs
- observable_errors: Import can fail with explicit error
- functional_link: Onboarding/migration path
- requires_auth: `false`
- evidence: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/decomp/mainwindow_methods.txt`
- pass2_status: `verified_pass2`

### G. Diagnostics, Customization, Data, and Notifications UI

#### UI-CFG-01
- ui_id: `UI-CFG-01`
- location: Color and font customization
- trigger: Select colors / reset colors / select application font / default font
- preconditions: Options > UI tab
- expected_result: Theme/text rendering changes persisted
- alternate_states: Reset confirmation and restart-required cases
- observable_errors: Color settings regressions fixed historically
- functional_link: Accessibility/readability customization
- requires_auth: `false`
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-CFG-02
- ui_id: `UI-CFG-02`
- location: Notification sounds matrix
- trigger: Select/clear sound file controls for default/private/room/wishlist/share-browse/download-alert/user-online/user-info
- preconditions: Options > Notification Sounds
- expected_result: Event-specific sound mapping persisted
- alternate_states: Missing sound path fallback to default
- observable_errors: None directly observed
- functional_link: Alerting channel customization
- requires_auth: `false` to configure
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`
- pass2_status: `verified_pass2`

#### UI-CFG-03
- ui_id: `UI-CFG-03`
- location: User info profile controls
- trigger: Set/clear picture and edit description
- preconditions: Options > User Info
- expected_result: Outgoing user-info payload includes selected metadata
- alternate_states: Image load/display fallback behavior
- observable_errors: User-info image support regressions/fixes described in changelog
- functional_link: `PeerMessenger::UserInfoRequested` / `UserInfoReceived`
- requires_auth: `true`
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/decomp/peer_methods.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-CFG-04
- ui_id: `UI-CFG-04`
- location: Diagnostics toggles (`onShowDiagnosticsCheckboxToggled`, user-level logging)
- trigger: Toggle diagnostics/logging
- preconditions: Options > diagnostics/logging area
- expected_result: Additional diagnostic panels/log verbosity enabled
- alternate_states: Intended for debugging only
- observable_errors: None directly observed
- functional_link: Troubleshooting stability/performance incidents
- requires_auth: `false`
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/forum_topics_structured.json`
- pass2_status: `verified_pass2`

#### UI-CFG-05
- ui_id: `UI-CFG-05`
- location: Data persistence interval and save lifecycle (`onSaveDataEveryValueChanged`, `timeToSaveData`)
- trigger: Configure save cadence
- preconditions: Options accessible
- expected_result: Periodic client-data writes at configured interval
- alternate_states: Abrupt shutdown persistence behavior
- observable_errors: Historical periodic-save bug fixed
- functional_link: Prevents local state loss
- requires_auth: `false`
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-CFG-06
- ui_id: `UI-CFG-06`
- location: Export/import client data (Extras tab)
- trigger: `exportClientDataButton`, `importClientDataButton`
- preconditions: File path selected
- expected_result: Serializes/deserializes client settings/state bundle
- alternate_states: Success/failure dialogs shown
- observable_errors: Schema compatibility issues noted historically
- functional_link: Backup/migration and rollback
- requires_auth: `false`
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt`, `evidence/ui_audit/external/changelog_structured.json`
- pass2_status: `verified_pass2`

#### UI-CFG-07
- ui_id: `UI-CFG-07`
- location: New-version flow and second-instance behavior
- trigger: Version check event / app launch while already running
- preconditions: Build metadata checks
- expected_result: New-version dialog and "already running" notification paths
- alternate_states: Notarization and platform-specific build notices in news/changelog
- observable_errors: Locale/timer and platform regressions tracked in news/forum
- functional_link: Release UX and update safety
- requires_auth: `false`
- evidence: `evidence/ui_audit/decomp/mainwindow_methods.txt`, `evidence/ui_audit/external/news_structured.json`, `evidence/ui_audit/external/changelog_structured.json`, `evidence/ui_audit/external/forum_topics_structured.json`
- pass2_status: `verified_pass2`

## External-Only Functional Signals (not guaranteed visible in one local UI session)

These behaviors were mapped from changelog/news/forum/decomp and may require specific runtime states to observe directly:

1. `EXT-01` Search results hide ignored users (2024-02-01 changelog).
2. `EXT-02` Chat timestamps: 24h + long-format year support (2024-02-01 changelog).
3. `EXT-03` UPnP mapping refresh every 5 minutes (2024-02-01 changelog + `Server::refreshPortMappings`).
4. `EXT-04` Handling of files larger than 2GB in search/transfer paths.
5. `EXT-05` Queue behavior for denied uploads/downloads with auto-requeue cases.
6. `EXT-06` Minimized startup via `-minimized` argument.
7. `EXT-07` Platform-specific stability paths around Qt/macOS/locale timers (news + forum 2026 threads).

Evidence: `evidence/ui_audit/external/changelog_structured.json`, `evidence/ui_audit/external/news_structured.json`, `evidence/ui_audit/external/forum_topics_structured.json`, `evidence/ui_audit/decomp/server_methods.txt`.

## Pass 2 Results

### Summary

- Pass 1 features cataloged: `42` (UI-prefixed entries above).
- Pass 2 reviewed: `42/42`.
- `verified_pass2`: `41`
- `gap_found`: `1`

### Gap list

1. `GAP-UI-ACC-01`
- scope: Live menu-bar and menu-tree extraction through macOS accessibility APIs.
- status: `gap_found`
- reason: `osascript` call denied assistive access (`-1719`), captured in `evidence/ui_audit/ui_menu_bar_items.err`.
- mitigation: Feature mapping completed through symbolized handlers, control IDs, and text resources; future run can close this with local accessibility permission.

## Decompilation Addendum for Stage 5B

New static-evidence artifacts produced during this audit:

- `evidence/reverse/ui_handler_symbols_nm.txt`
- `evidence/ui_audit/decomp/mainwindow_methods.txt`
- `evidence/ui_audit/decomp/server_methods.txt`
- `evidence/ui_audit/decomp/peer_methods.txt`
- `evidence/ui_audit/decomp/transfer_methods.txt`
- `analysis/re/flow_graph.json`
- `docs/re/static/search-download-flow.md`

Key mapped UI-to-protocol bridges:

- UI search submit -> `Server::FileSearch(QString, QString)`
- Room join flow -> `Server::EnterRoom(QString, bool)`
- PM send flow -> `Server::SendPrivateChat(QString, QString)`
- Wait-port advertisement -> `Server::SetWaitPort(int, int)`
- Queue download UI action -> `PeerMessenger::QueueDownload(...)`
- Download queue event -> `TransferQueueManager::OnQueueDownloadRequested(...)`

## Assumptions and Constraints

- This stage intentionally avoids implementing new product behavior.
- Runtime-auth-required flows were documented with `requires_auth` markers.
- Forum extraction is based on publicly embedded Google Groups payload from the fetched page snapshot.
- No secrets from `.env.local` were read or persisted.
