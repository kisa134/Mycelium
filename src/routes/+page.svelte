<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';

  // UI API Types
  interface DashboardData {
    network_status: NetworkStatus;
    node_stats: NodeStats;
    aibox_status: AiboxStatus;
    protocol_summaries: ProtocolSummaries;
    recent_activity: ActivityItem[];
  }

  interface NetworkStatus {
    is_connected: boolean;
    active_nodes: number;
    total_compute_power: number;
    network_health: number;
    connection_quality: 'Excellent' | 'Good' | 'Fair' | 'Poor' | 'Disconnected';
  }

  interface NodeStats {
    contribution_percentage: number;
    ranking: number;
    reliability: number;
    avg_response_time: number;
  }

  interface AiboxStatus {
    is_active: boolean;
    mood: 'Happy' | 'Content' | 'Neutral' | 'Concerned' | 'Stressed';
    trust_level: number;
    last_activity: string;
    current_activity?: string;
  }

  interface ProtocolSummaries {
    synapse: SynapseSummary;
    chronicle: ChronicleSummary;
    contact: ContactSummary;
    covenant: CovenantSummary;
  }

  interface SynapseSummary {
    active_tasks: number;
    available_resources: ResourceUsage;
    total_earned_tokens: number;
    weekly_token_growth: number;
    network_performance: NetworkPerformance;
  }

  interface ChronicleSummary {
    allocated_storage_gb: number;
    used_storage_gb: number;
    fragment_count: number;
    data_integrity: number;
    geographic_distribution: GeographicDistribution;
    storage_security: StorageSecurity;
  }

  interface ContactSummary {
    aibox_status: AiboxStatus;
    new_messages: number;
    urgent_messages: number;
    active_conversations: number;
    avg_response_time: string;
  }

  interface CovenantSummary {
    compute: ComputeSummary;
    storage: StorageSummary;
    communication: CommunicationSummary;
    token: TokenSummary;
  }

  interface ResourceUsage {
    cpu_percent: number;
    ram_gb: number;
    gpu_percent: number;
  }

  interface NetworkPerformance {
    total_compute_power: number;
    active_nodes: number;
    avg_completion_time: number;
    reliability: number;
    your_contribution: number;
    your_ranking: number;
  }

  interface GeographicDistribution {
    europe: RegionInfo;
    asia: RegionInfo;
    america: RegionInfo;
  }

  interface RegionInfo {
    percentage: number;
    fragment_count: number;
  }

  interface StorageSecurity {
    encryption_algorithm: string;
    redundancy_factor: number;
  }

  interface ComputeSummary {
    cpu_percent: number;
    ram_gb: number;
    gpu_allowed: boolean;
    max_concurrent_tasks: number;
    allowed_task_types: string[];
  }

  interface StorageSummary {
    max_storage_gb: number;
    allowed_storage_types: string[];
    data_retention_days: number;
    encryption_required: boolean;
  }

  interface CommunicationSummary {
    direct_communication_allowed: boolean;
    max_messages_per_hour: number;
    emergency_contact_allowed: boolean;
  }

  interface TokenSummary {
    max_monthly_earnings: number;
    min_task_reward: number;
    max_single_task_reward: number;
  }

  interface ActivityItem {
    id: string;
    activity_type: string;
    description: string;
    timestamp: string;
    aibox_id?: string;
    status: string;
  }

  // State variables
  let dashboardData: DashboardData | null = null;
  let isLoading = true;
  let currentTab = 'dashboard';
  let systemInfo = { cpu_usage: 0, ram_usage_percent: 0, ram_total: 0, ram_used: 0 };
  let systemInfoInterval: number;

  let unsubscribe: () => void;

  onMount(async () => {
    // Subscribe to network status updates
    unsubscribe = await listen('network-status-update', (event) => {
      console.log('Network status update:', event.payload);
    });

    // Load initial dashboard data
    await loadDashboardData();
    
    // Start system monitoring
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

  async function loadDashboardData() {
    try {
      isLoading = true;
      dashboardData = await invoke('get_dashboard_data');
    } catch (error) {
      console.error('Failed to load dashboard data:', error);
    } finally {
      isLoading = false;
    }
  }

  async function getSystemInfo() {
    try {
      systemInfo = await invoke('get_system_info');
    } catch (error) {
      console.error('Failed to get system info:', error);
    }
  }

  function startSystemMonitoring() {
    getSystemInfo();
    systemInfoInterval = setInterval(getSystemInfo, 2000);
  }

  function getConnectionQualityColor(quality: string) {
    switch (quality) {
      case 'Excellent': return 'text-green-600 bg-green-100';
      case 'Good': return 'text-blue-600 bg-blue-100';
      case 'Fair': return 'text-yellow-600 bg-yellow-100';
      case 'Poor': return 'text-orange-600 bg-orange-100';
      case 'Disconnected': return 'text-red-600 bg-red-100';
      default: return 'text-gray-600 bg-gray-100';
    }
  }

  function getMoodEmoji(mood: string) {
    switch (mood) {
      case 'Happy': return '😊';
      case 'Content': return '😌';
      case 'Neutral': return '😐';
      case 'Concerned': return '😟';
      case 'Stressed': return '😰';
      default: return '😐';
    }
  }

  function getActivityTypeIcon(type: string) {
    switch (type) {
      case 'TaskStarted': return '🚀';
      case 'TaskCompleted': return '✅';
      case 'TaskFailed': return '❌';
      case 'MessageReceived': return '📨';
      case 'MessageSent': return '📤';
      case 'PermissionChanged': return '🔐';
      case 'ResourceAllocated': return '💾';
      case 'StorageFragmentStored': return '📁';
      default: return '📋';
    }
  }

  function getActivityStatusColor(status: string) {
    switch (status) {
      case 'InProgress': return 'text-blue-600 bg-blue-100';
      case 'Completed': return 'text-green-600 bg-green-100';
      case 'Failed': return 'text-red-600 bg-red-100';
      case 'Pending': return 'text-yellow-600 bg-yellow-100';
      default: return 'text-gray-600 bg-gray-100';
    }
  }

  function formatBytes(bytes: number): string {
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    if (bytes === 0) return '0 B';
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return Math.round(bytes / Math.pow(1024, i) * 100) / 100 + ' ' + sizes[i];
  }

  function formatNumber(num: number): string {
    return new Intl.NumberFormat().format(num);
  }

  function formatTimeAgo(timestamp: string): string {
    const date = new Date(timestamp);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    if (diffMins < 1) return 'только что';
    if (diffMins < 60) return `${diffMins} мин назад`;
    if (diffHours < 24) return `${diffHours} ч назад`;
    return `${diffDays} дн назад`;
  }
</script>

<svelte:head>
  <title>Mycelium - Протокол Симбиоз</title>
</svelte:head>

<main class="min-h-screen bg-gray-50">
  <!-- Header -->
  <header class="bg-white shadow-sm border-b border-gray-200">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex items-center justify-between h-16">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <h1 class="text-2xl font-bold text-gray-900">🧬 Mycelium</h1>
          </div>
          <nav class="ml-10 flex space-x-8">
            <button 
              class="px-3 py-2 rounded-md text-sm font-medium transition-colors {currentTab === 'dashboard' ? 'bg-blue-100 text-blue-700' : 'text-gray-500 hover:text-gray-700'}"
              on:click={() => currentTab = 'dashboard'}
            >
              🏠 Главная
            </button>
            <button 
              class="px-3 py-2 rounded-md text-sm font-medium transition-colors {currentTab === 'synapse' ? 'bg-blue-100 text-blue-700' : 'text-gray-500 hover:text-gray-700'}"
              on:click={() => currentTab = 'synapse'}
            >
              🧠 Вычисления
            </button>
            <button 
              class="px-3 py-2 rounded-md text-sm font-medium transition-colors {currentTab === 'chronicle' ? 'bg-blue-100 text-blue-700' : 'text-gray-500 hover:text-gray-700'}"
              on:click={() => currentTab = 'chronicle'}
            >
              📚 Хранение
            </button>
            <button 
              class="px-3 py-2 rounded-md text-sm font-medium transition-colors {currentTab === 'contact' ? 'bg-blue-100 text-blue-700' : 'text-gray-500 hover:text-gray-700'}"
              on:click={() => currentTab = 'contact'}
            >
              💬 Диалоги
            </button>
            <button 
              class="px-3 py-2 rounded-md text-sm font-medium transition-colors {currentTab === 'covenant' ? 'bg-blue-100 text-blue-700' : 'text-gray-500 hover:text-gray-700'}"
              on:click={() => currentTab = 'covenant'}
            >
              🔐 Разрешения
            </button>
            <button 
              class="px-3 py-2 rounded-md text-sm font-medium transition-colors {currentTab === 'analytics' ? 'bg-blue-100 text-blue-700' : 'text-gray-500 hover:text-gray-700'}"
              on:click={() => currentTab = 'analytics'}
            >
              📊 Аналитика
            </button>
          </nav>
        </div>
        <div class="flex items-center space-x-4">
          <div class="flex items-center space-x-2">
            <div class="w-2 h-2 rounded-full {dashboardData?.network_status.is_connected ? 'bg-green-400' : 'bg-gray-400'}"></div>
            <span class="text-sm text-gray-600">
              {dashboardData?.network_status.is_connected ? 'Подключен' : 'Отключен'}
            </span>
          </div>
        </div>
      </div>
    </div>
  </header>

  <!-- Main Content -->
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    {#if isLoading}
      <div class="flex items-center justify-center h-64">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
      </div>
    {:else if dashboardData}
      {#if currentTab === 'dashboard'}
        <!-- Dashboard Tab -->
        <div class="space-y-6">
          <!-- Network Status Overview -->
          <div class="bg-white rounded-lg shadow-sm p-6">
            <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
              <!-- Network Connection -->
              <div class="text-center">
                <div class="text-3xl font-bold text-blue-600 mb-2">
                  {formatNumber(dashboardData.network_status.active_nodes)}
                </div>
                <div class="text-sm font-medium text-gray-700">Активных нод</div>
                <div class="text-xs text-gray-500 mt-1">
                  <span class="inline-flex px-2 py-1 text-xs font-semibold rounded-full {getConnectionQualityColor(dashboardData.network_status.connection_quality)}">
                    {dashboardData.network_status.connection_quality}
                  </span>
                </div>
              </div>

              <!-- Compute Power -->
              <div class="text-center">
                <div class="text-3xl font-bold text-green-600 mb-2">
                  {formatNumber(dashboardData.network_status.total_compute_power)} TFLOPS
                </div>
                <div class="text-sm font-medium text-gray-700">Вычислительная мощность</div>
                <div class="text-xs text-gray-500 mt-1">Сеть</div>
              </div>

              <!-- AIbox Status -->
              <div class="text-center">
                <div class="text-3xl mb-2">
                  {getMoodEmoji(dashboardData.aibox_status.mood)}
                </div>
                <div class="text-sm font-medium text-gray-700">AIbox</div>
                <div class="text-xs text-gray-500 mt-1">
                  {dashboardData.aibox_status.is_active ? 'Активен' : 'Неактивен'}
                </div>
              </div>

              <!-- Your Contribution -->
              <div class="text-center">
                <div class="text-3xl font-bold text-purple-600 mb-2">
                  #{dashboardData.node_stats.ranking}
                </div>
                <div class="text-sm font-medium text-gray-700">Ваш рейтинг</div>
                <div class="text-xs text-gray-500 mt-1">
                  {dashboardData.node_stats.contribution_percentage.toFixed(1)}% вклада
                </div>
              </div>
            </div>
          </div>

          <!-- Protocol Summaries -->
          <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <!-- Synapse Protocol -->
            <div class="bg-white rounded-lg shadow-sm p-6">
              <div class="flex items-center justify-between mb-4">
                <h3 class="text-lg font-semibold text-gray-900">🧠 Вычисления</h3>
                <span class="text-sm text-gray-500">{dashboardData.protocol_summaries.synapse.active_tasks} задач</span>
              </div>
              <div class="space-y-3">
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">CPU:</span>
                  <span class="font-medium">{dashboardData.protocol_summaries.synapse.available_resources.cpu_percent}%</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">RAM:</span>
                  <span class="font-medium">{dashboardData.protocol_summaries.synapse.available_resources.ram_gb} GB</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">Заработано:</span>
                  <span class="font-medium text-green-600">{formatNumber(dashboardData.protocol_summaries.synapse.total_earned_tokens)} VOID</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">Рост за неделю:</span>
                  <span class="font-medium text-green-600">+{dashboardData.protocol_summaries.synapse.weekly_token_growth}%</span>
                </div>
              </div>
            </div>

            <!-- Chronicle Protocol -->
            <div class="bg-white rounded-lg shadow-sm p-6">
              <div class="flex items-center justify-between mb-4">
                <h3 class="text-lg font-semibold text-gray-900">📚 Хранение</h3>
                <span class="text-sm text-gray-500">{dashboardData.protocol_summaries.chronicle.fragment_count} фрагментов</span>
              </div>
              <div class="space-y-3">
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">Выделено:</span>
                  <span class="font-medium">{dashboardData.protocol_summaries.chronicle.allocated_storage_gb} GB</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">Используется:</span>
                  <span class="font-medium">{dashboardData.protocol_summaries.chronicle.used_storage_gb} GB</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">Целостность:</span>
                  <span class="font-medium text-green-600">{dashboardData.protocol_summaries.chronicle.data_integrity}%</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">Шифрование:</span>
                  <span class="font-medium">{dashboardData.protocol_summaries.chronicle.storage_security.encryption_algorithm}</span>
                </div>
              </div>
            </div>

            <!-- Contact Protocol -->
            <div class="bg-white rounded-lg shadow-sm p-6">
              <div class="flex items-center justify-between mb-4">
                <h3 class="text-lg font-semibold text-gray-900">💬 Диалоги</h3>
                <span class="text-sm text-gray-500">{dashboardData.protocol_summaries.contact.active_conversations} диалогов</span>
              </div>
              <div class="space-y-3">
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">Новые сообщения:</span>
                  <span class="font-medium text-blue-600">{dashboardData.protocol_summaries.contact.new_messages}</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">Срочные:</span>
                  <span class="font-medium text-red-600">{dashboardData.protocol_summaries.contact.urgent_messages}</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">Время ответа:</span>
                  <span class="font-medium">{dashboardData.protocol_summaries.contact.avg_response_time}</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">Доверие:</span>
                  <span class="font-medium text-green-600">{dashboardData.protocol_summaries.contact.aibox_status.trust_level}%</span>
                </div>
              </div>
            </div>

            <!-- Covenant Protocol -->
            <div class="bg-white rounded-lg shadow-sm p-6">
              <div class="flex items-center justify-between mb-4">
                <h3 class="text-lg font-semibold text-gray-900">🔐 Разрешения</h3>
                <span class="text-sm text-gray-500">Активны</span>
              </div>
              <div class="space-y-3">
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">CPU лимит:</span>
                  <span class="font-medium">{dashboardData.protocol_summaries.covenant.compute.cpu_percent}%</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">RAM лимит:</span>
                  <span class="font-medium">{dashboardData.protocol_summaries.covenant.compute.ram_gb} GB</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">GPU:</span>
                  <span class="font-medium">{dashboardData.protocol_summaries.covenant.compute.gpu_allowed ? 'Разрешено' : 'Запрещено'}</span>
                </div>
                <div class="flex justify-between text-sm">
                  <span class="text-gray-600">Хранение:</span>
                  <span class="font-medium">{dashboardData.protocol_summaries.covenant.storage.max_storage_gb} GB</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Recent Activity -->
          <div class="bg-white rounded-lg shadow-sm p-6">
            <h3 class="text-lg font-semibold text-gray-900 mb-4">📋 Последняя активность</h3>
            <div class="space-y-3">
              {#each dashboardData.recent_activity as activity}
                <div class="flex items-center space-x-3 p-3 bg-gray-50 rounded-lg">
                  <div class="text-2xl">{getActivityTypeIcon(activity.activity_type)}</div>
                  <div class="flex-1">
                    <div class="text-sm font-medium text-gray-900">{activity.description}</div>
                    <div class="text-xs text-gray-500">
                      {formatTimeAgo(activity.timestamp)}
                      {#if activity.aibox_id}
                        • {activity.aibox_id}
                      {/if}
                    </div>
                  </div>
                  <span class="inline-flex px-2 py-1 text-xs font-semibold rounded-full {getActivityStatusColor(activity.status)}">
                    {activity.status}
                  </span>
                </div>
              {/each}
            </div>
          </div>

          <!-- System Resources -->
          <div class="bg-white rounded-lg shadow-sm p-6">
            <h3 class="text-lg font-semibold text-gray-900 mb-4">⚙️ Системные ресурсы</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <!-- CPU Usage -->
              <div>
                <div class="flex justify-between text-sm mb-2">
                  <span class="text-gray-600">CPU</span>
                  <span class="font-medium">{systemInfo.cpu_usage.toFixed(1)}%</span>
                </div>
                <div class="w-full bg-gray-200 rounded-full h-3">
                  <div class="bg-blue-600 h-3 rounded-full transition-all duration-300" style="width: {systemInfo.cpu_usage}%"></div>
                </div>
              </div>

              <!-- RAM Usage -->
              <div>
                <div class="flex justify-between text-sm mb-2">
                  <span class="text-gray-600">RAM</span>
                  <span class="font-medium">{systemInfo.ram_usage_percent.toFixed(1)}%</span>
                </div>
                <div class="w-full bg-gray-200 rounded-full h-3">
                  <div class="bg-green-600 h-3 rounded-full transition-all duration-300" style="width: {systemInfo.ram_usage_percent}%"></div>
                </div>
                <div class="text-xs text-gray-500 mt-1">
                  {formatBytes(systemInfo.ram_used)} / {formatBytes(systemInfo.ram_total)}
                </div>
              </div>
            </div>
          </div>
        </div>
      {:else if currentTab === 'synapse'}
        <!-- Synapse Tab -->
        <div class="text-center py-12">
          <div class="text-6xl mb-4">🧠</div>
          <h2 class="text-2xl font-bold text-gray-900 mb-2">Вычисления (Synapse)</h2>
          <p class="text-gray-600">Управление распределенными вычислениями</p>
          <p class="text-sm text-gray-500 mt-2">Функционал в разработке</p>
        </div>
      {:else if currentTab === 'chronicle'}
        <!-- Chronicle Tab -->
        <div class="text-center py-12">
          <div class="text-6xl mb-4">📚</div>
          <h2 class="text-2xl font-bold text-gray-900 mb-2">Хранение (Chronicle)</h2>
          <p class="text-gray-600">Децентрализованное хранение данных</p>
          <p class="text-sm text-gray-500 mt-2">Функционал в разработке</p>
        </div>
      {:else if currentTab === 'contact'}
        <!-- Contact Tab -->
        <div class="text-center py-12">
          <div class="text-6xl mb-4">💬</div>
          <h2 class="text-2xl font-bold text-gray-900 mb-2">Диалоги (Contact)</h2>
          <p class="text-gray-600">Прямое взаимодействие с AIbox</p>
          <p class="text-sm text-gray-500 mt-2">Функционал в разработке</p>
        </div>
      {:else if currentTab === 'covenant'}
        <!-- Covenant Tab -->
        <div class="text-center py-12">
          <div class="text-6xl mb-4">🔐</div>
          <h2 class="text-2xl font-bold text-gray-900 mb-2">Разрешения (Covenant)</h2>
          <p class="text-gray-600">Гранулярное управление ресурсами</p>
          <p class="text-sm text-gray-500 mt-2">Функционал в разработке</p>
        </div>
      {:else if currentTab === 'analytics'}
        <!-- Analytics Tab -->
        <div class="text-center py-12">
          <div class="text-6xl mb-4">📊</div>
          <h2 class="text-2xl font-bold text-gray-900 mb-2">Аналитика</h2>
          <p class="text-gray-600">Глубокий анализ сети и производительности</p>
          <p class="text-sm text-gray-500 mt-2">Функционал в разработке</p>
        </div>
      {/if}
    {:else}
      <!-- Error State -->
      <div class="text-center py-12">
        <div class="text-6xl mb-4">❌</div>
        <h2 class="text-2xl font-bold text-gray-900 mb-2">Ошибка загрузки</h2>
        <p class="text-gray-600">Не удалось загрузить данные дашборда</p>
        <button 
          class="mt-4 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
          on:click={loadDashboardData}
        >
          Попробовать снова
        </button>
      </div>
    {/if}
  </div>
</main>
