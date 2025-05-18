<template>
  <div class="game-map-container zelda-corner-decorations">
    <ClientOnly>
      <div id="map-container" class="map-container">
        <!-- 右键/长按菜单 -->
        <Teleport to="body">
          <div v-if="showContextMenu" class="map-context-menu zelda-container" :style="contextMenuStyle">
            <div class="menu-item" @click="handleMenuClick">
              <i class="cartoon-icon">📍</i>
              <span>设为当前位置</span>
            </div>
          </div>
        </Teleport>

        <!-- 功能提示 -->
        <div v-if="!hasShownTip" class="map-tip">
          <i class="cartoon-icon">💡</i>
          <span>长按或右键点击地图可设置位置</span>
          <el-button class="zelda-btn" size="small" type="primary" link @click="dismissTip">知道了</el-button>
        </div>

        <!-- 地图加载中提示 -->
        <div v-if="isLoading" class="map-loading">
          <i class="cartoon-icon pulse-animation">🗺️</i>
          <span>地图加载中...</span>
        </div>
      </div>

      <!-- 地图控制 -->
      <div class="map-controls">
        <div class="control-button" @click="handleZoomIn">
          <i class="cartoon-icon">+</i>
        </div>
        <div class="control-button" @click="handleZoomOut">
          <i class="cartoon-icon">-</i>
        </div>
        <div class="control-button" @click="handleCenterToCurrentLocation">
          <i class="cartoon-icon">🎯</i>
        </div>
      </div>

      <template #fallback>
        <div class="map-loading-fallback">
          <i class="cartoon-icon pulse-animation">🗺️</i>
          <span>地图加载中...</span>
        </div>
      </template>
    </ClientOnly>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch, nextTick } from 'vue'
import { ElMessage } from 'element-plus'
import { DEFAULT_LATITUDE, DEFAULT_LONGITUDE, DEFAULT_LOCATION_NAME } from '~/utils/constants'
import AMapLoader from '@amap/amap-jsapi-loader'
import { useRuntimeConfig } from '#app'

// 定义默认位置常量（南天门）
const DEFAULT_LOCATION = {
  latitude: DEFAULT_LATITUDE,
  longitude: DEFAULT_LONGITUDE,
  locationName: DEFAULT_LOCATION_NAME
}


defineOptions({
  name: 'GameMap'
})

interface MapLocation {
  latitude: number;
  longitude: number;
}

const props = defineProps({
  location: {
    type: Object as () => MapLocation,
    required: true
  },
  locationName: {
    type: String,
    default: ''
  }
})

const emit = defineEmits(['update:location', 'update:locationName'])

// 地图状态
const isLoading = ref(true)
const hasShownTip = ref(false)
const showContextMenu = ref(false)
const contextMenuPosition = ref<MapLocation>({ longitude: 0, latitude: 0 })
const contextMenuStyle = ref({
  left: '0',
  top: '0'
})

// 标记是否需要在下次位置更新时居中显示
const shouldCenterOnNextLocationUpdate = ref(true)

// 右键菜单点击
const handleMenuClick = () => {
  moveToClickPosition()
  // 触感反馈
  navigator.vibrate?.(30)
}

// 关闭提示
const dismissTip = () => {
  hasShownTip.value = true
  localStorage.setItem('map_tip_shown', 'true')
}

// 关闭右键菜单
const closeContextMenu = () => {
  showContextMenu.value = false
}

// AMap 实例和标记实例
let map: any = null
let marker: any = null

// 初始化地图
const initMap = async () => {
  isLoading.value = true
  try {
    await nextTick()
    // 动态设置安全配置
    const runtimeConfig = useRuntimeConfig()
    // 直接用 runtimeConfig 判断环境
    if (runtimeConfig.public.amapServiceSecurityCode) {
      (window as any)._AMapSecurityConfig = {
        securityJsCode: String(runtimeConfig.public.amapServiceSecurityCode)
      }
    } else if (runtimeConfig.public.amapServiceHost) {
      (window as any)._AMapSecurityConfig = {
        serviceHost: String(runtimeConfig.public.amapServiceHost)
      }
    }
    // 动态加载高德地图脚本
    const AMap = await AMapLoader.load({
      key: String(runtimeConfig.public.amapApiKey || ''),
      version: '2.0',
    })
    // 初始化地图
    map = new AMap.Map('map-container', {
      zoom: 14,
      center: [props.location.longitude, props.location.latitude],
      resizeEnable: true
    })
    marker = new AMap.Marker({
      position: [props.location.longitude, props.location.latitude],
      draggable: true,
      cursor: 'move',
      animation: 'AMAP_ANIMATION_DROP'
    })
    map.add(marker)
    marker.on('dragend', (e: any) => {
      const position = e.target.getPosition()
      handlePositionChange(position.lng, position.lat)
    })
    setupMapInteractions(AMap)
    map.on('complete', () => {
      isLoading.value = false
      console.log('地图加载完成')
    })
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : String(error)
    console.error('地图初始化失败:', errorMsg)
    ElMessage.error(`地图加载失败：${errorMsg}`)
    isLoading.value = false
  }
}

