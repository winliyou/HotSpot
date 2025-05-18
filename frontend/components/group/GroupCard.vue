<template>
  <div class="group-card" @click="$emit('click', group)">
    <div class="group-avatar">
      <el-avatar :src="group.avatar_url || defaultAvatar" size="large">{{ group.name?.slice(0, 1) }}</el-avatar>
    </div>
    <div class="group-info">
      <div class="group-title">{{ group.name }}</div>
      <div class="group-desc">{{ group.description || '暂无简介' }}</div>
      <div class="group-meta">
        <span><el-icon><UserFilled /></el-icon> {{ group.member_count || 0 }}</span>
        <span v-if="group.distance">· <el-icon><Location /></el-icon> {{ (group.distance / 1000).toFixed(1) }}km</span>
        <span v-if="group.is_member" class="joined-tag">已加入</span>
      </div>
      <div class="group-actions">
        <el-button v-if="!group.is_member" size="small" type="primary" @click.stop="onJoin">加入</el-button>
        <el-button v-else size="small" @click.stop="onQuit">退出</el-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { UserFilled, Location } from '@element-plus/icons-vue';
import { groupService } from '~/utils/services/group-service';
import { ElMessage } from 'element-plus';
import { ref } from 'vue';
import type { GroupInfo } from '~/types/group';

const props = defineProps<{ group: GroupInfo }>();
const defaultAvatar = '/images/group-default.png';

const onJoin = async () => {
  try {
    await groupService.joinGroup({ group_id: props.group.group_id });
    ElMessage.success('已加入群组');
    // 可 emit 刷新事件
  } catch (e: any) {
    ElMessage.error(e.message || '加入失败');
  }
};
const onQuit = async () => {
  try {
    await groupService.leaveGroup({ group_id: props.group.group_id });
    ElMessage.success('已退出群组');
    // 可 emit 刷新事件
  } catch (e: any) {
    ElMessage.error(e.message || '退出失败');
  }
};
</script>

<style scoped>
.group-card {
  display: flex;
  align-items: center;
  padding: 16px;
  border-radius: 10px;
  background: #f8fafc;
  box-shadow: 0 2px 8px rgba(30,136,229,0.06);
  cursor: pointer;
  transition: box-shadow 0.2s;
}
.group-card:hover {
  box-shadow: 0 4px 16px rgba(30,136,229,0.12);
}
.group-avatar {
  margin-right: 16px;
}
.group-title {
  font-weight: bold;
  font-size: 18px;
  color: #1e88e5;
}
.group-desc {
  font-size: 14px;
  color: #666;
  margin: 4px 0 8px 0;
}
.group-meta {
  font-size: 12px;
  color: #999;
  display: flex;
  gap: 12px;
  align-items: center;
}
.joined-tag {
  color: #67c23a;
  font-weight: bold;
  margin-left: 8px;
}
.group-actions {
  margin-top: 8px;
  display: flex;
  gap: 8px;
}
</style>
