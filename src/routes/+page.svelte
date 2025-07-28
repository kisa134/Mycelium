<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';

  interface Peer {
    id: string;
    status: 'Discovered' | 'Connected' | 'Disconnected';
    last_seen: string;
  }

  interface NetworkStatus {
    total_peers: number;
    connected_peers: number;
    discovered_peers: number;
    peers: Peer[];
    local_peer_id: string;
  }

  let isRunning = false;
  let networkStatus: NetworkStatus = {
    total_peers: 0,
    connected_peers: 0,
    discovered_peers: 0,
    peers: [],
    local_peer_id: ''
  };
  let statusText = 'Отключен';
  let cpuUsage = 0;
  let ramUsage = 0;
  let ramTotal = 0;
  let ramUsed = 0;
  let logs: string[] = [];
  let systemInfoInterval: number;

  let unsubscribe: () => void;
  let networkStatusUnsubscribe: () => void;

  onMount(async () => {
    // Subscribe to P2P events
    unsubscribe = await listen('p2p_event', (event) => {
      const data = event.payload as any;
      
      switch (data.type) {
        case 'STATUS_UPDATE':
          statusText = data.payload.status_text;
          addLog(data.payload.status_text);
          break;
        case 'ERROR':
          addLog(`Ошибка: ${data.payload.error}`);
          break;
      }
    });

    // Subscribe to network status updates
    networkStatusUnsubscribe = await listen('network-status-update', (event) => {
      networkStatus = event.payload as NetworkStatus;
    });

    // Start system info monitoring
    startSystemMonitoring();
  });

  onDestroy(() => {
    if (unsubscribe) {
      unsubscribe();
    }
    if (networkStatusUnsubscribe) {
      networkStatusUnsubscribe();
    }
    if (systemInfoInterval) {
      clearInterval(systemInfoInterval);
    }
  });

  async function startNode() {
    try {
      await invoke('start_node');
      isRunning = true;
      statusText = 'Поиск участников...';
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
      networkStatus = {
        total_peers: 0,
        connected_peers: 0,
        discovered_peers: 0,
        peers: [],
        local_peer_id: ''
      };
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
    if (networkStatus.connected_peers > 0) return 'bg-green-400';
    return 'bg-yellow-400';
  }

  function getPeerStatusColor(status: string) {
    switch (status) {
      case 'Connected': return 'text-green-600 bg-green-100';
      case 'Discovered': return 'text-yellow-600 bg-yellow-100';
      case 'Disconnected': return 'text-red-600 bg-red-100';
      default: return 'text-gray-600 bg-gray-100';
    }
  }

  function getPeerStatusText(status: string) {
    switch (status) {
      case 'Connected': return 'Подключен';
      case 'Discovered': return 'Подключаюсь...';
      case 'Disconnected': return 'Отключился';
      default: return 'Неизвестно';
    }
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
  <div class="max-w-6xl mx-auto">
    <!-- Header -->
    <div class="bg-white rounded-lg shadow-sm p-6 mb-6">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-gray-900">Mycelium Node</h1>
          <p class="text-gray-600 mt-1">Децентрализованная вычислительная сеть</p>
          {#if networkStatus.local_peer_id}
            <p class="text-sm text-blue-600 mt-1">Ваш ID: {networkStatus.local_peer_id}</p>
          {/if}
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

    <!-- Main Dashboard -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-6">
      <!-- Main Counter -->
      <div class="lg:col-span-1 bg-white rounded-lg shadow-sm p-6">
        <div class="text-center">
          <div class="text-6xl font-bold text-blue-600 mb-2">
            {networkStatus.connected_peers}
          </div>
          <div class="text-lg font-medium text-gray-700 mb-1">
            Участников в сети
          </div>
          <div class="text-sm text-gray-500">
            Всего обнаружено: {networkStatus.total_peers}
          </div>
        </div>
      </div>

      <!-- System Stats -->
      <div class="lg:col-span-2 grid grid-cols-1 md:grid-cols-2 gap-4">
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
    </div>

    <!-- Network Participants -->
    <div class="bg-white rounded-lg shadow-sm p-6 mb-6">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-semibold text-gray-900">Обнаруженные участники</h2>
        <div class="text-sm text-gray-500">
          {networkStatus.peers.length} участников
        </div>
      </div>
      
      {#if networkStatus.peers.length > 0}
        <div class="overflow-x-auto">
          <table class="min-w-full divide-y divide-gray-200">
            <thead class="bg-gray-50">
              <tr>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Peer ID
                </th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Статус
                </th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Последняя активность
                </th>
              </tr>
            </thead>
            <tbody class="bg-white divide-y divide-gray-200">
              {#each networkStatus.peers as peer}
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900">
                    {peer.id}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span class="inline-flex px-2 py-1 text-xs font-semibold rounded-full {getPeerStatusColor(peer.status)}">
                      {getPeerStatusText(peer.status)}
                    </span>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {peer.last_seen}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {:else}
        <div class="text-center py-8 text-gray-500">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
          </svg>
          <p class="mt-2">Участники не обнаружены</p>
          <p class="text-sm">Запустите ноду для поиска участников сети</p>
        </div>
      {/if}
    </div>

    <!-- Logs -->
    <div class="bg-white rounded-lg shadow-sm p-6">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-semibold text-gray-900">Логи сети</h2>
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
