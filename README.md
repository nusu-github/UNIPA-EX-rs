<div align="center">

  <h1><code>UNIPA-EX-rs</code></h1>

<strong>UNIVERSAL PASSPORT EXのHTMLページを解析するRust + WebAssemblyライブラリ</strong>

  <p>
    <img src="https://img.shields.io/badge/rust-stable-brightgreen.svg?style=flat-square" alt="Rust Version" />
    <img src="https://img.shields.io/badge/wasm--pack-ready-orange.svg?style=flat-square" alt="WASM Pack Ready" />
  </p>

  <h3>
    <a href="#使用方法">使用方法</a>
    <span> | </span>
    <a href="#機能">機能</a>
    <span> | </span>
    <a href="#開発">開発</a>
  </h3>

<sub>🦀 Rust + 🕸 WebAssembly で構築</sub>

</div>

## 概要

`UNIPA-EX-rs`は、UNIVERSAL PASSPORT EXのHTMLページを解析し、構造化されたデータとして抽出するためのRustライブラリです。WebAssemblyにコンパイルされ、JavaScriptから利用できます。

### 主な特徴

- **包括的なページ解析**: UNIVERSAL PASSPORT EXの主要機能ページに対応
- **型安全**: Rustの型システムによる安全なデータ処理
- **WebAssembly対応**: ブラウザ環境での高速実行
- **構造化データ**: JSONとして利用可能な構造化された出力

## 対応機能

### 📚 学習関連

- **成績照会**: 個別科目の成績とGPA情報
- **シラバス**: 検索・詳細表示・授業計画
- **課題提出**: 提出状況と課題一覧
- **小テスト**: 実施状況と結果管理

### 📅 時間割関連

- **学生時間割**: 履修科目と単位取得状況
- **教員時間割**: 担当授業と検索機能
- **学科開講一覧**: 提供科目の管理

### 🏫 大学生活

- **ポータル**: メイン画面の各種情報
- **教室利用状況**: 予約・検索機能
- **学籍情報照会**: 基本情報と所属情報
- **アンケート**: 回答状況管理

### 📊 判定・予測

- **卒業見込判定**: 卒業要件の充足状況
- **進級見込判定**: 進級要件の確認
- **免許取得見込判定**: 教員免許等の取得見込

## 🚴 使用方法

### 📦 インストール

```bash
npm install unipa_ex
```

### 💻 基本的な使用例

```javascript
import init, { StudentTimetableParser } from "unipa_ex";

async function parseStudentTimetable() {
  // WebAssemblyモジュールを初期化
  await init();

  // パーサーを作成
  const parser = new StudentTimetableParser();

  // HTMLを解析
  const htmlContent = `<!-- UNIVERSAL PASSPORT EXの学生時間割HTML -->`;
  const timetable = parser.list_view(htmlContent);

  console.log("履修科目:", timetable.schedule);
  console.log("単位取得状況:", timetable.credit_status);
}
```

### 🔧 各種パーサーの使用例

```javascript
import init, {
  PortalParser,
  GradeInquiryParser,
  SyllabusSearchParser,
  StudentInfoInquiryParser,
} from "unipa_ex";

await init();

// ポータル画面の解析
const portalParser = new PortalParser();
const portalData = portalParser.parse(htmlContent);

// 成績照会の解析
const gradeParser = new GradeInquiryParser();
const gradeData = gradeParser.parse(htmlContent);

// シラバス検索結果の解析
const syllabusParser = new SyllabusSearchParser();
const syllabusData = syllabusParser.search_result(htmlContent);

// 学籍情報の解析
const studentInfoParser = new StudentInfoInquiryParser();
const studentInfo = studentInfoParser.student_info_inquiry(htmlContent);
```

## 🛠️ 開発

### 前提条件

- Rust (最新安定版)
- wasm-pack
- Node.js (開発・テスト用)

### ビルド

```bash
# WebAssemblyパッケージをビルド
wasm-pack build

# 開発用ビルド（デバッグ情報付き）
wasm-pack build --dev
```

### テスト

```bash
# Rustテストを実行
cargo test

# ブラウザテストを実行
wasm-pack test --headless --firefox
```


## 📁 プロジェクト構造

```
src/
├── common/           # 共通機能とトレイト
├── parsers/          # 各機能のパーサー
│   ├── portal/       # ポータル画面
│   ├── grades/       # 成績関連
│   ├── timetable/    # 時間割関連
│   ├── syllabus/     # シラバス
│   ├── assignment_submit/  # 課題提出
│   ├── quiz_answer_list/   # 小テスト
│   ├── classroom_usage/    # 教室利用
│   ├── student_info_inquiry/ # 学籍情報
│   ├── questionnaire/      # アンケート
│   └── course_evaluation/  # 授業評価
├── utils/            # ユーティリティ関数
└── lib.rs           # ライブラリエントリポイント
```

## 🔋 含まれる機能

- **wasm-bindgen**: WebAssemblyとJavaScript間の通信
- **scraper**: HTML解析エンジン
- **serde**: シリアライゼーション
- **tsify**: TypeScript型定義生成
- **regex-lite**: 軽量正規表現エンジン

## 📝 ライセンス

このプロジェクトは以下のライセンスの下で公開されています：

- Apache License 2.0 ([LICENSE_APACHE](LICENSE_APACHE))

## 🤝 コントリビューション

プルリクエストやイシューの報告を歓迎します。開発に参加する際は、以下のガイドラインに従ってください：

1. **コードスタイル**: `cargo fmt`でフォーマット
2. **テスト**: 新機能には適切なテストを追加
3. **ドキュメント**: 公開APIには日本語でのドキュメントを記述
4. **型安全性**: 可能な限り型安全なAPIを設計

## 📞 サポート

- バグ報告: [GitHub Issues](https://github.com/nusu-github/UNIPA-EX-rs/issues)
- 機能要望: [GitHub Discussions](https://github.com/nusu-github/UNIPA-EX-rs/discussions)

---

<div align="center">
  <sub>UNIVERSAL PASSPORT EXをより使いやすく 🎓</sub>
</div>
