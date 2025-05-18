<template>
  <div class="layout-container">
    <header class="app-header">
      <div class="container header-content">
        <div class="logo-section">
          <div class="logo">
            <nuxt-link to="/">
              <span class="logo-text">HotSpot</span>
              <span class="logo-icon">📍</span>
            </nuxt-link>
          </div>
          <div class="nav-menu hide-sm" v-if="isLoggedIn">
            <nuxt-link to="/" class="nav-item">首页</nuxt-link>
            <nuxt-link to="/checkin" class="nav-item">打卡</nuxt-link>
            <nuxt-link to="/group" class="nav-item">小组</nuxt-link>
            <nuxt-link to="/message" class="nav-item">消息</nuxt-link>
          </div>
        </div>
        
        <div class="user-section">
          <div class="auth-buttons" v-if="!isLoggedIn">
            <nuxt-link to="/login" class="login-btn cartoon-btn">登录</nuxt-link>
          </div>
          <div class="user-menu" v-else>
            <el-dropdown trigger="click">
              <div class="user-dropdown">
                <span class="user-nickname">{{ nickname }}</span>
                <el-avatar :size="36" class="user-avatar">{{ nickname?.charAt(0) }}</el-avatar>
              </div>
              <template #dropdown>
                <el-dropdown-menu class="cartoon-dropdown">
                  <el-dropdown-item @click="navigateTo('/user/profile')">
                    <i class="menu-icon">👤</i> 个人资料
                  </el-dropdown-item>
                  <el-dropdown-item @click="navigateTo('/user/settings')">
                    <i class="menu-icon">⚙️</i> 设置
                  </el-dropdown-item>
                  <el-dropdown-item divided @click="handleLogout">
                    <i class="menu-icon">🚪</i> 退出登录
                  </el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </div>
          <div class="mobile-menu-toggle show-sm" @click="toggleMobileMenu">
            <i class="menu-icon">☰</i>
          </div>
        </div>
      </div>
      
      <div class="mobile-menu show-sm" v-if="mobileMenuVisible">
        <nuxt-link to="/" class="mobile-nav-item" @click="closeMobileMenu">首页</nuxt-link>
        <nuxt-link to="/checkin" class="mobile-nav-item" @click="closeMobileMenu">打卡</nuxt-link>
        <nuxt-link to="/group" class="mobile-nav-item" @click="closeMobileMenu">小组</nuxt-link>
        <nuxt-link to="/message" class="mobile-nav-item" @click="closeMobileMenu">消息</nuxt-link>
      </div>
    </header>
    
    <main class="content-container">
      <slot />
    </main>
    
    <footer class="app-footer">
      <div class="container">
        <div class="footer-content">
          <div class="footer-logo">
            <span class="logo-text">HotSpot</span>
            <span class="logo-icon">📍</span>
          </div>
          <p class="copyright">&copy; 2025 HotSpot - 分享生活中的每一个精彩瞬间</p>
        </div>
      </div>
    </footer>
    
    <client-only>
      <el-backtop class="cartoon-backtop" />
    </client-only>
  </div>
</template>

<script setup lang="ts">
import '~/assets/styles/main.scss'
import { ref, computed } from 'vue'
import { useAuthStore } from '~/stores/auth'

const authStore = useAuthStore()
const mobileMenuVisible = ref(false)

const isLoggedIn = computed(() => authStore.isAuthenticated)
const nickname = computed(() => authStore.user?.nickname)

function toggleMobileMenu() {
  mobileMenuVisible.value = !mobileMenuVisible.value
}

function closeMobileMenu() {
  mobileMenuVisible.value = false
}

function handleLogout() {
  authStore.logout()
  navigateTo('/login')
}
</script>

<style lang="scss" scoped>
.layout-container {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: var(--cartoon-bg-light);
}

.app-header {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: var(--header-height);
  background-color: white;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  z-index: 1000;
  border-bottom: 3px solid var(--cartoon-primary);
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 100%;
}

.logo-section {
  display: flex;
  align-items: center;
}

.logo {
  display: flex;
  align-items: center;
  
  a {
    display: flex;
    align-items: center;
    text-decoration: none;
  }
  
  .logo-text {
    font-size: 1.8rem;
    font-weight: 700;
    color: var(--cartoon-primary);
    font-family: 'Comic Sans MS', cursive, sans-serif;
    margin-right: 5px;
  }
  
  .logo-icon {
    font-size: 1.5rem;
  }
}

