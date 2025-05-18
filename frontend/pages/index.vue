<template>
  <div class="app-home">
    <div class="app-panel app-container">
      <div class="app-panel-content">
        <h1 class="app-title">HotSpot 活动广场</h1>
        <p class="app-subtitle">发现附近的签到和活动，结识新朋友</p>
        <div class="app-actions">
          <el-button class="app-btn app-btn-main" type="primary" size="large" @click="showCheckinDialog = true" v-if="authStore.isAuthenticated">
            <i class="app-icon">📝</i> 发起签到
          </el-button>
          <el-button class="app-btn" size="large" @click="navigateTo('/group')">
            <i class="app-icon">👥</i> 群组
          </el-button>
          <el-button class="app-btn app-btn-map" size="large" @click="showMapModal = true">
            <i class="app-icon">🗺️</i> 活动地图
          </el-button>
        </div>
      </div>
      <div class="app-panel-image">
        <div class="app-float app-emoji" aria-label="主图">📍</div>
      </div>
    </div>
    <div class="app-layout">
      <div class="app-main">
        <div class="activity-board app-container app-corner" v-if="userStore.hasLocation">
          <div class="app-section-header">
            <h3 class="app-section-title"><i class="app-icon">🔥</i> 附近签到/活动</h3>
            <el-button class="app-refresh-btn" @click="loadNearbyCheckins(true)"><i class="app-icon">🔄</i></el-button>
          </div>
          <div class="app-loading" v-if="checkinStore.isLoading && !checkinStore.checkins.length">
            <el-skeleton :rows="3" animated />
            <el-skeleton :rows="3" animated />
          </div>
          <div v-else-if="checkinStore.checkins.length === 0" class="app-empty">
            <div class="app-empty-img app-pulse app-emoji" aria-label="空数据">🗒️</div>
            <p>暂无附近签到或活动，快来发布吧！</p>
            <el-button class="app-btn app-btn-main" type="primary" @click="showCheckinDialog = true">
              <i class="app-icon">📝</i> 发起签到
            </el-button>
          </div>
          <div v-else class="activity-list">
            <CheckinCard v-for="checkin in checkinStore.checkins" :key="checkin.checkin_id" :checkin="checkin" class="activity-card" />
            <div class="app-load-more" v-if="checkinStore.hasMore">
              <el-button class="app-btn" @click="loadMoreCheckins" :loading="checkinStore.isLoading">加载更多</el-button>
            </div>
          </div>
        </div>
        <div class="app-get-location app-container app-corner" v-else>
          <div class="app-loc-img app-float app-emoji" aria-label="获取位置">📡</div>
          <h3 class="app-section-title">定位您的位置</h3>
          <p>请授权定位，才能为你推荐附近签到和活动！</p>
          <el-button class="app-btn app-btn-main" type="primary" @click="getLocation" :loading="isGettingLocation">
            <i class="app-icon">📡</i> 获取我的位置
          </el-button>
        </div>
      </div>
      <div class="app-sidebar">
        <div class="user-board app-container app-corner">
          <div class="app-section-header">
            <h3 class="app-section-title"><i class="app-icon">🧑‍🤝‍🧑</i> 附近用户</h3>
            <el-button class="app-refresh-btn" @click="loadNearbyUsers(true)" v-if="userStore.hasLocation"><i class="app-icon">🔄</i></el-button>
          </div>
          <div v-if="!userStore.hasLocation" class="app-empty"><p>请先定位</p></div>
          <div class="app-loading" v-else-if="userStore.isLoadingNearby && !userStore.nearbyUsers.length"><el-skeleton :rows="3" animated /></div>
          <div v-else-if="userStore.nearbyUsers.length === 0" class="app-empty"><div class="app-empty-img app-emoji" aria-label="没有用户">👤</div><p>附近暂无用户</p></div>
          <div v-else class="user-list">
            <div v-for="user in userStore.nearbyUsers" :key="user.user_id" class="user-item" @click="navigateTo(`/user/${user.user_id}`)">
              <div class="user-avatar"><el-avatar :size="45" class="app-avatar">{{ user.nickname.charAt(0) }}</el-avatar></div>
              <div class="user-info">
                <div class="user-name">{{ user.nickname }}</div>
                <div class="user-location"><small><i class="app-icon">📍</i> {{ user.location_name }}</small></div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <CheckinDialog v-model="showCheckinDialog" @success="onCheckinSuccess" />
    <ClientOnly>
      <GameMap v-if="showMapModal" @update-location="handleLocationUpdate" @update-location-name="handleLocationNameUpdate" />
      <MessageDialog v-if="showMessageDialog" @close="handleMessageDialogClose" />
    </ClientOnly>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAuthStore } from '~/stores/auth'
