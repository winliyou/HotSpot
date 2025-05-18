<template>
  <div class="group-list">
    <el-skeleton v-if="loading" :rows="3" animated>
      <template #template>
        <div class="group-card-skeleton">
          <el-skeleton-item variant="circle" style="width:48px;height:48px;margin-right:16px;" />
          <div class="group-card-skeleton-info">
            <el-skeleton-item variant="text" style="width:120px;height:18px;margin-bottom:8px;" />
            <el-skeleton-item variant="text" style="width:180px;height:14px;" />
          </div>
        </div>
      </template>
    </el-skeleton>
    <template v-else>
      <div v-if="groups.length === 0" class="group-list-empty">
        <el-empty description="暂无群组">
          <template #image>
            <el-icon :size="48" class="empty-icon"><UserFilled /></el-icon>
          </template>
          <template #description>
            <div>还没有发现任何群组</div>
            <el-button type="primary" @click="$emit('create-group')">创建群组</el-button>
          </template>
        </el-empty>
      </div>
      <div v-else>
        <GroupCard
          v-for="group in groups"
          :key="group.group_id"
          :group="group"
          @click="$emit('group-click', group)"
        />
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';
import GroupCard from './GroupCard.vue';
import { UserFilled } from '@element-plus/icons-vue';
import type { GroupInfo } from '~/types/group';

const props = defineProps<{
  groups: GroupInfo[];
  loading?: boolean;
}>();
const emit = defineEmits(['group-click', 'create-group']);
</script>

<style scoped>
.group-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.group-list-empty {
  padding: 32px 0;
}
.group-card-skeleton {
  display: flex;
  align-items: center;
  padding: 16px 0;
}
.group-card-skeleton-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.empty-icon {
  color: #b2becd;
}
</style>