.nav-menu {
  display: flex;
  align-items: center;
  margin-left: 30px;
  
  .nav-item {
    padding: 8px 15px;
    margin: 0 5px;
    text-decoration: none;
    color: var(--cartoon-text-dark);
    font-weight: 600;
    border-radius: 20px;
    transition: all 0.3s ease;
    position: relative;
    
    &:hover, &.router-link-active {
      color: var(--cartoon-primary);
      background-color: var(--cartoon-bg-light);
    }
    
    &.router-link-active::after {
      content: '';
      position: absolute;
      bottom: -3px;
      left: 50%;
      transform: translateX(-50%);
      width: 30px;
      height: 3px;
      background-color: var(--cartoon-primary);
      border-radius: 3px;
    }
  }
}

.user-section {
  display: flex;
  align-items: center;
}

.auth-buttons {
  .login-btn {
    background-color: var(--cartoon-primary);
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 20px;
    font-weight: 600;
    text-decoration: none;
    transition: all 0.3s ease;
    
    &:hover {
      background-color: var(--cartoon-primary-dark);
      transform: translateY(-2px);
      box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    }
  }
}

.user-menu {
  cursor: pointer;
}

.user-dropdown {
  display: flex;
  align-items: center;
  padding: 5px 10px;
  border-radius: 20px;
  transition: all 0.3s ease;
  background-color: var(--cartoon-bg-light);
  
  &:hover {
    background-color: var(--cartoon-bg-medium);
  }
  
  .user-nickname {
    margin-right: 10px;
    font-weight: 600;
    color: var(--cartoon-text-dark);
  }
  
  .user-avatar {
    background-color: var(--cartoon-primary);
    color: white;
    border: 2px solid white;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
  }
}

.cartoon-dropdown {
  border-radius: 16px;
  overflow: hidden;
  border: 2px solid var(--cartoon-primary);
  padding: 8px;
  
  :deep(.el-dropdown-menu__item) {
    border-radius: 10px;
    margin: 3px 0;
    display: flex;
    align-items: center;
    
    &:hover {
      background-color: var(--cartoon-bg-light);
      color: var(--cartoon-primary);
    }
    
    .menu-icon {
      margin-right: 8px;
      font-style: normal;
    }
  }
}

.mobile-menu-toggle {
  font-size: 24px;
  cursor: pointer;
  color: var(--cartoon-text-dark);
  
  .menu-icon {
    font-style: normal;
  }
}

.mobile-menu {
  position: absolute;
  top: var(--header-height);
  left: 0;
  width: 100%;
  background-color: white;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  border-bottom: 3px solid var(--cartoon-primary);
  
  .mobile-nav-item {
    padding: 15px 20px;
    text-decoration: none;
    color: var(--cartoon-text-dark);
    font-weight: 600;
    border-bottom: 2px dashed var(--cartoon-border-light);
    transition: all 0.3s ease;
    
    &:last-child {
      border-bottom: none;
    }
    
    &:hover, &.router-link-active {
      background-color: var(--cartoon-bg-light);
      color: var(--cartoon-primary);
    }
  }
}

.content-container {
  flex: 1;
  margin-top: var(--header-height);
  padding-bottom: 50px;
}

.app-footer {
  background-color: white;
  border-top: 3px solid var(--cartoon-primary);
  padding: 40px 0;
  margin-top: auto;
}

.footer-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.footer-logo {
  display: flex;
  align-items: center;
  margin-bottom: 20px;
  
  .logo-text {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--cartoon-primary);
    font-family: 'Comic Sans MS', cursive, sans-serif;
    margin-right: 5px;
  }
  
  .logo-icon {
    font-size: 1.3rem;
  }
}

.footer-links {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  margin-bottom: 20px;
  
  .footer-link {
    margin: 0 15px;
    color: var(--cartoon-text-dark);
    text-decoration: none;
    transition: all 0.3s ease;
    
    &:hover {
      color: var(--cartoon-primary);
      text-decoration: underline;
    }
  }
}

.copyright {
  color: var(--cartoon-text-light);
  font-size: 0.9rem;
}

.cartoon-backtop {
  :deep(.el-backtop) {
    background-color: var(--cartoon-primary);
    color: white;
    border-radius: 50%;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    transition: all 0.3s ease;
    
    &:hover {
      transform: translateY(-3px);
      box-shadow: 0 6px 16px rgba(0, 0, 0, 0.25);
    }
  }
}

.hide-sm {
  @media (max-width: 576px) {
    display: none !important;
  }
}

.show-sm {
  display: none !important;
  @media (max-width: 576px) {
    display: block !important;
  }
}

.menu-icon {
  font-style: normal;
}
</style>