const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const channelList = document.getElementById("channel-list");
const scopeSelect = document.getElementById("scope-select");
const addForm = document.getElementById("add-channel-form");
const messages = document.getElementById("messages");
const chatForm = document.getElementById("chat-form");
const chatQuestion = document.getElementById("chat-question");
const modelStatusText = document.getElementById("model-status-text");
const modelHelpText = document.getElementById("model-help-text");
const openModelsFolderBtn = document.getElementById("open-models-folder-btn");
const refreshModelBtn = document.getElementById("refresh-model-btn");

let channels = [];
let syncProgressEls = {};

function addMessage(role, text) {
  const wrapper = document.createElement("div");
  wrapper.className = `message ${role}`;
  const textSpan = document.createElement("span");
  textSpan.className = "text";
  textSpan.textContent = text;
  wrapper.appendChild(textSpan);
  messages.appendChild(wrapper);
  messages.scrollTop = messages.scrollHeight;
  return wrapper;
}

function renderChannels() {
  channelList.innerHTML = "";
  const currentScope = scopeSelect.value;
  scopeSelect.innerHTML = '<option value="">All channels</option>';

  for (const c of channels) {
    const li = document.createElement("li");
    li.className = "channel-item";
    li.dataset.slug = c.slug;
    li.innerHTML = `
      <div class="name">${c.name}</div>
      <div class="meta">${c.topic} · ${c.video_count || 0} videos${c.last_synced ? " · synced" : " · not synced yet"}</div>
      <div class="actions">
        <button data-action="sync">Sync</button>
        <button data-action="remove">Remove</button>
      </div>
      <div class="progress-bar" hidden><div style="width:0%"></div></div>
    `;
    channelList.appendChild(li);
    syncProgressEls[c.slug] = li.querySelector(".progress-bar");

    const option = document.createElement("option");
    option.value = c.slug;
    option.textContent = c.name;
    scopeSelect.appendChild(option);
  }
  scopeSelect.value = currentScope;
}

async function refreshChannels() {
  channels = await invoke("list_channels");
  renderChannels();
}

addForm.addEventListener("submit", async (e) => {
  e.preventDefault();
  const url = document.getElementById("channel-url").value.trim();
  const name = document.getElementById("channel-name").value.trim();
  const topic = document.getElementById("channel-topic").value.trim();
  if (!url || !name || !topic) return;

  const channel = await invoke("add_channel", { url, name, topic });
  addForm.reset();
  await refreshChannels();
  syncChannel(channel.slug);
});

async function syncChannel(slug) {
  const bar = syncProgressEls[slug];
  if (bar) bar.hidden = false;
  try {
    await invoke("sync_channel", { slug });
  } catch (err) {
    addMessage("system", `Sync failed for ${slug}: ${err}`);
  }
  await refreshChannels();
}

channelList.addEventListener("click", async (e) => {
  const button = e.target.closest("button");
  if (!button) return;
  const slug = button.closest(".channel-item").dataset.slug;
  if (button.dataset.action === "sync") {
    syncChannel(slug);
  } else if (button.dataset.action === "remove") {
    await invoke("remove_channel", { slug });
    await refreshChannels();
  }
});

listen("sync-progress", (event) => {
  const p = event.payload;
  const bar = syncProgressEls[p.slug];
  if (!bar) return;
  bar.hidden = p.finished;
  const inner = bar.querySelector("div");
  const pct = p.total > 0 ? Math.round((p.done / p.total) * 100) : 0;
  inner.style.width = `${pct}%`;
});

async function refreshModelStatus() {
  const status = await invoke("model_status");
  if (status.downloaded) {
    modelStatusText.textContent = `Local model ready (${status.model_path}).`;
    modelHelpText.hidden = true;
  } else {
    modelStatusText.textContent = "No local model found yet.";
    modelHelpText.hidden = false;
  }
}

openModelsFolderBtn.addEventListener("click", async () => {
  try {
    await invoke("open_models_folder");
  } catch (err) {
    modelStatusText.textContent = `Couldn't open folder: ${err}`;
  }
});

refreshModelBtn.addEventListener("click", refreshModelStatus);

let currentAssistantEl = null;
let sendInFlight = false;

chatForm.addEventListener("submit", async (e) => {
  e.preventDefault();
  if (sendInFlight) return;
  const question = chatQuestion.value.trim();
  if (!question) return;

  addMessage("user", question);
  chatQuestion.value = "";
  currentAssistantEl = addMessage("assistant", "");
  sendInFlight = true;
  chatForm.querySelector("button").disabled = true;

  try {
    await invoke("ask_question", {
      slug: scopeSelect.value || null,
      question,
    });
  } catch (err) {
    currentAssistantEl.textContent = `Error: ${err}`;
  }
});

listen("chat-sources", (event) => {
  if (!currentAssistantEl) return;
  const sources = event.payload;
  if (!sources.length) return;
  const box = document.createElement("div");
  box.className = "sources";
  box.innerHTML =
    "Sources: " +
    sources
      .map((s, i) => `<a href="${s.url}" target="_blank">[${i + 1}] ${s.title}</a>`)
      .join(" · ");
  currentAssistantEl.appendChild(box);
});

listen("chat-token", (event) => {
  const { token, done } = event.payload;
  if (!currentAssistantEl) return;
  if (token) {
    currentAssistantEl.querySelector(".text").appendChild(document.createTextNode(token));
    messages.scrollTop = messages.scrollHeight;
  }
  if (done) {
    sendInFlight = false;
    chatForm.querySelector("button").disabled = false;
  }
});

refreshChannels();
refreshModelStatus();