import { useUserStore } from '~/stores/user'
import { useCheckinStore } from '~/stores/checkin'
import { useMessageStore } from '~/stores/message'
import CheckinDialog from '~/components/checkin/CheckinDialog.vue'
import GameMap from '~/components/map/GameMap.client.vue'
import MessageDialog from '~/components/chat/MessageDialog.client.vue'
import { DEFAULT_LATITUDE, DEFAULT_LONGITUDE, DEFAULT_LOCATION_NAME } from '~/utils/constants'

const authStore = useAuthStore()
const userStore = useUserStore()
const checkinStore = useCheckinStore()
const messageStore = useMessageStore()

const isGettingLocation = ref(false)
const showMessageDialog = ref(false)
const showMapModal = ref(false)
const showCheckinDialog = ref(false)

// 获取位置
async function getLocation() {
  isGettingLocation.value = true
  
  try {
    const coords = await userStore.getDeviceLocation()
    if (coords) {
      // 获取位置成功，调用地理编码服务获取位置名称
      // 简化版本：使用"当前位置"或"南天门"作为位置名称
      userStore.setCurrentLocation(
        coords.latitude,
        coords.longitude,
        coords.latitude === DEFAULT_LATITUDE && coords.longitude === DEFAULT_LONGITUDE ? DEFAULT_LOCATION_NAME : '当前位置'
      )
      
      // 更新位置到服务器
      if (authStore.isAuthenticated) {
        await userStore.updateLocationToServer()
      }
      
      // 加载附近数据
      loadNearbyData()
    }
  } catch (error) {
    console.error('获取位置失败', error)
    // 使用默认位置（南天门）
    userStore.setCurrentLocation(35.16, 112.68, '南天门')
    loadNearbyData()
  } finally {
    isGettingLocation.value = false
  }
}

// 加载附近数据
function loadNearbyData() {
  loadNearbyCheckins(true)
  loadNearbyUsers(true)
}

// 加载附近签到
async function loadNearbyCheckins(refresh = false) {
  if (!userStore.hasLocation || !userStore.currentLocation) return
  
  try {
    await checkinStore.fetchNearbyCheckins(
      userStore.currentLocation.latitude,
      userStore.currentLocation.longitude,
      1000, // 默认半径1000米
      refresh
    )
  } catch (err) {
    console.error('获取附近签到失败:', err)
  }
}

// 加载更多签到
function loadMoreCheckins() {
  loadNearbyCheckins(false)
}

// 加载附近用户
async function loadNearbyUsers(refresh = false) {
  if (!userStore.hasLocation || !userStore.currentLocation) return
  
  try {
    await userStore.fetchNearbyUsers(refresh)
  } catch (err) {
    console.error('获取附近用户失败:', err)
  }
}

// 加载更多用户
function loadMoreUsers() {
  loadNearbyUsers(false)
}

// 格式化距离
function formatDistance(meters: number): string {
  if (meters < 1000) {
    return `${Math.round(meters)}米`
  } else {
    return `${(meters / 1000).toFixed(1)}公里`
  }
}

// 打开消息对话框
function openMessageDialog() {
  if (!authStore.isAuthenticated) {
    navigateTo('/login')
    return
  }
  
  showMessageDialog.value = true
}

// 处理消息对话框关闭
function handleMessageDialogClose() {
  showMessageDialog.value = false
}

