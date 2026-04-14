# AI 奧瑞岡式辯論系統｜技術文件 v1

## 1. 文件資訊

* **文件名稱**：AI 奧瑞岡式辯論系統 技術文件
* **版本**：v1.0
* **文件角色**：Product Manager 起草
* **適用對象**：Frontend、Backend、設計、測試、未來維運者
* **專案代號**：Agora Debate Theater

---

## 2. 專案摘要

本專案旨在打造一套以 **奧瑞岡式辯論** 為核心流程的多代理 AI 辯論系統。使用者可指定辯題、辯論定義、正反雙方立場與辯士角色，由系統驅動多位 AI 辯士依規則完成申論、質詢、答辯與結辯，並由人類裁判全程觀戰、記錄與最終裁決。

第一版系統聚焦於以下能力：

1. 支援單一辯題建立完整辯論場次
2. 支援正反雙方各三位 AI 辯士
3. 依奧瑞岡式規則推進回合
4. 支援人類裁判筆記與最終判決
5. 提供可視化舞台、事件紀錄與重播能力
6. 以地端模型為主要推理來源

本專案將採用：

* **後端**：Rust
* **前端**：React
* **Monorepo 管理**：Nx
* **地端模型運行方式**：Ollama HTTP API
* **第一版模型策略**：單一主模型或最多兩顆小模型

---

## 3. 產品目標

### 3.1 核心目標

本專案第一階段目標是交付一套 **可運作、可觀戰、可重播、可裁決** 的 AI 辯論系統 MVP。

系統需具備以下特性：

* 使用者可設定辯題與題目定義
* 系統可依既定賽制驅動辯論流程
* 使用者可從「上帝視角」觀看整場辯論
* 所有發言、階段與裁判行為皆可被記錄
* 場次結束後可查看結果並重播整場內容

### 3.2 商業與展示價值

本產品具備以下潛在用途：

* AI 多代理互動展示
* 辯論與論證研究
* 產品展示與 Demo Day
* 教育或訓練工具
* 模型人格與立場控制實驗平台

---

## 4. 非目標（Out of Scope）

以下功能不屬於第一版範圍：

1. 語音輸入 / TTS 配音
2. 真正同步並行的多模型運算
3. 多房間多人連線對戰
4. AI 自動裁判勝負
5. 複雜長期記憶向量資料庫
6. 影像角色 Live2D / 3D avatar
7. 內建影片剪輯功能
8. 高度動態規則編輯器

---

## 5. 使用者與角色

### 5.1 主要使用者

#### A. 裁判 / 主控使用者

* 建立辯論場次
* 設定辯題與規則
* 啟動、暫停、逐輪推進辯論
* 在過程中做筆記
* 於結束後判定勝負

#### B. 觀戰使用者（未來可延伸）

* 觀看辯論流程
* 查看逐輪發言與爭點
* 播放重播內容

### 5.2 系統角色

#### A. Affirmative Team（正方）

* 正一辯
* 正二辯
* 正三辯

#### B. Negative Team（反方）

* 反一辯
* 反二辯
* 反三辯

#### C. System Orchestrator

* 控制賽制流程
* 組裝 prompt
* 呼叫模型
* 更新辯論狀態
* 記錄事件

---

## 6. 題目與定義

### 6.1 第一版預設辯題

**AI 是否可能完整取代人類工作**

### 6.2 第一版預設題目定義

本題所稱「AI 可能完整取代人類工作」，係指在可預見之技術與制度條件下，AI 及其自動化系統足以承擔大多數可經濟交易、可標準化、可流程化工作之核心職能，使人類不再作為該等工作的主要常態執行者。

### 6.3 題目定義要求

系統需支援：

* 使用者手動輸入題目定義
* 題目定義於場次建立後鎖定
* 所有辯士 prompt 均引用同一份辯題定義
* 前端固定顯示「題目定義區」

---

## 7. 辯論賽制設計

### 7.1 第一版採用賽制

* **模式**：奧瑞岡式（Oregon-style）三對三辯論
* **隊伍數**：2 隊（正方 / 反方）
* **每隊辯士數**：3 位
* **裁判**：1 位人類使用者

### 7.2 辯論流程（MVP 版）

第一版將採用「**固定順序 + 可展示化縮時**」設計。

建議流程如下：

