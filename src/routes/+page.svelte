<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';

  let isRunning = false;
  let peerCount = 0;
  let statusText = 'Отключен';
  let cpuUsage = 0;
  let ramUsage = 0;
  let ramTotal = 0;
  let ramUsed = 0;
  let logs: string[] = [];
  let systemInfoInterval: number;

  let unsubscribe: () => void;

  onMount(async () => {
    // Subscribe to P2P events
    unsubscribe = await listen('p2p_event', (event) => {
      const data = event.payload as any;
      
      switch (data.type) {
        case 'PEER_CONNECTED':
          peerCount++;
          addLog(`Подключен пир: ${data.payload.peer_id}`);
          break;
        case 'PEER_DISCONNECTED':
          peerCount = Math.max(0, peerCount - 1);
          addLog(`Отключен пир: ${data.payload.peer_id}`);
          break;
        case 'STATUS_UPDATE':
          statusText = data.payload.status_text;
          addLog(`Статус: ${statusText}`);
          break;
        case 'PEER_COUNT':
          peerCount = data.payload.count;
          break;
        case 'ERROR':
          addLog(`Ошибка: ${data.payload.error}`);
          break;
      }
    });

    // Start system info monitoring
    startSystemMonitoring();
  });

  onDestroy(() => {
    if (unsubscribe) {
      unsubscribe();
    }
    if (systemInfoInterval) {
      clearInterval(systemInfoInterval);
    }
  });

  async function startNode() {
    try {
      await invoke('start_node');
      isRunning = true;
      statusText = 'Поиск пиров...';
      addLog('Нода запущена');
    } catch (error) {
      addLog(`Ошибка запуска: ${error}`);
    }
  }

  async function stopNode() {
    try {
      await invoke('stop_node');
      isRunning = false;
      statusText = 'Отключен';
      peerCount = 0;
      addLog('Нода остановлена');
    } catch (error) {
      addLog(`Ошибка остановки: ${error}`);
    }
  }

  async function getSystemInfo() {
    try {
      const info = await invoke('get_system_info');
      cpuUsage = info.cpu_usage;
      ramUsage = info.ram_usage_percent;
      ramTotal = info.ram_total;
      ramUsed = info.ram_used;
    } catch (error) {
      console.error('Ошибка получения системной информации:', error);
    }
  }

  function startSystemMonitoring() {
    getSystemInfo();
    systemInfoInterval = setInterval(getSystemInfo, 2000);
  }

  function addLog(message: string) {
    const timestamp = new Date().toLocaleTimeString();
    logs = [...logs, `[${timestamp}] ${message}`].slice(-50); // Keep last 50 logs
  }

  function getStatusColor() {
    if (!isRunning) return 'bg-gray-400';
    if (peerCount > 0) return 'bg-green-400';
    return 'bg-yellow-400';
  }

  function formatBytes(bytes: number): string {
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    if (bytes === 0) return '0 B';
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return Math.round(bytes / Math.pow(1024, i) * 100) / 100 + ' ' + sizes[i];
  }
</script>

<svelte:head>
  <title>Mycelium Node</title>
</svelte:head>

<main class="min-h-screen bg-gray-50 p-4">
  <div class="max-w-4xl mx-auto">
    <!-- Header -->
    <div class="bg-white rounded-lg shadow-sm p-6 mb-6">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-gray-900">Mycelium Node</h1>
          <p class="text-gray-600 mt-1">Децентрализованная вычислительная сеть</p>
        </div>
        <div class="flex items-center space-x-4">
          <div class="flex items-center space-x-2">
            <div class="w-3 h-3 rounded-full {getStatusColor()}"></div>
            <span class="text-sm font-medium text-gray-700">{statusText}</span>
          </div>
          <button
            on:click={isRunning ? stopNode : startNode}
            class="px-6 py-2 rounded-lg font-medium transition-colors {isRunning 
              ? 'bg-red-500 hover:bg-red-600 text-white' 
              : 'bg-green-500 hover:bg-green-600 text-white'}"
          >
            {isRunning ? 'Остановить' : 'Запустить'}
          </button>
        </div>
      </div>
    </div>

    <!-- Stats Grid -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
      <!-- Peer Count -->
      <div class="bg-white rounded-lg shadow-sm p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-medium text-gray-600">Обнаружено пиров</p>
            <p class="text-2xl font-bold text-gray-900">{peerCount}</p>
          </div>
          <div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center">
            <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path>
            </svg>
          </div>
        </div>
      </div>

      <!-- CPU Usage -->
      <div class="bg-white rounded-lg shadow-sm p-6">
        <div class="flex items-center justify-between mb-2">
          <p class="text-sm font-medium text-gray-600">CPU</p>
          <p class="text-sm font-medium text-gray-900">{cpuUsage.toFixed(1)}%</p>
        </div>
        <div class="w-full bg-gray-200 rounded-full h-2">
          <div class="bg-blue-600 h-2 rounded-full transition-all duration-300" style="width: {cpuUsage}%"></div>
        </div>
      </div>

      <!-- RAM Usage -->
      <div class="bg-white rounded-lg shadow-sm p-6">
        <div class="flex items-center justify-between mb-2">
          <p class="text-sm font-medium text-gray-600">RAM</p>
          <p class="text-sm font-medium text-gray-900">{ramUsage.toFixed(1)}%</p>
        </div>
        <div class="w-full bg-gray-200 rounded-full h-2 mb-1">
          <div class="bg-green-600 h-2 rounded-full transition-all duration-300" style="width: {ramUsage}%"></div>
        </div>
        <p class="text-xs text-gray-500">{formatBytes(ramUsed)} / {formatBytes(ramTotal)}</p>
      </div>
    </div>

    <!-- Logs -->
    <div class="bg-white rounded-lg shadow-sm p-6">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-semibold text-gray-900">Логи</h2>
        <button
          on:click={() => logs = []}
          class="text-sm text-gray-500 hover:text-gray-700"
        >
          Очистить
        </button>
      </div>
      <div class="bg-gray-50 rounded-lg p-4 h-64 overflow-y-auto font-mono text-sm">
        {#each logs as log}
          <div class="text-gray-700 mb-1">{log}</div>
        {/each}
        {#if logs.length === 0}
          <div class="text-gray-400">Логи пусты</div>
        {/if}
      </div>
    </div>
  </div>
</main>
