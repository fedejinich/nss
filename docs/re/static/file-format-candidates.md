# Static Persistence Format Candidates

- Generated at: `2026-02-17T01:13:42+00:00`
- Candidate count: `6`

| ID | Confidence | Symbol Hits | Method Hits | String Hits |
|---|---:|---:|---:|---:|
| `FMT-SETTINGS-QSETTINGS` | `high` | 4 | 3 | 8 |
| `FMT-TRANSFER-STATE` | `high` | 5 | 3 | 8 |
| `FMT-USERLIST-IMPORT` | `medium` | 3 | 2 | 4 |
| `FMT-CLIENT-DATA-EXPORT-IMPORT` | `high` | 4 | 2 | 8 |
| `FMT-SHARE-SCAN-CACHE` | `high` | 3 | 3 | 8 |
| `FMT-SEARCH-HISTORY-PREFERENCES` | `high` | 2 | 3 | 8 |

## FMT-SETTINGS-QSETTINGS

- Title: QSettings-backed persistent options
- Confidence: `high`
- Notes: Key/value persistence for UI and transfer behavior toggles.
- Evidence snippets:
  - symbol_hits: `4`
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:713` (undefined) external QSettings::remove(QAnyStringView) (from QtCore)
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:714` (undefined) external QSettings::setValue(QAnyStringView, QVariant const&) (from QtCore)
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:994` (undefined) external QSettings::value(QAnyStringView) const (from QtCore)
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:995` (undefined) external QSettings::contains(QAnyStringView) const (from QtCore)
  - method_hits: `3`
    - `evidence/ui_audit/decomp/mainwindow_methods.txt:62` MainWindow::onExportClientDataClicked()
    - `evidence/ui_audit/decomp/mainwindow_methods.txt:66` MainWindow::onImportClientDataClicked()
    - `evidence/ui_audit/decomp/mainwindow_methods.txt:136` MainWindow::saveData()
  - string_hits: `8`
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:397` 10928:minimize_on_close
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:402` 10939:transfer_queued
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:419` 10974:max_results_per_search
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:420` 10975:max_results_per_search_enabled
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:459` 11043:1onShowPrivateSearchResultsClicked(bool)
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:460` 11044:1onShowPrivatelySharedFilesInSharesClicked(bool)
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:622` 11345:showPrivateResultsCheckBox
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:623` 11346:showPrivatelySharedFilesInSharesCheckBox

## FMT-TRANSFER-STATE

- Title: Transfer queue and progress persistence
- Confidence: `high`
- Notes: Serialized transfer queue/in-progress/completed state and requeue semantics.
- Evidence snippets:
  - symbol_hits: `5`
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:1808` 00000001000bbc68 (__TEXT,__text) external WriteString(QFile&, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>)
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:1809` 00000001000bbd18 (__TEXT,__text) external ReadString(QFile&, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>&)
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:2741` 000000010019eb50 (__TEXT,__text) external QFileStreamer::ReadBuffer(void*, unsigned long)
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:2742` 000000010019eb7c (__TEXT,__text) external QFileStreamer::WriteBuffer(void const*, unsigned long)
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:2743` 000000010019eba8 (__TEXT,__text) external non-virtual thunk to QFileStreamer::WriteBuffer(void const*, unsigned long)
  - method_hits: `3`
    - `evidence/ui_audit/decomp/transfer_methods.txt:30` TransferQueueManager::OnDataLoaded()
    - `evidence/ui_audit/decomp/transfer_methods.txt:59` TransferQueueManager::TransfersLoaded()
    - `evidence/ui_audit/decomp/transfer_methods.txt:88` TransferQueueManager::requeueDownload(Item)
  - string_hits: `8`
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:402` 10939:transfer_queued
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:407` 10955:Transfers - Queued
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:491` 11088:max_queued_uploads_per_user_enabled
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:492` 11089:max_queued_mb_per_user_enabled
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:494` 11091:max_queued_uploads_per_user
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:495` 11092:max_queued_mb_per_user
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:574` 11250:Data successfully imported
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:575` 11251:Client data successfully imported!

## FMT-USERLIST-IMPORT

- Title: User list import format
- Confidence: `medium`
- Notes: Legacy hotlist import pathway from `hotlist.cfg`.
- Evidence snippets:
  - symbol_hits: `3`
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:89` (undefined) external QFileDialog::getOpenFileName(QWidget*, QString const&, QString const&, QString const&, QString*, QFlags<QFileDialog::Option>) (from QtWidgets)
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:391` (undefined) external QFile::open(QFlags<QIODeviceBase::OpenModeFlag>) (from QtCore)
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:1247` 00000001000280cc (__TEXT,__text) external MainWindow::importConfigurationData()
  - method_hits: `2`
    - `evidence/ui_audit/decomp/mainwindow_methods.txt:13` MainWindow::importConfigurationData()
    - `evidence/ui_audit/decomp/mainwindow_methods.txt:67` MainWindow::onImportUserListClicked()
  - string_hits: `4`
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:560` 11218:To import your user list, your original Soulseek client must be set not to save configuration data to the registry (under Options->General), and the hotlist.cfg file must then be located on your computer. The next dialog will ask you to navigate to the exact location of the hotlist.cfg file.
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:665` 11457:importUserListButton
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:2352` 22075:To import your user list, your original Soulseek client must be set not to save configuration data to the registry (under Options->General), and the hotlist.cfg file must then be located on your computer. The next dialog will ask you to navigate to the exact location of the hotlist.cfg file.
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:2457` 22314:importUserListButton