1. 正一申論
2. 反二質詢正一
3. 反一申論
4. 正三質詢反一
5. 正二申論
6. 反三質詢正二
7. 反二申論
8. 正一質詢反二
9. 正三申論
10. 反一質詢正三
11. 反三申論
12. 正二質詢反三
13. 結辯（順序可固定於 MVP）

### 7.3 時間控制

第一版支援兩種模式：

#### A. 正式模式

* 每段 3 分鐘

#### B. Demo 模式

* 每段 30–60 秒
* 適用於展示與錄製

### 7.4 規則限制

* 發言內容需維持本方立場一致
* 結辯不得提出全新主論點
* 質詢階段以問答形式為主
* 系統需將每輪階段類型明確標記

---

## 8. 產品功能需求

### 8.1 場次建立

使用者可建立一場新的辯論，包含：

* 辯題
* 題目定義
* 模式（正式 / Demo）
* 模型設定
* 正反隊伍設定
* 是否自動推進回合

### 8.2 辯論舞台

主舞台需顯示：

* 辯題
* 題目定義
* 當前階段
* 倒數計時
* 目前發言者
* 逐輪 transcript
* 爭點摘要
* 裁判控制區

### 8.3 裁判控制台

裁判需可執行：

* Start
* Pause
* Resume
* Next Turn
* Auto Run
* Add Note
* Mark Key Point
* Declare Winner

### 8.4 辯論重播

系統需提供：

* 逐輪播放
* 快轉 / 倍速
* 跳轉到指定回合
* 顯示該輪發言者與內容

### 8.5 匯出與記錄

第一版需支援匯出：

* `transcript.md`
* `events.jsonl`
* `session.json`
* `judge-decision.json`

---

## 9. 視覺化設計要求

### 9.1 主畫面布局

#### 左欄：正方隊伍

* 辯士卡片
* 名稱 / 角色 / 模型
* 當前狀態（待命 / 發言中 / 已完成）

#### 中欄：辯論舞台

* 當前階段標題
* 倒數計時器
* 發言內容流
* 爭點摘要
* 系統提示訊息

#### 右欄：反方隊伍 + 裁判區

* 反方辯士資訊
* 裁判筆記輸入區
* 回合控制按鈕
* 最終判決區

#### 底部：時間軸

* 各回合節點
* 可跳轉 / 高亮
* 顯示關鍵回合標記

### 9.2 必要視覺狀態

需明確設計以下 UI 狀態：

* loading
* generating response
* paused
* replaying
* completed
* error

---

## 10. 技術架構概覽

### 10.1 Monorepo 結構

```text
/apps
  /web
  /api
/packages
  /shared-types
  /prompt-spec
  /ui
  /debate-protocol
/tools
  /scripts
```

### 10.2 架構說明

#### apps/web

React 前端應用，負責：

* 建立場次
* 顯示辯論舞台
* 裁判控制台
* 重播頁

#### apps/api

Rust 後端，負責：

* Orchestrator
* State management
* Ollama model gateway
* Event recording
* WebSocket / SSE 推播

#### packages/shared-types

共用型別定義與 schema contract

#### packages/prompt-spec

集中管理 prompt template 與角色模板

#### packages/ui

可重用 UI 元件

#### packages/debate-protocol

賽制、回合、事件格式與流程規則定義

---

## 11. 模型調用策略

### 11.1 第一版模型原則

第一版不採用「每位辯士一顆模型」，而採用：

* **1 顆主模型**：所有辯士共用
* **可選第 2 顆模型**：摘要 / 規則檢查專用

### 11.2 推薦模型配置

#### 基礎版

* `qwen3:4b`

#### 進階版

* `qwen3:4b`：辯士發言
* `gemma3:4b` 或同型號第二顆：摘要與規則檢查

### 11.3 調用方式

後端透過 Ollama HTTP API 調用地端模型：

* `POST /api/chat`
* `stream: false`
* 統一要求 JSON schema output
* 使用 `keep_alive` 保持模型駐留記憶體

### 11.4 為何不用多顆常駐模型

原因如下：

* 第一版主要瓶頸不在模型數量，而在賽制與事件流
* 多顆模型會增加資源壓力與除錯難度
* 同模型不同 persona 已足夠實現角色差異

---

## 12. Prompt 與角色設計

### 12.1 Prompt 組成

