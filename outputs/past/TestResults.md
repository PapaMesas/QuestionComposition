# テスト結果 — 設問作成支援ツール

実施日: 2026-04-30

---

## 実行コマンド

```
cargo test
```

---

## 単体テスト結果 (Unit Tests)

| ID | テスト名 | 結果 |
|---|---|---|
| UT-01 | `encrypt_returns_non_empty_string` | ✅ PASS |
| UT-02 | `decrypt_reverses_encrypt` | ✅ PASS |
| UT-03 | `decrypt_invalid_input_returns_error` | ✅ PASS |
| UT-04 | `encrypt_different_inputs_produce_different_outputs` | ✅ PASS |
| UT-05 | `reads_subject_correctly` | ✅ PASS |
| UT-06 | `reads_correct_question_count` | ✅ PASS |
| UT-07 | `reads_question_fields_correctly` | ✅ PASS |
| UT-08 | `default_num_choices_is_four` | ✅ PASS |
| UT-09 | `skips_empty_rows` | ✅ PASS |
| UT-10 | `returns_error_for_nonexistent_file` | ✅ PASS |
| UT-11 | `parses_plain_json_array` | ✅ PASS |
| UT-12 | `parses_json_array_inside_code_block` | ✅ PASS |
| UT-13 | `returns_error_when_no_json_array` | ✅ PASS |
| UT-14 | `correct_answer_inserted_at_varying_positions` | ✅ PASS |
| UT-15 | `choices_count_matches_num_choices` | ✅ PASS |
| UT-11b | `generate_choices_uses_llm_response` (モッククライアント) | ✅ PASS |
| UT-16 | `default_ruleset_content_is_not_empty` | ✅ PASS |
| UT-17 | `loads_custom_md_file` | ✅ PASS |
| UT-18 | `returns_error_for_nonexistent_md_file` | ✅ PASS |

**小計: 19 / 19 PASS**

---

## 統合テスト結果 (Integration Tests)

| ID | テスト名 | 結果 |
|---|---|---|
| IT-01 | `api_key_roundtrip_through_config_file` | ✅ PASS |
| IT-02 | `xlsx_roundtrip_preserves_data` | ✅ PASS |

**小計: 2 / 2 PASS**

---

## 静的解析

| チェック | 結果 |
|---|---|
| `cargo clippy -- -D warnings` | ✅ エラーなし |
| `cargo fmt --check` | ✅ エラーなし |
| `cargo build --release` | ✅ 成功 |

---

## 総合結果

| 区分 | 件数 | 合格 | 不合格 |
|---|---|---|---|
| 単体テスト (UT) | 19 | 19 | 0 |
| 統合テスト (IT) | 2 | 2 | 0 |
| **合計** | **21** | **21** | **0** |

**判定: ✅ 全テスト合格 — リリース可**

---

## 備考

- GUI手動テスト (GT-01〜GT-25) は実際のAPIキーが必要なため、別途実施が必要
- LLM API呼び出し部分 (生成テスト) はモッククライアントで代替済み