// 设置地图交互事件
const setupMapInteractions = (AMapInstance?: any) => {
  if (!map) {
    console.error('地图未初始化，无法设置交互事件')
    return
  }
  map.on('complete', () => {
    console.log('地图加载完成')
  })
  // 直接访问 document
  const mapContainer = document.getElementById('map-container')
  if (mapContainer) {
    mapContainer.addEventListener('contextmenu', (e) => {
      e.preventDefault()
      const rect = mapContainer.getBoundingClientRect()
      const x = e.clientX - rect.left
      const y = e.clientY - rect.top
      const mockLngLat = simulateGeoCoordinate(x, y)
      contextMenuPosition.value = {
        longitude: mockLngLat.lng,
        latitude: mockLngLat.lat
      }
      contextMenuStyle.value = {
        left: `${e.clientX}px`,
        top: `${e.clientY}px`
      }
      showContextMenu.value = true
    })
    document.addEventListener('click', () => {
      if (showContextMenu.value) {
        closeContextMenu()
        map.setStatus({ dragEnable: true })
      }
    })
    let longPressTimer: ReturnType<typeof setTimeout> | null = null
    mapContainer.addEventListener('touchstart', (e) => {
      longPressTimer = setTimeout(() => {
        const touch = e.touches[0]
        const rect = mapContainer.getBoundingClientRect()
        const x = touch.clientX - rect.left
        const y = touch.clientY - rect.top
        const mockLngLat = simulateGeoCoordinate(x, y)
        contextMenuPosition.value = {
          longitude: mockLngLat.lng,
          latitude: mockLngLat.lat
        }
        contextMenuStyle.value = {
          left: `${touch.clientX}px`,
          top: `${touch.clientY}px`
        }
        showContextMenu.value = true
        navigator.vibrate?.(30)
      }, 500)
    })
    mapContainer.addEventListener('touchend', () => {
      if (longPressTimer) {
        clearTimeout(longPressTimer)
        longPressTimer = null
      }
    })
    mapContainer.addEventListener('touchmove', () => {
      if (longPressTimer) {
        clearTimeout(longPressTimer)
        longPressTimer = null
      }
    })
  }
}

// 模拟从屏幕坐标到地理坐标的转换
// 实际项目中应该使用地图API提供的方法
const simulateGeoCoordinate = (x: number, y: number) => {
  // 这里使用一个非常简单的模拟，实际项目中应使用真实地图API
  const baseLocation = props.location
  const offsetX = (x / 300 - 0.5) * 0.02 // 模拟经度变化
  const offsetY = (0.5 - y / 300) * 0.02 // 模拟纬度变化

  return {
    lng: baseLocation.longitude + offsetX,
    lat: baseLocation.latitude + offsetY
  }
}

// 右键菜单点击后移动位置
const moveToClickPosition = async () => {
  if (!contextMenuPosition.value || !map) return
  try {
    const { longitude, latitude } = contextMenuPosition.value
    updateMapCenter({ latitude, longitude })
    let locationName = props.locationName
    try {
      const runtimeConfig = useRuntimeConfig()
      const AMap = await AMapLoader.load({
        key: String(runtimeConfig.public.amapApiKey || ''),
        version: '2.0',
      })
      const geocoder = new AMap.Geocoder()
      const result = await new Promise((resolve, reject) => {
        geocoder.getAddress([longitude, latitude], (status: string, result: any) => {
          if (status === 'complete' && result.regeocode) {
            resolve(result.regeocode.formattedAddress)
          } else {
            reject(new Error('获取位置名称失败'))
          }
        })
      })
      locationName = result as string
    } catch (error) {
      console.error('获取位置名称失败:', error)
      locationName = DEFAULT_LOCATION.locationName
    }
    emit('update:location', { latitude, longitude })
    emit('update:locationName', locationName)
    closeContextMenu()
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : String(error)
    console.error('移动到点击位置失败:', errorMsg)
    ElMessage.error(`设置位置失败：${errorMsg}`)
  }
}

// 更新地图中心到当前位置
const updateMapCenter = ({ latitude, longitude }: MapLocation) => {
  if (!map) return

  try {
    // 更新地图中心点
    map.setCenter([longitude, latitude])

    // 更新标记位置
    if (marker) {
      marker.setPosition([longitude, latitude])
    }
  } catch (error) {
    console.error('更新地图中心点失败:', error)
  }
}

