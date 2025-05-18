<template>
  <div class="auth-section app-container">
    <h2 class="app-title">用户登录</h2>
    <el-tabs v-model="activeTab" stretch class="auth-tabs">
      <el-tab-pane label="登录" name="login">
        <el-form :model="loginForm" :rules="loginRules" class="auth-form" @submit.prevent="handleLogin">
          <el-form-item prop="login_id">
            <el-input v-model="loginForm.login_id" prefix-icon="User" placeholder="账号/邮箱/手机号" clearable />
          </el-form-item>
          <el-form-item prop="password">
            <el-input v-model="loginForm.password" prefix-icon="Lock" type="password" placeholder="密码" show-password
              clearable />
          </el-form-item>
          <el-form-item>
            <el-button class="auth-btn" type="primary" native-type="submit" :loading="loading" block>登录</el-button>
          </el-form-item>
          <div class="auth-switch">
            <span>还没有账号？<a @click.prevent="switchToRegister">注册</a></span>
            <span class="ml-2"><a @click.prevent="activeTab = 'temp'">临时体验</a></span>
          </div>
        </el-form>
      </el-tab-pane>
      <el-tab-pane label="注册" name="register">
        <el-form :model="registerForm" :rules="registerRules" class="auth-form" @submit.prevent="handleRegister">
          <el-form-item prop="login_id">
            <el-input v-model="registerForm.login_id" prefix-icon="User" placeholder="用户ID" clearable />
          </el-form-item>
          <el-form-item prop="nickname">
            <el-input v-model="registerForm.nickname" prefix-icon="Avatar" placeholder="昵称" clearable />
          </el-form-item>
          <el-form-item prop="email">
            <el-input v-model="registerForm.email" prefix-icon="Message" placeholder="邮箱" clearable />
          </el-form-item>
          <el-form-item prop="password">
            <el-input v-model="registerForm.password" prefix-icon="Lock" type="password" placeholder="密码" show-password
              clearable />
          </el-form-item>
          <el-form-item prop="confirm_password">
            <el-input v-model="registerForm.confirm_password" prefix-icon="Check" type="password" placeholder="确认密码"
              show-password clearable />
          </el-form-item>
          <el-form-item>
            <el-button class="auth-btn" type="primary" native-type="submit" :loading="loading" block>注册</el-button>
          </el-form-item>
          <div class="auth-switch">
            <span>已有账号？<a @click.prevent="switchToLogin">登录</a></span>
          </div>
        </el-form>
      </el-tab-pane>
      <el-tab-pane label="临时体验" name="temp">
        <el-form :model="tempUserForm" class="auth-form" @submit.prevent="handleCreateTempUser">
          <el-form-item prop="nickname">
            <el-input v-model="tempUserForm.nickname" prefix-icon="Avatar" placeholder="昵称（2-20字）" clearable />
          </el-form-item>
          <el-form-item>
            <el-button class="auth-btn" type="primary" native-type="submit" :loading="loading" block>一键体验</el-button>
          </el-form-item>
          <div class="auth-switch">
            <span>已有账号？<a @click.prevent="switchToLogin">登录</a></span>
          </div>
        </el-form>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, watchEffect } from 'vue'
import { useAuthStore } from '~/stores/auth'
import type { LoginRequest, CreateTempUserRequest, RegisterRequest } from '~/types/user'
import { ElMessage } from 'element-plus'
import { useRoute } from 'vue-router'
import { User, Lock, Avatar, Message, Check } from '@element-plus/icons-vue'
import { showErrorMessage } from '~/utils/error'

// 定义页面元数据，设置页面别名
definePageMeta({
  alias: ['/login', '/register']
})

const authStore = useAuthStore()
const router = useRouter()
const route = useRoute()

// 如果已经登录，直接跳转到首页
if (process.client && authStore.isAuthenticated) {
  router.push('/')
}

// 根据路由路径设置初始选中的标签
const activeTab = ref('login')

// 处理不同的路由路径
watchEffect(() => {
  if (route.path === '/login') {
    activeTab.value = 'login'
  } else if (route.path === '/register') {
    activeTab.value = 'register'
  } else if (route.query.tab && ['login', 'register', 'temp'].includes(route.query.tab as string)) {
    // 如果URL中有指定tab参数，则切换到对应标签
    activeTab.value = route.query.tab as string
  }
})