// 处理地图位置更新
function handleLocationUpdate(newLocation: any) {
  userStore.setCurrentLocation(
    newLocation.latitude,
    newLocation.longitude,
    userStore.currentLocation?.locationName || '未知位置'
  )
}

// 处理地点名称更新
function handleLocationNameUpdate(newName: string) {
  if (userStore.currentLocation) {
    userStore.setCurrentLocation(
      userStore.currentLocation.latitude,
      userStore.currentLocation.longitude,
      newName
    )
  }
}

// 更新位置到服务器
async function updateLocationToServer() {
  if (authStore.isAuthenticated && userStore.hasLocation) {
    try {
      await userStore.updateLocationToServer()
      showMapModal.value = false
      loadNearbyData()
    } catch (err) {
      console.error('更新位置失败:', err)
    }
  }
}

// 签到成功处理
function onCheckinSuccess() {
  loadNearbyCheckins(true)
}

// 初始化数据
onMounted(async () => {
  if (authStore.isAuthenticated) {
    // 加载消息未读计数
    try {
      await messageStore.fetchUnreadCount()
    } catch (err) {
      console.error('获取未读消息数量失败:', err)
    }
    
    // 如果已有位置信息，加载附近数据
    if (userStore.hasLocation) {
      loadNearbyCheckins(true)
      loadNearbyUsers(true)
      
      // 如果需要，更新服务器上的位置信息
      try {
        await userStore.updateLocationToServer()
      } catch (err) {
        console.error('更新位置信息失败:', err)
      }
    }
  }
})
</script>

<style scoped>
.app-home {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.app-panel {
  flex: 0 0 auto;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  overflow: hidden;
  padding: 20px;
  background: linear-gradient(135deg, #f0f4ff 0%, #d9e2ff 100%);
  border-bottom-left-radius: 40px;
  border-bottom-right-radius: 40px;
}

.app-title {
  font-size: 28px;
  font-weight: bold;
  margin: 0;
  color: #333;
}

.app-subtitle {
  font-size: 16px;
  margin: 10px 0 0;
  color: #666;
}

.app-actions {
  display: flex;
  gap: 10px;
  margin-top: 20px;
}

.app-btn {
  flex: 1;
  padding: 10px;
  font-size: 16px;
}

.app-btn-main {
  background-color: #007bff;
  color: #fff;
  border: none;
}

.app-btn-map {
  background-color: #28a745;
  color: #fff;
  border: none;
}

.app-panel-image {
  position: absolute;
  top: 0;
  right: 0;
  width: 100%;
  height: 100%;
  z-index: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.app-emoji {
  font-size: 100px;
  opacity: 0.1;
}

.app-layout {
  flex: 1 0 auto;
  display: flex;
  padding: 20px;
  gap: 20px;
}

.app-main {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.activity-board {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: #fff;
  border-radius: 20px;
  padding: 20px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.app-section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.app-section-title {
  font-size: 18px;
  font-weight: bold;
  margin: 0;
  color: #333;
}

.app-refresh-btn {
  background: none;
  border: none;
  cursor: pointer;
}

.app-loading {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.app-empty {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
}

.app-empty-img {
  font-size: 50px;
  margin-bottom: 10px;
}

.activity-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
}

.app-load-more {
  text-align: center;
  margin-top: 10px;
}

.app-get-location {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
}

.app-loc-img {
  font-size: 60px;
  margin-bottom: 10px;
}

.user-board {
  display: flex;
  flex-direction: column;
  background: #fff;
  border-radius: 20px;
  padding: 20px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.user-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.user-item {
  display: flex;
  align-items: center;
  cursor: pointer;
}

.user-avatar {
  flex-shrink: 0;
}

.user-info {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.user-name {
  font-size: 16px;
  font-weight: bold;
  color: #333;
}

.user-location {
  font-size: 14px;
  color: #666;
}

.app-pulse {
  animation: pulse-animation 2s infinite;
}

@keyframes pulse-animation {
  0% {
    transform: scale(1);
    opacity: 1;
  }
  50% {
    transform: scale(1.05);
    opacity: 0.7;
  }
  100% {
    transform: scale(1);
    opacity: 1;
  }
}
</style>