## FMT-CLIENT-DATA-EXPORT-IMPORT

- Title: Client data backup/restore format
- Confidence: `high`
- Notes: Export/import UI surface for complete client-data snapshot.
- Evidence snippets:
  - symbol_hits: `4`
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:76` (undefined) external QDataStream::writeBytes(char const*, long long) (from QtCore)
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:77` (undefined) external QDataStream::readRawData(char*, long long) (from QtCore)
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:1247` 00000001000280cc (__TEXT,__text) external MainWindow::importConfigurationData()
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:1252` 0000000100029858 (__TEXT,__text) external MainWindow::saveData()
  - method_hits: `2`
    - `evidence/ui_audit/decomp/mainwindow_methods.txt:62` MainWindow::onExportClientDataClicked()
    - `evidence/ui_audit/decomp/mainwindow_methods.txt:66` MainWindow::onImportClientDataClicked()
  - string_hits: `8`
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:571` 11240:Client data exported
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:572` 11242:Client data export failed
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:575` 11251:Client data successfully imported!
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:576` 11252:Data import failed
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:2363` 22097:Client data exported
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:2364` 22099:Client data export failed
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:2367` 22108:Client data successfully imported!
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:2368` 22109:Data import failed

## FMT-SHARE-SCAN-CACHE

- Title: Share scan and file-index cache
- Confidence: `high`
- Notes: Persisted share scan outcomes and refresh behavior linked to transfer availability.
- Evidence snippets:
  - symbol_hits: `3`
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:392` (undefined) external QFile::exists(QString const&) (from QtCore)
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:393` (undefined) external QFile::remove(QString const&) (from QtCore)
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:975` (undefined) external QFileInfo::absoluteFilePath() const (from QtCore)
  - method_hits: `3`
    - `evidence/ui_audit/decomp/mainwindow_methods.txt:104` MainWindow::onSharedFilesRescanned(int, int)
    - `evidence/ui_audit/decomp/mainwindow_methods.txt:105` MainWindow::onSharedFolderAdded(Item)
    - `evidence/ui_audit/decomp/transfer_methods.txt:79` TransferQueueManager::onUserShare(QString, QSharedPointer<std::__1::map<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::set<FileInfo, std::__1::less<FileInfo>, std::__1::allocator<FileInfo>>, std::__1::less<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>>, std::__1::allocator<std::__1::pair<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const, std::__1::set<FileInfo, std::__1::less<FileInfo>, std::__1::allocator<FileInfo>>>>>>, QSharedPointer<std::__1::map<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::set<FileInfo, std::__1::less<FileInfo>, std::__1::allocator<FileInfo>>, std::__1::less<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>>, std::__1::allocator<std::__1::pair<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const, std::__1::set<FileInfo, std::__1::less<FileInfo>, std::__1::allocator<FileInfo>>>>>>, std::__1::map<int, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::less<int>, std::__1::allocator<std::__1::pair<int const, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>>>>&)
  - string_hits: `8`
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:1` 74:shared_fH
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:3` 78:shared_fH
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:5` 82:shared_fH
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:7` 87:shared_fH
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:16` 363:shared_fH
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:17` 504:shared_fH
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:115` 3413:shared_fH
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:117` 3415:shared_fH

## FMT-SEARCH-HISTORY-PREFERENCES

- Title: Search history and preference persistence
- Confidence: `high`
- Notes: Search history controls and related UX preference keys.
- Evidence snippets:
  - symbol_hits: `2`
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:714` (undefined) external QSettings::setValue(QAnyStringView, QVariant const&) (from QtCore)
    - `evidence/ui_audit/decomp/nm_demangled_full.txt:994` (undefined) external QSettings::value(QAnyStringView) const (from QtCore)
  - method_hits: `3`
    - `evidence/ui_audit/decomp/mainwindow_methods.txt:37` MainWindow::onClearSearchHistoryClicked()
    - `evidence/ui_audit/decomp/mainwindow_methods.txt:114` MainWindow::onShowPrivateSearchResultsClicked(bool)
    - `evidence/ui_audit/decomp/mainwindow_methods.txt:123` MainWindow::onUseOldStyleSearchResultsCheckboxToggled(bool)
  - string_hits: `8`
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:528` 11152:1onClearSearchHistoryClicked()
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:643` 11396:clearSearchHistoryButton
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:713` 11530:Clear Search History
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:1743` 19224:onClearSearchHistoryClicked
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:2320` 22009:1onClearSearchHistoryClicked()
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:2435` 22253:clearSearchHistoryButton
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:2505` 22387:Clear Search History
    - `evidence/ui_audit/ui_strings_feature_candidates.txt:3535` 30090:onClearSearchHistoryClicked