const loading = ref(false)

// 登录表单
const loginForm = reactive<LoginRequest>({
  login_id: '',
  password: ''
})

// 注册表单
const registerForm = reactive<RegisterRequest>({
  login_id: '',
  nickname: '',
  email: '',
  password: '',
  confirm_password: ''
})

// 临时用户表单
const tempUserForm = reactive<CreateTempUserRequest>({
  nickname: ''
})

// 登录表单验证规则
const loginRules = {
  login_id: [
    { required: true, message: '请输入账号', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' }
  ]
}

// 注册表单验证规则
const registerRules = {
  login_id: [
    { required: true, message: '请输入用户ID', trigger: 'blur' },
    { min: 3, max: 20, message: '用户ID长度应在3-20个字符之间', trigger: 'blur' }
  ],
  nickname: [
    { required: true, message: '请输入昵称', trigger: 'blur' },
    { min: 2, max: 20, message: '昵称长度应在2-20个字符之间', trigger: 'blur' }
  ],
  email: [
    { required: true, message: '请输入邮箱', trigger: 'blur' },
    { pattern: /^[^\s@]+@[^\s@]+\.[^\s@]+$/, message: '请输入正确的邮箱格式', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, message: '密码长度最少为6个字符', trigger: 'blur' }
  ],
  confirm_password: [
    { required: true, message: '请再次输入密码', trigger: 'blur' },
    {
      validator: (rule: any, value: string, callback: Function) => {
        if (value !== registerForm.password) {
          callback(new Error('两次输入的密码不一致'))
        } else {
          callback()
        }
      },
      trigger: 'blur'
    }
  ]
}

// 登录处理
async function handleLogin() {
  try {
    loading.value = true
    await authStore.login(loginForm)
    ElMessage.success('登录成功！')
    router.push('/')
  } catch (error) {
    showErrorMessage(error, '登录失败，请重试')
  } finally {
    loading.value = false
  }
}

// 注册处理
async function handleRegister() {
  try {
    loading.value = true
    await authStore.register(registerForm)
    ElMessage.success('注册成功！')
    activeTab.value = 'login'
  } catch (error) {
    showErrorMessage(error, '注册失败，请重试')
  } finally {
    loading.value = false
  }
}

// 创建临时用户
async function handleCreateTempUser() {
  try {
    loading.value = true
    await authStore.createTempUser(tempUserForm)
    ElMessage.success('临时用户创建成功！')
    router.push('/')
    tempUserForm.nickname = '' // 只在创建成功后手动清空
  } catch (error) {
    showErrorMessage(error, '创建临时用户失败，请重试')
  } finally {
    loading.value = false
  }
}

// 切换到注册标签
function switchToRegister() {
  activeTab.value = 'register'
}

// 切换到登录标签
function switchToLogin() {
  activeTab.value = 'login'
}
</script>

<style lang="scss" scoped>
.auth-section {
  background: #fff;
  border-radius: 16px;
  box-shadow: 0 4px 16px rgba(30, 136, 229, 0.08);
  padding: 32px 24px;
  margin: 40px auto;
  max-width: 400px;
}

.app-title {
  font-size: 32px;
  color: #1e88e5;
  font-family: 'Baloo 2', 'PingFang SC', 'Arial', sans-serif;
  margin-bottom: 24px;
  text-align: center;
}

.auth-btn {
  border-radius: 10px;
  font-family: 'Baloo 2', 'PingFang SC', 'Arial', sans-serif;
  text-transform: uppercase;
  letter-spacing: 1px;
  border: 2px solid #1e88e5;
  background: #fff;
  color: #1e88e5;
  font-weight: 600;
  box-shadow: 0 2px 8px rgba(30, 136, 229, 0.08);
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

.auth-switch {
  display: flex;
  justify-content: space-between;
  font-size: 14px;
  margin-top: 8px;
  color: #90caf9;

  a {
    color: #1e88e5;
    cursor: pointer;
    text-decoration: underline;
    margin-left: 4px;
  }
}

.auth-tabs {
  margin-top: 12px;
}
</style>