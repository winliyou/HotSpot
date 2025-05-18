<template>
  <el-dialog v-model="visible" title="签到" width="480px" :close-on-click-modal="false" @close="onClose">
    <el-form
      ref="checkinForm$"
      :model="checkinForm"
      :rules="checkinRules"
      label-position="top"
      @submit.prevent="handleCreateCheckin"
    >
      <el-form-item label="描述" prop="description">
        <el-input 
          :model-value="checkinForm.description" 
          @update:model-value="checkinForm.description = $event" 
          type="textarea" 
          :rows="3" 
          placeholder="分享你的经历、心情或想法..." 
          maxlength="1000" 
          show-word-limit />
      </el-form-item>
      <el-form-item label="位置">
        <div class="location-card">
          <div v-if="userStore.hasLocation" class="current-location">
            <div class="location-name">
              <el-icon><Location /></el-icon>
              {{ userStore.getCurrentLocation?.locationName }}
            </div>
            <el-button link @click="refreshLocation">刷新位置</el-button>
          </div>
          <div v-else class="no-location">
            <p>未获取位置信息</p>
            <el-button type="primary" size="small" @click="getLocation" :loading="isGettingLocation">
              获取我的位置
            </el-button>
          </div>
        </div>
      </el-form-item>
      <el-form-item label="标签">
        <el-select
          v-model="selectedTags"
          multiple
          filterable
          allow-create
          default-first-option
          placeholder="添加标签"
          :max="10"
        >
          <el-option
            v-for="tag in commonTags"
            :key="tag"
            :label="tag"
            :value="tag"
          />
        </el-select>
        <div class="tags-hint text-secondary">
          <small>最多可添加10个标签，每个标签不超过20个字符</small>
        </div>
      </el-form-item>
      <div class="form-actions">
        <el-button 
          type="primary" 
          native-type="submit" 
          :loading="loading"
          :disabled="!userStore.hasLocation"
          class="checkin-submit-btn"
          @click="handleCreateCheckin"
        >
          发布签到
        </el-button>
      </div>
    </el-form>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { Location } from '@element-plus/icons-vue'
import { useAuthStore } from '~/stores/auth'
import { useUserStore } from '~/stores/user'
import { useCheckinStore } from '~/stores/checkin'
import type { CreateCheckinRequest } from '~/types/checkin'
import { ElMessage } from 'element-plus'

const props = defineProps<{ modelValue: boolean }>()
const emit = defineEmits(['update:modelValue', 'success'])

const visible = ref(props.modelValue)
watch(() => props.modelValue, v => visible.value = v)
watch(visible, v => emit('update:modelValue', v))

const authStore = useAuthStore()
const userStore = useUserStore()
const checkinStore = useCheckinStore()

const loading = ref(false)
const isGettingLocation = ref(false)
const selectedTags = ref<string[]>([])

const checkinForm = reactive<Omit<CreateCheckinRequest, 'tags' | 'latitude' | 'longitude' | 'location_name'>>({
  description: ''
})

const checkinRules = {
  description: [
    { required: true, message: '请输入签到描述', trigger: 'blur' },
    { min: 1, max: 1000, message: '描述长度应在1-1000个字符之间', trigger: 'blur' }
  ]
}

const commonTags = [
  '旅行', '美食', '风景', '运动', '工作', '学习', '生活', '打卡',
  '电影', '音乐', '阅读', '购物', 'party', '展览', '演出', '约会'
]

const checkinForm$ = ref()

async function handleCreateCheckin() {
  const valid = await checkinForm$.value.validate().catch(() => false)
  if (!valid) return
  if (!userStore.hasLocation) {
    ElMessage.warning('请先获取位置信息')
    return
  }
  loading.value = true
  try {
    const location = userStore.getCurrentLocation!
    const createData: CreateCheckinRequest = {
      description: checkinForm.description,
      latitude: location.latitude,
      longitude: location.longitude,
      location_name: location.locationName,
      tags: selectedTags.value.length > 0 ? selectedTags.value : undefined
    }
    await checkinStore.createCheckin(createData)
    ElMessage.success('签到成功')
    emit('success')
    visible.value = false
    resetForm()
  } catch (error: any) {
    ElMessage.error(error.message || '创建签到失败')
  } finally {
    loading.value = false
  }
}

async function getLocation() {
  isGettingLocation.value = true
  try {
    const coords = await userStore.getDeviceLocation()
    if (coords) {
      userStore.setCurrentLocation(
        coords.latitude,
        coords.longitude,
        '当前位置'
      )
      if (authStore.isAuthenticated) {
        await userStore.updateLocationToServer()
      }
    }
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : String(error)
    ElMessage.error(`获取位置失败：${errorMsg}`)
  } finally {
    isGettingLocation.value = false
  }
}

function refreshLocation() {
  getLocation()
}

function resetForm() {
  checkinForm.description = ''
  selectedTags.value = []
}

function onClose() {
  resetForm()
}
</script>

<style scoped lang="scss">
.location-card {
  background: linear-gradient(90deg, #b2ebf2 60%, #fffde7 100%);
  border-radius: 14px;
  box-shadow: 0 2px 8px rgba(30,136,229,0.10);
  padding: 14px 18px;
  display: flex;
  flex-direction: column;
  gap: 6px;
  border: 1.5px solid #1e88e5;
}
.current-location {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 15px;
  color: #1976d2;
  font-weight: 600;
}
.location-name {
  display: flex;
  align-items: center;
  gap: 6px;
}
.no-location {
  color: #bdbdbd;
  font-size: 15px;
  font-weight: 500;
}
.tags-hint {
  margin-top: 4px;
  color: #90caf9;
}
.form-actions {
  margin-top: 18px;
}
.checkin-submit-btn {
  width: 100%;
  font-size: 18px;
  border-radius: 14px !important;
  font-family: 'Baloo 2', 'PingFang SC', 'Arial', sans-serif;
  font-weight: 700;
  letter-spacing: 2px;
  box-shadow: 0 4px 16px rgba(255,179,0,0.10);
  padding: 12px 0;
}
</style>
