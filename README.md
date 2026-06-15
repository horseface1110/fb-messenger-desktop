# Messenger Desktop

Messenger Desktop 是一個非官方的 Facebook Messenger 桌面版包裝器。
它做的事情很單純：用 Tauri 建立一個桌面 App 視窗，直接載入官方的
Messenger 網頁：

```text
https://www.messenger.com
```

換句話說，它比較像是「把 Messenger 網頁固定在一個桌面 App 裡」，
並加上一些桌面使用體驗，例如系統匣、關閉到背景、快速顯示/隱藏。

## 使用者需要安裝什麼嗎？

一般使用者不需要安裝 Node.js、Rust、Cargo、npm 或任何開發工具。
那些只是在開發與打包這個 App 時才需要。

Windows 使用者可以直接安裝 `.exe` 或 `.msi`。
macOS 使用者可以直接安裝 `.dmg` 或 `.app`。

注意事項：

- Windows 需要 Microsoft Edge WebView2 Runtime。Windows 11 通常已內建，新的 Windows 10 多數也已安裝。
- macOS 使用系統內建 WebKit，不需要額外瀏覽器核心。
- 目前版本未做商業簽章，Windows SmartScreen 或 macOS Gatekeeper 可能會跳安全提醒。

## 安全性說明

這個 App 的設計目標是：不要接觸、儲存、轉送你的 Facebook 帳號密碼。

登入流程發生在官方網站：

```text
https://www.messenger.com
```

當你輸入帳號密碼時，畫面其實是 Messenger 官方網頁。帳號密碼送往
Facebook / Meta 的伺服器，而不是送到這個專案作者的伺服器。

這個專案沒有自己的登入伺服器，也沒有自建後端 API 來接收你的帳密。
它沒有實作 Facebook 登入流程，沒有保存 Messenger token，也沒有呼叫
Facebook 私有 API。

## 這個 App 的原理

Messenger Desktop 使用 Tauri v2 製作。

Tauri App 大致分成兩層：

- 桌面外殼：Rust / Tauri，負責視窗、系統匣、快捷鍵、設定檔。
- 網頁內容：系統 WebView，直接開啟 `https://www.messenger.com`。

在 Windows 上，WebView 使用 Microsoft Edge WebView2。
在 macOS 上，WebView 使用系統 WebKit。

這代表 Messenger 的登入狀態、Cookie、Session 都存在作業系統 WebView
自己的資料裡，而不是存在本專案的伺服器裡。

本 App 目前的本機設定只包含這類內容：

```json
{
  "start_on_login": false,
  "close_to_tray": true,
  "start_minimized": false,
  "shortcut": "Ctrl+Shift+M",
  "messenger_url": "https://www.messenger.com"
}
```

這些設定用來控制 App 行為，不包含你的 Facebook 密碼。

## 我怎麼確認它沒有偷密碼？

你可以用幾個角度檢查：

- 專案是開源的，可以直接看程式碼。
- 主要視窗設定在 `src-tauri/tauri.conf.json`，可以看到它載入的是 `https://www.messenger.com`。
- Rust 程式碼在 `src-tauri/src/`，主要處理系統匣、快捷鍵、設定讀寫。
- 前端設定頁在 `src/settings.ts`，只處理本機設定，不處理 Facebook 帳密。
- 專案沒有提供任何自建登入 API，也沒有把密碼送到第三方伺服器的程式碼。

如果你是從 Release 下載安裝檔，最安全的方式是確認該 Release 是由這個
GitHub 專案的 Actions 自動打包產生，而不是來路不明的第三方檔案。

## 隱私與限制

這個 App 不會主動讀取你的 Messenger 訊息內容，也不會把訊息上傳到作者伺服器。
不過，Messenger 網頁本身仍然是 Facebook / Meta 的服務，所以它原本會收集什麼資料，
仍然依照 Meta 的服務條款與隱私政策。

這個 App 是非官方工具，和 Meta / Facebook 沒有合作、授權或背書關係。

## 功能

