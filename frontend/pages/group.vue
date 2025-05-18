<template>
  <div class="group-section app-container">
    <h2 class="app-title">群组</h2>
    <div class="group-actions">
      <el-button class="app-btn" type="primary" @click="fetchNearbyGroups">发现附近群组</el-button>
      <el-button class="app-btn" @click="fetchUserGroups">我的群组</el-button>
      <el-input v-model="searchKeyword" placeholder="搜索群组" class="group-search" clearable @clear="onSearch" @keyup.enter="onSearch" />
    </div>
    <GroupList :groups="groups" :loading="loading" @group-click="handleGroupClick" @create-group="handleCreateGroup" />
    <div class="group-pagination" v-if="hasMore">
      <el-button @click="loadMoreGroups" :loading="loading">加载更多</el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import GroupList from '~/components/group/GroupList.vue';
import { groupService } from '~/utils/services/group-service';
import { useUserStore } from '~/stores/user';
import type { GroupInfo } from '~/types/group';
import { ElMessage } from 'element-plus';

const groups = ref<GroupInfo[]>([]);
const loading = ref(false);
const hasMore = ref(false);
const cursor = ref<number | undefined>(undefined);
const userStore = useUserStore();
const searchKeyword = ref('');

// 直接用 userStore.currentLocation，兼容 ref 和 null
function getLocationForApi() {
  // userStore.currentLocation 只会是 null 或 { latitude, longitude, locationName }
  const loc = userStore.currentLocation;
  if (loc && typeof loc === 'object' && 'latitude' in loc && 'longitude' in loc) {
    return loc;
  }
  return { latitude: 0, longitude: 0 };
}

const fetchNearbyGroups = async () => {
  loading.value = true;
  try {
    const loc = getLocationForApi();
    const res = await groupService.getNearbyGroups(loc.latitude, loc.longitude, 5000, undefined, 10);
    groups.value = res.items;
    hasMore.value = res.pagination.has_more;
    cursor.value = res.pagination.next_cursor;
  } finally {
    loading.value = false;
  }
};

const fetchUserGroups = async () => {
  loading.value = true;
  try {
    const res = await groupService.getUserGroups();
    groups.value = res.items;
    hasMore.value = res.pagination.has_more;
    cursor.value = res.pagination.next_cursor;
  } finally {
    loading.value = false;
  }
};

const loadMoreGroups = async () => {
  if (!cursor.value) return;
  loading.value = true;
  try {
    const loc = getLocationForApi();
    const res = await groupService.getNearbyGroups(loc.latitude, loc.longitude, 5000, cursor.value, 10);
    groups.value = [...groups.value, ...res.items];
    hasMore.value = res.pagination.has_more;
    cursor.value = res.pagination.next_cursor;
  } finally {
    loading.value = false;
  }
};

const onSearch = async () => {
  if (!searchKeyword.value) {
    fetchNearbyGroups();
    return;
  }
  loading.value = true;
  try {
    // 假设 groupService 有按名称搜索接口
    const res = await groupService.findGroupsByName(searchKeyword.value, undefined, 10);
    groups.value = res.items;
    hasMore.value = res.pagination.has_more;
    cursor.value = res.pagination.next_cursor;
  } catch (e: any) {
    ElMessage.error(e.message || '搜索失败');
  } finally {
    loading.value = false;
  }
};

const handleGroupClick = (group: GroupInfo) => {
  // 跳转到群组详情页
  navigateTo(`/group/${group.group_id}`);
};
const handleCreateGroup = () => {
  // 跳转到创建群组页面
  navigateTo('/group/create');
};

// 页面加载时自动拉取附近群组
fetchNearbyGroups();
</script>

<style lang="scss" scoped>
.group-section {
  background: #fff;
  border-radius: 16px;
  box-shadow: 0 4px 16px rgba(30,136,229,0.08);
  padding: 32px 24px;
  margin: 40px auto;
  max-width: 700px;
}
.app-title {
  font-size: 32px;
  color: #1e88e5;
  font-family: 'Baloo 2', 'PingFang SC', 'Arial', sans-serif;
  margin-bottom: 24px;
  text-align: center;
}
.group-list {
  margin-bottom: 32px;
}
.group-actions {
  text-align: center;
  display: flex;
  gap: 12px;
  margin-bottom: 24px;
  flex-wrap: wrap;
  align-items: center;
  justify-content: center;
}
.group-search {
  max-width: 220px;
}
.group-pagination {
  text-align: center;
  margin-top: 16px;
}
.app-btn {
  border-radius: 10px;
  font-family: 'Baloo 2', 'PingFang SC', 'Arial', sans-serif;
  text-transform: uppercase;
  letter-spacing: 1px;
  border: 2px solid #1e88e5;
  background: #fff;
  color: #1e88e5;
  font-weight: 600;
  box-shadow: 0 2px 8px rgba(30,136,229,0.08);
  transition: all 0.2s;
  position: relative;
  overflow: hidden;
  &:hover {
    background: #b2ebf2;
    color: #ffb300;
    border-color: #ffb300;
    transform: translateY(-2px) scale(1.04);
  }
}
</style>