// 缩放控制
const handleZoomIn = () => {
  if (map) map.zoomIn()
}

const handleZoomOut = () => {
  if (map) map.zoomOut()
}

// 处理地图中心与当前位置同步
const handleCenterToCurrentLocation = () => {
  shouldCenterOnNextLocationUpdate.value = true
  updateMapCenter(props.location)
}

// 监听位置变化，更新地图中心
watch(() => props.location, (newLocation) => {
  // 如果标记了需要居中显示，或者是初始加载时，则更新地图中心
  if (shouldCenterOnNextLocationUpdate.value && map && marker) {
    updateMapCenter(newLocation)
    shouldCenterOnNextLocationUpdate.value = false
  }
}, { deep: true })

// 处理位置变化
const handlePositionChange = (lng: number, lat: number) => {
  if (!map) return

  // 更新内部状态
  contextMenuPosition.value = { longitude: lng, latitude: lat }

  // 发送事件
  emit('update:location', { latitude: lat, longitude: lng })
}

// 组件挂载时初始化地图
onMounted(() => {
  hasShownTip.value = localStorage.getItem('map_tip_shown') === 'true'
  nextTick(() => {
    initMap()
  })
})

// 组件卸载时清理资源
onBeforeUnmount(() => {
  // 清理地图实例和相关事件
  map = null
  marker = null
})
</script>

<style lang="scss" scoped>
.game-map-container {
  position: relative;
  width: 100%;
  height: 300px;
  margin: 20px 0;
  border: 2px solid var(--zelda-border-medium);
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 4px 12px rgba(47, 79, 45, 0.15);

  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 5px;
    background-color: var(--zelda-primary);
    z-index: 5;
  }
}

.map-container {
  position: relative;
  width: 100%;
  height: 100%;
  background-color: #e8f0d8; // 塞尔达风格的浅绿色
  background-image: url('data:image/svg+xml;utf8,<svg width="100" height="100" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg"><path d="M0 0h100v100H0z" fill="%23e8f0d8"/><path d="M0 0h50v50H0zM50 50h50v50H50z" fill="%23dbe8c9"/></svg>');
  overflow: hidden;
  cursor: grab;

  &:active {
    cursor: grabbing;
  }
}

.map-controls {
  position: absolute;
  right: 15px;
  top: 15px;
  z-index: 10;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.control-button {
  width: 40px;
  height: 40px;
  border-radius: 4px;
  background-color: var(--zelda-bg-light);
  border: 2px solid var(--zelda-primary);
  display: flex;
  justify-content: center;
  align-items: center;
  box-shadow: 0 2px 6px rgba(47, 79, 45, 0.15);
  cursor: pointer;
  transition: all 0.2s ease;

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 10px rgba(47, 79, 45, 0.2);
    background-color: var(--zelda-primary-light);
  }

  &:active {
    transform: translateY(0);
  }

  .cartoon-icon {
    font-size: 20px;
    color: var(--zelda-primary-dark);
  }
}

.map-context-menu {
  position: fixed;
  z-index: 1000;
  background-color: var(--zelda-bg-light);
  width: 150px;
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  border: 2px solid var(--zelda-primary);
  overflow: hidden;

  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 3px;
    background-color: var(--zelda-secondary);
  }

  .menu-item {
    padding: 10px;
    display: flex;
    align-items: center;
    cursor: pointer;
    transition: background-color 0.3s;

    &:hover {
      background-color: var(--zelda-bg-medium);
    }

    i {
      margin-right: 8px;
      font-size: 16px;
    }

    span {
      font-size: 14px;
      color: var(--zelda-text-dark);
    }
  }
}

.map-tip {
  position: absolute;
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
  background-color: rgba(255, 255, 255, 0.9);
  border: 2px solid var(--zelda-secondary);
  border-radius: 6px;
  padding: 10px 15px;
  display: flex;
  align-items: center;
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.2);
  max-width: 90%;
  z-index: 5;

  i {
    font-size: 18px;
    margin-right: 8px;
    color: var(--zelda-secondary);
  }

  span {
    flex: 1;
    font-size: 14px;
    color: var(--zelda-text-dark);
  }

  .zelda-btn {
    margin-left: 10px;
    color: var(--zelda-primary);
    font-weight: bold;

    &:hover {
      color: var(--zelda-primary-dark);
    }
  }
}

.map-loading,
.map-loading-fallback {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  background-color: rgba(232, 240, 216, 0.8);
  z-index: 5;

  i {
    font-size: 40px;
    margin-bottom: 15px;
  }

  span {
    font-size: 16px;
    color: var(--zelda-text-dark);
  }
}
</style>