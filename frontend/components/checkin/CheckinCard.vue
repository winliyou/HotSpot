<template>
  <div class="checkin-card">
    <div class="card-header">
      <div class="user-info" @click="navigateTo(`/user/${checkin.user_id}`)">
        <el-avatar :size="40">{{ checkin.nickname.charAt(0) }}</el-avatar>
        <div class="user-details">
          <div class="user-name">{{ checkin.nickname }}</div>
          <div class="checkin-time">{{ formatTime(checkin.created_at) }}</div>
        </div>
      </div>

      <div class="checkin-location" @click="showLocation">
        <el-tooltip :content="checkin.location_name" placement="top">
          <span class="location-text">{{ checkin.location_name }}</span>
        </el-tooltip>
        <span class="location-distance text-secondary">{{ formatDistance(checkin.distance) }}</span>
      </div>
    </div>

    <div class="card-content" @click="navigateTo(`/checkin/${checkin.checkin_id}`)">
      <div class="checkin-text">{{ checkin.description }}</div>

      <div class="checkin-tags" v-if="checkin.tags && checkin.tags.length">
        <el-tag v-for="tag in checkin.tags" :key="tag" size="small" class="tag-item" @click.stop="searchByTag(tag)">
          {{ tag }}
        </el-tag>
      </div>
    </div>

    <div class="card-footer">
      <div class="action-buttons">
        <el-button :type="checkin.liked_by_me ? 'primary' : 'default'" size="small" link @click="toggleLike">
          <el-icon>
            <Star />
          </el-icon>
          赞 {{ checkin.likes_count > 0 ? checkin.likes_count : '' }}
        </el-button>

        <el-button type="default" size="small" link @click="navigateTo(`/checkin/${checkin.checkin_id}`)">
          <el-icon>
            <ChatDotRound />
          </el-icon>
          评论 {{ checkin.comments_count > 0 ? checkin.comments_count : '' }}
        </el-button>

        <el-dropdown v-if="isCurrentUser" trigger="click" @command="handleCommand">
          <el-button type="default" size="small" link>
            <el-icon>
              <MoreFilled />
            </el-icon>
          </el-button>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="delete">删除</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Star, ChatDotRound, MoreFilled } from '@element-plus/icons-vue'
import { type CheckInInfo } from '~/types/checkin'
import { useAuthStore } from '~/stores/auth'
import { useCheckin } from '~/composables/useCheckin'
import { ElMessageBox } from 'element-plus'

const authStore = useAuthStore()
const { likeCheckin, unlikeCheckin, deleteCheckin } = useCheckin()

// 组件属性
const props = defineProps<{
  checkin: CheckInInfo
}>()

// 是否为当前用户的签到
const isCurrentUser = computed(() => {
  return authStore.user?.user_id === props.checkin.user_id
})

// 切换点赞状态
async function toggleLike() {
  if (!authStore.isAuthenticated) {
    navigateTo('/login')
    return
  }

  try {
    if (props.checkin.liked_by_me) {
      await unlikeCheckin(props.checkin.checkin_id)
    } else {
      await likeCheckin(props.checkin.checkin_id)
    }
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : String(error)
    console.error('点赞操作失败', errorMsg)
  }
}

// 删除签到
async function deleteCheckinHandler() {
  try {
    await ElMessageBox.confirm(
      '确定要删除这条签到吗？此操作不可恢复。',
      '删除确认',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )

    await deleteCheckin(props.checkin.checkin_id)
  } catch (error) {
    if (error !== 'cancel') {
      const errorMsg = error instanceof Error ? error.message : String(error)
      console.error('删除签到失败', errorMsg)
    }
  }
}

// 下拉菜单命令处理
function handleCommand(command: string) {
  if (command === 'delete') {
    deleteCheckinHandler()
  }
}

// 显示位置
function showLocation() {
  // 实际应该跳转到地图页面或显示地图对话框
  navigateTo({
    path: '/map',
    query: {
      lat: props.checkin.latitude.toString(),
      lng: props.checkin.longitude.toString(),
      name: props.checkin.location_name
    }
  })
}

// 按标签搜索
function searchByTag(tag: string) {
  navigateTo({
    path: '/checkin/search',
    query: { tag }
  })
}

// 格式化时间
function formatTime(timeString: string): string {
  const date = new Date(timeString)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffSec = Math.floor(diffMs / 1000)
  const diffMin = Math.floor(diffSec / 60)
  const diffHour = Math.floor(diffMin / 60)
  const diffDay = Math.floor(diffHour / 24)

  if (diffSec < 60) {
    return '刚刚'
  } else if (diffMin < 60) {
    return `${diffMin}分钟前`
  } else if (diffHour < 24) {
    return `${diffHour}小时前`
  } else if (diffDay < 30) {
    return `${diffDay}天前`
  } else {
    return date.toLocaleDateString()
  }
}

// 格式化距离
function formatDistance(meters: number): string {
  if (meters < 1000) {
    return `${Math.round(meters)}米`
  } else {
    return `${(meters / 1000).toFixed(1)}公里`
  }
}
</script>

<style lang="scss" scoped>
.checkin-card {
  background-color: var(--background-color);
  border-radius: var(--border-radius);
  box-shadow: var(--card-shadow);
  overflow: hidden;
  transition: box-shadow var(--transition-speed);

  &:hover {
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 15px 15px 0;

    .user-info {
      display: flex;
      align-items: center;
      cursor: pointer;

      .user-details {
        margin-left: 10px;

        .user-name {
          font-weight: 500;
          font-size: 0.95rem;
        }

        .checkin-time {
          font-size: 0.8rem;
          color: var(--text-color-secondary);
        }
      }
    }

    .checkin-location {
      display: flex;
      flex-direction: column;
      align-items: flex-end;
      font-size: 0.85rem;
      cursor: pointer;

      .location-text {
        max-width: 150px;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
      }

      .location-distance {
        font-size: 0.75rem;
        margin-top: 3px;
      }
    }
  }

  .card-content {
    padding: 15px;
    cursor: pointer;

    .checkin-text {
      margin-bottom: 12px;
      line-height: 1.5;
      word-break: break-word;
    }

    .checkin-tags {
      display: flex;
      flex-wrap: wrap;
      gap: 8px;

      .tag-item {
        cursor: pointer;
      }
    }
  }

  .card-footer {
    border-top: 1px solid var(--border-color);
    padding: 10px 15px;

    .action-buttons {
      display: flex;
      justify-content: space-around;
    }
  }
}

@media (max-width: var(--sm)) {
  .checkin-card {
    .card-header {
      flex-direction: column;

      .checkin-location {
        align-items: flex-start;
        margin-top: 10px;
        margin-left: 50px;
      }
    }
  }
}
</style>