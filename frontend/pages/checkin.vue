<template>
  <div class="checkin-page-redirect">
    <el-result icon="info" title="页面已迁移" sub-title="请在首页通过弹窗发起签到">
      <template #extra>
        <el-button type="primary" @click="goHome">返回首页</el-button>
      </template>
    </el-result>
  </div>
</template>

<script setup lang="ts">
const router = useRouter()
function goHome() {
  router.push('/')
}
</script>

<style scoped>
.checkin-page-redirect {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 60vh;
}
</style>