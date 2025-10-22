<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  let downloadDir: string = "";
  let progress: string = "";

  const qualities = [
    { label: "Best Quality", value: "best" },
    { label: "Audio Only", value: "bestaudio" },
    { label: "480p", value: "bestvideo[height<=480]+bestaudio/best" },
    { label: "720p HD", value: "bestvideo[height<=720]+bestaudio/best" },
    { label: "1080p Full HD", value: "bestvideo[height<=1080]+bestaudio/best" },
    { label: "1440p 2K", value: "bestvideo[height<=1440]+bestaudio/best" },
    { label: "2160p 4K", value: "bestvideo[height<=2160]+bestaudio/best" },
    { label: "Lowest Quality", value: "worst" },
  ];

  async function chooseFolder() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Select Download Folder",
    });
    if (typeof selected === "string") {
      downloadDir = selected;
    }
  }
  let url = "";
  let quality = "bestvideo+bestaudio/best";
  let status = "";
  let downloading = false;

  onMount(() => {
    const unlisten = listen<string>("download_progress", (event) => {
      progress = event.payload;
      status = `Downloading... ${progress}`;
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  });
  async function startDownload() {
    if (!url.trim()) {
      status = "Please enter a valid Twitter video URL";
      return;
    }
    if (!downloadDir) {
      status = "Please select a download folder";
      return;
    }

    downloading = true;
    status = "Downloading...";

    try {
      const res = await invoke("download_video", {
        url,
        quality,
        outputPath: downloadDir,
      });
      status = res as string;
    } catch (e) {
      status = `Error: ${(e as Error).message}`;
    } finally {
      downloading = false;
    }
  }
</script>

<main>
  <div class="container">
    <header>
      <div class="icon">
        <svg
          width="32"
          height="32"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M19 3H5C3.89543 3 3 3.89543 3 5V19C3 20.1046 3.89543 21 5 21H19C20.1046 21 21 20.1046 21 19V5C21 3.89543 20.1046 3 19 3Z"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
          <path
            d="M9 12L11 14L15 10"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </div>
      <h1>Twitter Video Downloader</h1>
      <p class="subtitle">Save videos from Twitter in high quality</p>
    </header>

    <section class="form">
      <div class="input-group">
        <label for="url">Video URL</label>
        <input
          id="url"
          type="text"
          bind:value={url}
          placeholder="https://twitter.com/username/status/..."
        />
      </div>

      <div class="input-group">
        <label for="quality">Quality</label>
        <select id="quality" bind:value={quality}>
          {#each qualities as q}
            <option value={q.value}>{q.label}</option>
          {/each}
        </select>
      </div>

      <div class="input-group">
        <label for="folder">Download Location</label>
        <div class="folder-input">
          <input
            id="folder"
            type="text"
            value={downloadDir || "No folder selected"}
            readonly
            placeholder="Choose a folder..."
          />
          <button type="button" class="browse-btn" on:click={chooseFolder}>
            Choose
          </button>
        </div>
      </div>

      {#if downloading && progress}
        <div class="progress-container">
          <div class="progress-bar">
            <div class="progress-fill" style="width: {progress}"></div>
          </div>
          <span class="progress-text">{progress}</span>
        </div>
      {/if}

      <button
        class="download-btn"
        on:click={startDownload}
        disabled={downloading}
      >
        <svg
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M12 3V16M12 16L16 12M12 16L8 12"
            stroke="currentColor"
            stroke-width="2.5"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
          <path
            d="M3 16V19C3 20.1046 3.89543 21 5 21H19C20.1046 21 21 20.1046 21 19V16"
            stroke="currentColor"
            stroke-width="2.5"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
        {downloading ? "Downloading..." : "Download Video"}
      </button>

      {#if status}
        <p
          class="status"
          class:error={status.includes("Error") || status.includes("Please")}
        >
          {status}
        </p>
      {/if}
    </section>
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    background: #1e1e2e;
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Display", "Segoe UI",
      "Helvetica Neue", Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
  }

  main {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
    padding: 2rem;
  }

  .container {
    width: 100%;
    max-width: 480px;
    background: rgba(36, 39, 58, 0.7);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border-radius: 20px;
    padding: 2.5rem;
    box-shadow:
      0 1px 3px rgba(0, 0, 0, 0.3),
      0 10px 40px rgba(0, 0, 0, 0.5);
    border: 1px solid rgba(205, 214, 244, 0.1);
  }

  header {
    text-align: center;
    margin-bottom: 2rem;
  }

  .icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 56px;
    height: 56px;
    background: linear-gradient(135deg, #f5c2e7 0%, #cba6f7 100%);
    border-radius: 14px;
    margin-bottom: 1rem;
    color: #1e1e2e;
  }

  h1 {
    margin: 0 0 0.5rem 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: #cdd6f4;
    letter-spacing: -0.02em;
  }

  .subtitle {
    font-size: 0.9rem;
    color: #bac2de;
    margin: 0;
    font-weight: 400;
  }

  .form {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  label {
    font-size: 0.875rem;
    font-weight: 500;
    color: #cdd6f4;
    letter-spacing: -0.01em;
  }

  input,
  select {
    width: 100%;
    padding: 0.75rem 0.875rem;
    border: 1.5px solid #45475a;
    border-radius: 10px;
    background-color: #313244;
    color: #cdd6f4;
    font-size: 0.9375rem;
    transition: all 0.2s ease;
    box-sizing: border-box;
    font-family: inherit;
  }

  input:focus,
  select:focus {
    outline: none;
    border-color: #f5c2e7;
    background-color: #45475a;
    box-shadow: 0 0 0 4px rgba(245, 194, 231, 0.15);
  }

  input::placeholder {
    color: #7f849c;
  }

  input:disabled {
    background-color: #181825;
    color: #6c7086;
  }

  select {
    appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg width='12' height='8' viewBox='0 0 12 8' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M1 1.5L6 6.5L11 1.5' stroke='%23bac2de' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 0.875rem center;
    padding-right: 2.5rem;
    cursor: pointer;
  }

  .folder-input {
    display: flex;
    gap: 0.5rem;
  }

  .folder-input input {
    flex: 1;
    cursor: default;
  }

  .browse-btn {
    padding: 0.75rem 1.25rem;
    background: #45475a;
    border: none;
    border-radius: 10px;
    color: #cdd6f4;
    font-weight: 500;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.2s ease;
    white-space: nowrap;
  }

  .browse-btn:hover {
    background: #585b70;
  }

  .browse-btn:active {
    transform: scale(0.98);
  }

  .progress-container {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .progress-bar {
    width: 100%;
    height: 6px;
    background: #313244;
    border-radius: 10px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #f5c2e7, #cba6f7);
    border-radius: 10px;
    transition: width 0.3s ease;
  }

  .progress-text {
    font-size: 0.8125rem;
    color: #bac2de;
    text-align: center;
    font-weight: 500;
  }

  .download-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    width: 100%;
    padding: 0.875rem;
    margin-top: 0.5rem;
    font-size: 0.9375rem;
    font-weight: 600;
    background: linear-gradient(135deg, #f5c2e7 0%, #cba6f7 100%);
    color: #1e1e2e;
    border: none;
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.2s ease;
    letter-spacing: -0.01em;
  }

  .download-btn:hover:enabled {
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(245, 194, 231, 0.4);
  }

  .download-btn:active:enabled {
    transform: translateY(0);
  }

  .download-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .status {
    margin: 0;
    text-align: center;
    font-size: 0.875rem;
    color: #a6e3a1;
    font-weight: 500;
    padding: 0.75rem;
    background: rgba(166, 227, 161, 0.1);
    border-radius: 10px;
  }

  .status.error {
    color: #f38ba8;
    background: rgba(243, 139, 168, 0.1);
  }
</style>