每輪 prompt 由以下部分組成：

1. 固定系統規則
2. 辯題與定義
3. 當前隊伍立場
4. 該辯士角色職責
5. 目前公開對話摘要
6. 本輪任務
7. JSON 輸出要求

### 12.2 辯士角色模板

每位辯士至少包含：

* name
* side
* role
* persona
* objective
* style
* prohibited_behavior

### 12.3 結構化輸出要求

每輪模型回傳需至少包含：

* `public_speech`
* `key_claims`
* `attack_targets`
* `defense_targets`
* `round_summary`

---

## 13. 後端系統設計

### 13.1 核心模組

#### A. Debate Orchestrator

負責：

* 建立場次
* 決定下一輪發言者
* 組裝 prompt
* 呼叫模型
* 產生事件
* 更新 session state

#### B. Debate State Store

負責：

* 保存 session
* 保存回合與事件
* 保存裁判筆記與決策

#### C. Model Gateway

負責：

* 對接 Ollama
* 統一模型請求與回應格式
* 錯誤處理與 retry

#### D. Recorder

負責：

* 產生 transcript
* 寫入 jsonl event log
* 保存 replay 所需快照

#### E. Realtime Stream

負責：

* 向前端推送最新回合事件

### 13.2 後端建議模組切分

```text
/apps/api/src
  /domain
  /application
  /infrastructure
  /interfaces
```

#### domain

* debate_session
* debate_round
* debater_agent
* judge_decision

#### application

* run_turn_use_case
* start_session_use_case
* declare_winner_use_case

#### infrastructure

* ollama_client
* file_store
* sqlite_store（可後續加入）

#### interfaces

* http routes
* websocket handlers

---

## 14. 前端系統設計

### 14.1 頁面規劃

#### A. 場次建立頁

* 建立辯題
* 設定辯題定義
* 選擇模型
* 選擇模式

#### B. 辯論舞台頁

* 主舞台
* 雙方隊伍面板
* 裁判控制台
* transcript

#### C. 重播頁

* 時間軸
* 逐輪播放
* 回合詳情

### 14.2 前端狀態管理

需追蹤以下狀態：

* session metadata
* current phase
* current speaker
* countdown timer
* transcript events
* judge notes
* replay state

### 14.3 前端元件建議

* `DebaterCard`
* `DebateTimeline`
* `SpeechPanel`
* `JudgePanel`
* `DefinitionPanel`
* `RoundStatusBanner`
* `ReplayControls`

---

## 15. 主要資料模型

### 15.1 DebateSession

```ts
export type DebateSession = {
  id: string
  motion: string
  definition: string
  format: 'oregon-3v3'
  mode: 'formal' | 'demo'
  status: 'draft' | 'running' | 'paused' | 'finished'
  currentRound: number
  currentPhase: string
  affirmativeTeamId: string
  negativeTeamId: string
  judgeId: string
  createdAt: string
  updatedAt: string
}
```

### 15.2 DebaterAgent

```ts
export type DebaterAgent = {
  id: string
  sessionId: string
  side: 'affirmative' | 'negative'
  role: 'first' | 'second' | 'third' | 'closer'
  displayName: string
  persona: string
  objective: string
  style: string
  model: string
}
```

### 15.3 DebateEvent

```ts
export type DebateEvent = {
  id: string
  sessionId: string
  round: number
  phase: string
  speakerId: string
  targetId?: string
  kind: 'speech' | 'question' | 'answer' | 'system' | 'judge_note'
  content: string
  meta?: Record<string, unknown>
  startedAt: string
  endedAt?: string
}
```

### 15.4 JudgeDecision

```ts
export type JudgeDecision = {
  sessionId: string
  winner: 'affirmative' | 'negative'
  reasoning: string
  notesByRound: Array<{
    round: number
    note: string
  }>
  decidedAt: string
}
```

---

## 16. API 初版需求

### 16.1 Session API

* `POST /sessions`
* `GET /sessions/:id`
* `POST /sessions/:id/start`
* `POST /sessions/:id/pause`
* `POST /sessions/:id/resume`
* `POST /sessions/:id/next-turn`
* `POST /sessions/:id/declare-winner`

### 16.2 Event API

* `GET /sessions/:id/events`
* `GET /sessions/:id/replay`

### 16.3 Judge API

