# Windows DLL 依存関係調査報告

調査日: 2026-04-30  
対象ファイル: `question_composition.exe`  
ターゲット: `x86_64-pc-windows-gnu` (64ビット Windows)

---

## 調査方法

```bash
x86_64-w64-mingw32-objdump -p question_composition.exe | grep "DLL Name"
```

---

## 依存 DLL 一覧

| DLL 名 | 分類 | 説明 |
|---|---|---|
| `KERNEL32.dll` | Windows コア | プロセス・スレッド・メモリ管理 |
| `ntdll.dll` | Windows コア | NT カーネルインターフェース |
| `USER32.dll` | Windows コア | ウィンドウ・メッセージ管理 |
| `ADVAPI32.dll` | Windows コア | レジストリ・セキュリティ |
| `SHELL32.dll` | Windows コア | シェル操作 |
| `SHLWAPI.dll` | Windows コア | シェルユーティリティ |
| `GDI32.dll` | グラフィック | 2D 描画 |
| `OPENGL32.dll` | グラフィック | OpenGL（egui の GPU レンダリング） |
| `dwmapi.dll` | グラフィック | デスクトップウィンドウマネージャー |
| `uxtheme.dll` | グラフィック | ビジュアルスタイル |
| `imm32.dll` | 入力 | Input Method Manager（日本語入力） |
| `uiautomationcore.dll` | アクセシビリティ | UI オートメーション |
| `ole32.dll` | COM | OLE / COM 基盤 |
| `oleaut32.dll` | COM | OLE オートメーション |
| `ws2_32.dll` | ネットワーク | Winsock（LLM API 通信） |
| `bcrypt.dll` | 暗号化 | 暗号化プリミティブ（AES-GCM） |
| `bcryptprimitives.dll` | 暗号化 | 暗号化プリミティブ |
| `crypt32.dll` | 暗号化 | 証明書・TLS |
| `ncrypt.dll` | 暗号化 | 次世代暗号化 |
| `secur32.dll` | 暗号化 | セキュリティ |
| `userenv.dll` | 環境 | ユーザー環境変数・プロファイル |
| `ktmw32.dll` | トランザクション | カーネルトランザクションマネージャー |
| `api-ms-win-crt-runtime-l1-1-0.dll` | UCRT | C ランタイム（プロセス制御） |
| `api-ms-win-crt-stdio-l1-1-0.dll` | UCRT | C ランタイム（標準入出力） |
| `api-ms-win-crt-string-l1-1-0.dll` | UCRT | C ランタイム（文字列処理） |
| `api-ms-win-crt-math-l1-1-0.dll` | UCRT | C ランタイム（数学関数） |
| `api-ms-win-crt-heap-l1-1-0.dll` | UCRT | C ランタイム（ヒープ管理） |
| `api-ms-win-crt-locale-l1-1-0.dll` | UCRT | C ランタイム（ロケール） |
| `api-ms-win-crt-environment-l1-1-0.dll` | UCRT | C ランタイム（環境変数） |
| `api-ms-win-crt-private-l1-1-0.dll` | UCRT | C ランタイム（内部） |
| `api-ms-win-core-path-l1-1-0.dll` | UCRT | パス操作 |
| `api-ms-win-core-synch-l1-2-0.dll` | UCRT | 同期プリミティブ |
| `api-ms-win-core-winrt-error-l1-1-0.dll` | UCRT | WinRT エラー処理 |

---

## 調査結果サマリー

### 外部再配布 DLL

**なし。** すべての依存 DLL は Windows OS に標準搭載のシステム DLL です。

### MinGW 固有 DLL

`libgcc_s_seh-1.dll`、`libstdc++-6.dll` 等の MinGW ランタイム DLL は**依存していません**。  
Rust コンパイラが GNU ランタイムを静的リンクしているためです。

---

## OS バージョン別の動作可否

| OS | 動作 | 備考 |
|---|---|---|
| Windows 11 | ✅ 単体動作可 | UCRT 標準搭載 |
| Windows 10 | ✅ 単体動作可 | UCRT 標準搭載 |
| Windows 8.1 | ⚠️ 要確認 | Windows Update (KB2999226) が必要な場合あり |
| Windows 7 | ⚠️ 要確認 | Windows Update (KB2999226) が必要な場合あり |

---

## 結論

**`question_composition.exe` は Windows 10 / 11 であれば単体で動作します。**  
追加ランタイムや DLL の同梱は不要です。