- 直接開啟官方 Messenger Web。
- 支援登入後保持 WebView session。
- 系統匣選單：Show、Hide、Reload Messenger、Settings、Quit。
- 關閉視窗時可隱藏到系統匣。
- `Ctrl + Shift + M` 快速顯示/隱藏。
- 本機設定頁。
- 可打包成 Windows 與 macOS 安裝檔。

## 安裝包

Windows 打包後的安裝檔會在：

```text
src-tauri/target/release/bundle/
```

常見輸出：

```text
src-tauri/target/release/bundle/nsis/Messenger Desktop_0.1.2_x64-setup.exe
src-tauri/target/release/bundle/msi/Messenger Desktop_0.1.2_x64_en-US.msi
```

一般 Windows 使用者建議下載 `.exe` 安裝包。

## 開發需求

如果你只是安裝使用，不需要這些。
如果你要自己改程式或重新打包，才需要：

- Node.js and npm
- Rust and Cargo through rustup
- Microsoft C++ Build Tools
- Microsoft Edge WebView2 Runtime

## 開發指令

如果 PowerShell 因執行政策擋住 `npm.ps1`，請使用 `npm.cmd`。

```powershell
npm.cmd install
npm.cmd run icons
npm.cmd run build
npm.cmd run tauri -- dev
npm.cmd run tauri -- build
```

## 修改 App 圖示

App 圖示的來源圖在：

```text
src-tauri/icons/app-icon.png
```

想要更換桌面縮圖、工具列圖示、安裝程式圖示、Windows `.ico` 或 macOS `.icns`
時，請先替換這個檔案。

建議圖片格式：

- PNG
- 正方形，建議 `1024x1024`
- 背景透明
- 圓角要做在圖案本身，圓角外面要是透明像素
- 不要在圖示後方留下白色方形背景

如果 Windows 桌面上看到白色方框，通常代表 `app-icon.png` 四角有不透明白色像素。
請把圓角外側的白底移除，但保留圖示內原本需要的白色圖案，例如聊天泡泡。

原本白底版本的備份在：

```text
src-tauri/icons/app-icon-with-white-bg.png
```

替換 `app-icon.png` 後，重新產生平台圖示：

```powershell
npm.cmd run icons
```

這會更新：

```text
src-tauri/icons/icon.ico
src-tauri/icons/icon.icns
src-tauri/icons/32x32.png
src-tauri/icons/128x128.png
src-tauri/icons/128x128@2x.png
```

接著重新打包：

```powershell
npm.cmd run tauri -- build
```

如果安裝新版後 Windows 還顯示舊圖示，請關閉 App、取消釘選舊的工具列項目、
重新安裝新版，然後重啟 Windows Explorer 或重新開機。Windows 會積極快取桌面
與工具列圖示。

## 發佈到 GitHub Releases

這個專案包含 GitHub Actions workflow：

```text
.github/workflows/release.yml
```

它會用原生 runner 打包：

- Windows x64 on `windows-latest`
- macOS Intel on `macos-13`
- macOS Apple Silicon on `macos-14`

第一次推到 GitHub：

```powershell
git init
git add .
git commit -m "Initial Messenger Desktop app"
git branch -M main
git remote add origin <your-github-repo-url>
git push -u origin main
```

建立 release tag：

```powershell
git tag v0.1.2
git push origin v0.1.2
```

GitHub Actions 跑完後，會建立一個 draft release。
確認附件沒有問題後，再手動按 Publish release。

## 簽章狀態

目前產出的安裝包是未簽章版本。

這代表：

- Windows 可能出現 SmartScreen 警告。
- macOS 可能出現 Gatekeeper 警告。

如果要讓一般使用者看到更少安全警告，需要：

- Windows code signing certificate
- Apple Developer 帳號
- macOS signing and notarization

這些是發佈層級的簽章流程，和 App 是否會竊取密碼是不同問題。
本專案的安全核心仍然是：登入畫面直接載入官方 Messenger 網頁，不經過作者伺服器。