* `POST /sessions/:id/judge-notes`
* `GET /sessions/:id/judge-decision`

### 16.4 Realtime

* `WS /sessions/:id/stream`

---

## 17. 錄製與重播策略

### 17.1 第一版錄製策略

第一版分為兩層：

#### A. 結構化錄製（必要）

* `events.jsonl`
* `transcript.md`
* `session.json`
* `judge-decision.json`

#### B. 視覺錄影（外部工具先行）

* 使用者可搭配 macOS 錄影或 OBS 錄製 UI 畫面

### 17.2 Replay 要求

Replay 不重新呼叫模型，而是播放已記錄事件。

系統需支援：

* 基於 event log 重建時間軸
* 任意跳輪
* 逐輪顯示發言內容

---

## 18. 錯誤處理與可觀測性

### 18.1 錯誤情境

需處理至少以下錯誤：

* 模型無回應
* 模型輸出無法解析 JSON
* 回合中斷
* 檔案寫入失敗
* WebSocket 斷線

### 18.2 錯誤策略

* retry 機制
* 記錄錯誤事件
* 場次保持可恢復狀態
* 對前端提供可理解錯誤訊息

### 18.3 日誌要求

需至少記錄：

* request id
* session id
* round id
* model name
* generation time
* parse result
* failure reason

---

## 19. 驗收標準（MVP）

以下條件全部成立，方視為 MVP 可驗收：

1. 可建立一場新辯論
2. 可依固定賽制跑完整場正反三對三辯論
3. 所有回合可被正確記錄
4. 前端可顯示當前發言者與階段
5. 裁判可新增筆記並最終宣判
6. 系統可匯出 transcript 與 jsonl
7. 系統可進行重播
8. 模型失敗時不致使整場資料消失

---

## 20. 開發 Todo List

### P0：可開工骨架

* [ ] 建立 Nx workspace
* [ ] 建立 React app
* [ ] 建立 Rust backend app
* [ ] 將 Rust targets 整合進 Nx
* [ ] 建立 shared-types 套件
* [ ] 建立 debate-protocol 套件
* [ ] 建立 prompt-spec 套件
* [ ] 實作 session 建立 API
* [ ] 實作 Ollama client
* [ ] 實作 run turn use case
* [ ] 實作 event log writer
* [ ] 實作最小舞台頁
* [ ] 實作裁判宣判

### P1：完整流程

* [ ] 實作全場辯論流程
* [ ] 實作倒數計時 UI
* [ ] 實作時間軸
* [ ] 實作 replay
* [ ] 實作 transcript export
* [ ] 實作 judge notes 儲存
* [ ] 實作錯誤恢復流程

### P2：展示增強

* [ ] 實作關鍵爭點摘要
* [ ] 實作回合高亮
* [ ] 實作更好的 loading / generating animation
* [ ] 支援第二顆模型做摘要 / 檢查
* [ ] 匯出 replay package

---

## 21. 風險與注意事項

### 21.1 主要風險

* 小模型可能論證不穩定
* prompt 若定義不夠精準，角色會混亂
* JSON schema 若設計過細，模型可能頻繁格式失敗
* 若回合過長，展示節奏可能不佳

### 21.2 緩解策略

* 先以 Demo 模式縮短回合
* 結構化輸出只保留必要欄位
* 將 prompt 版本化
* 將辯題定義固定顯示於前端
* 對單輪 generation 設 timeout 與 retry

---

## 22. 後續擴充方向

### 第二期候選功能

* 多辯題模板
* AI 評論員 / 旁白員
* 不同模型混搭
* 辯士人格庫
* 觀眾模式
* 多場次比較
* 視覺錄製內建功能
* 統計報表與對戰分析

---

## 23. 結論

本文件定義了第一版 AI 奧瑞岡式辯論系統的產品目標、技術方向與可交付範圍。第一版成功的標準不在於模型數量或規模，而在於：

1. 是否能正確執行一場完整辯論
2. 是否能提供清晰的觀戰體驗
3. 是否能完整記錄與重播
4. 是否能讓人類裁判做出可追溯判決

因此本專案將優先投入於：

* 賽制正確性
* 事件流與結構化記錄
* 地端模型穩定調用
* 可視化體驗

在此基礎上，再逐步擴充模型數量、角色複雜度與展示效果。
