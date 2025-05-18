<template>
  <el-dialog :modelValue="visible" @update:modelValue="$emit('update:visible', $event)" :title="title" width="600px"
    custom-class="message-dialog zelda-container" :before-close="handleClose">
    <!-- 对话框内容 -->
    <div class="message-container">
      <!-- 消息类型标签 -->
      <div class="message-tabs zelda-corner-decorations">
        <div v-for="tab in tabs" :key="tab.key" class="message-tab" :class="{ active: activeTab === tab.key }"
          @click="setActiveTab(tab.key)">
          {{ tab.name }}
          <span v-if="getTabCount(tab.key) > 0" class="message-badge">{{ getTabCount(tab.key) }}</span>
        </div>
      </div>

      <!-- 聊天列表模式 -->
      <div v-if="activeTab === 'chat' && !currentConversation" class="conversations-list">
        <div v-if="messageStore.isLoading && !messageStore.conversations.length" class="loading-container">
          <el-skeleton :rows="3" animated />
          <el-skeleton :rows="3" animated />
        </div>

        <div v-else-if="messageStore.conversations.length === 0" class="no-messages">
          <img src="/images/defaults/empty-messages.png" alt="没有消息" class="empty-image">
          <p>暂无聊天消息</p>
        </div>

        <div v-else v-for="conversation in messageStore.conversations" :key="conversation.conversation_id"
          class="conversation-item" :class="{ unread: conversation.unread_count > 0 }"
          @click="openConversation(conversation.conversation_id, conversation.peer_id)">
          <div class="avatar">
            <img src="/images/defaults/user-avatar.png" alt="用户头像">
            <div v-if="conversation.unread_count > 0" class="unread-dot"></div>
          </div>

          <div class="conversation-content">
            <div class="conversation-header">
              <span class="peer-name">{{ conversation.peer_name }}</span>
              <span class="time">{{ formatTime(conversation.last_message_time) }}</span>
            </div>
            <p class="last-message">{{ conversation.last_message }}</p>
          </div>
        </div>
      </div>

      <!-- 聊天详情模式 -->
      <div v-else-if="activeTab === 'chat' && currentConversation" class="chat-detail zelda-corner-decorations">
        <!-- 返回按钮 -->
        <div class="chat-header">
          <el-button type="text" @click="closeConversation" class="back-button">
            <i class="cartoon-icon">←</i> 返回
          </el-button>
          <h3 class="chat-title">{{ currentPeerName }}</h3>
          <el-button type="text" @click="messageStore.markConversationRead(currentConversation)"
            :disabled="!hasUnreadMessages" class="mark-read-button">
            标记已读
          </el-button>
        </div>

        <!-- 消息列表 -->
        <div class="messages-list">
          <div v-if="messageStore.isLoadingMessages && !messageStore.messages.length" class="loading-container">
            <el-skeleton :rows="3" animated />
            <el-skeleton :rows="3" animated />
          </div>

          <div v-else-if="messageStore.messages.length === 0" class="no-messages">
            <p>暂无消息记录，发送一条消息开始聊天吧</p>
          </div>

          <template v-else>
            <div v-if="messageStore.hasMoreMessages" class="load-more">
              <el-button type="text" @click="loadMoreMessages" :loading="messageStore.isLoadingMessages">
                加载更多
              </el-button>
            </div>

            <div v-for="message in messageStore.messages" :key="message.message_id" class="message-bubble" :class="{
              sent: message.sender_id === userId,
              received: message.sender_id !== userId,
              unread: !message.read_at && message.sender_id !== userId
            }">
              <div class="message-content">
                <p>{{ message.content }}</p>
                <span class="message-time">{{ formatTime(message.sent_at) }}</span>
              </div>
            </div>
          </template>
        </div>

        <!-- 消息输入区域 -->
        <div class="message-input-area">
          <el-input :model-value="newMessage" @update:model-value="newMessage = $event" type="textarea" :rows="2"
            placeholder="输入消息..." resize="none" @keydown.enter.exact.prevent="handleSendMessage" />
          <el-button type="primary" @click="sendMessage" :loading="messageStore.isSending"
            :disabled="!newMessage.trim()" class="send-button">
            发送
          </el-button>
        </div>
      </div>

      <!-- 系统通知 -->
      <div v-else-if="activeTab === 'system'" class="notifications-list">
        <div v-if="messageStore.isLoading && !messageStore.systemNotifications.length" class="loading-container">
          <el-skeleton :rows="3" animated />
          <el-skeleton :rows="3" animated />
        </div>

        <div v-else-if="messageStore.systemNotifications.length === 0" class="no-messages">
          <img src="/images/defaults/empty-notification.png" alt="没有通知" class="empty-image">
          <p>暂无系统通知</p>
        </div>

        <div v-else v-for="notification in messageStore.systemNotifications" :key="notification.message_id"
          class="notification-item" :class="{ unread: !notification.read_at }">
          <div class="notification-avatar">
            <img src="/images/defaults/system-avatar.png" alt="系统通知">
          </div>

          <div class="notification-content">
            <div class="notification-header">
              <span class="notification-title">系统通知</span>
              <span class="time">{{ formatTime(notification.sent_at) }}</span>
            </div>
            <p class="notification-message">{{ notification.content }}</p>
          </div>
        </div>
      </div>

      <!-- 签到互动 -->
      <div v-else-if="activeTab === 'checkin'" class="checkin-notifications">
        <div class="no-messages">
          <img src="/images/defaults/empty-checkin.png" alt="没有签到互动" class="empty-image">
          <p>暂无签到互动消息</p>
        </div>
      </div>
    </div>
  </el-dialog>
</template>

<script setup>
import { ref, computed, watch, nextTick } from 'vue'
import { useMessageStore } from '~/stores/message'
import { useAuthStore } from '~/stores/auth'
import { useUserStore } from '~/stores/user'
import { ElMessage } from 'element-plus'

const props = defineProps({
  visible: {
    type: Boolean,
    default: false
  }
})

const emit = defineEmits(['update:visible', 'close'])

const messageStore = useMessageStore()
const authStore = useAuthStore()
const userStore = useUserStore()

// 消息标签页
const tabs = [
  { key: 'chat', name: '私信' },
  { key: 'system', name: '系统通知' },
  { key: 'checkin', name: '签到互动' }
]

const activeTab = ref('chat')
const currentConversation = ref(null)
const currentPeerId = ref(null)
const currentPeerName = ref('')
const newMessage = ref('')
const messagesContainer = ref(null)

// 当前用户ID
const userId = computed(() => authStore.userData?.user_id)

// 对话框标题
const title = computed(() => {
  if (activeTab.value === 'chat') {
    return currentConversation.value ? currentPeerName.value : '我的消息';
  } else if (activeTab.value === 'system') {
    return '系统通知';
  } else if (activeTab.value === 'checkin') {
    return '签到互动';
  }
  return '消息中心';
})

// 是否有未读消息
const hasUnreadMessages = computed(() => {
  return messageStore.messages.some(msg => !msg.read_at && msg.sender_id !== userId.value);
})

// 初始化数据
async function initData() {
  if (!messageStore.conversations.length) {
    await messageStore.loadInitialData()
  }
}

// 设置活动标签
function setActiveTab(tab) {
  activeTab.value = tab
  currentConversation.value = null
}

// 获取标签未读数
function getTabCount(tab) {
  if (tab === 'chat') {
    return messageStore.unreadCounts.direct || 0
  } else if (tab === 'system') {
    return messageStore.unreadCounts.system || 0
  } else if (tab === 'checkin') {
    return messageStore.unreadCounts.checkin + messageStore.unreadCounts.like + messageStore.unreadCounts.comment || 0
  }
  return 0
}

// 打开会话
async function openConversation(conversationId, peerId) {
  currentConversation.value = conversationId
  currentPeerId.value = peerId

  // 获取对方用户名
  try {
    const conv = messageStore.conversations.find(c => c.conversation_id === conversationId)
    if (conv) {
      currentPeerName.value = conv.peer_name
    } else {
      const user = await userStore.getUserById(peerId)
      currentPeerName.value = user.nickname
    }
  } catch (err) {
    currentPeerName.value = `用户${peerId}`
  }

  // 加载消息历史
  await messageStore.fetchMessageHistory(conversationId, undefined, true)

  // 滚动到底部
  scrollToBottom()
}

// 关闭会话
function closeConversation() {
  currentConversation.value = null
  currentPeerId.value = null
  currentPeerName.value = ''
  newMessage.value = ''
}

// 发送消息
async function sendMessage() {
  if (!newMessage.value.trim() || !currentPeerId.value) return

  // 检查用户位置
  if (!userStore.hasLocation) {
    ElMessage.warning('需要位置信息才能发送消息，请先获取位置')
    return
  }

  try {
    await messageStore.sendDirectMessage(
      currentPeerId.value,
      newMessage.value,
      {
        latitude: userStore.currentLocation.latitude,
        longitude: userStore.currentLocation.longitude
      }
    )

    newMessage.value = ''
    scrollToBottom()
  } catch (err) {
    const errorMsg = err instanceof Error ? err.message : String(err)
    ElMessage.error('发送消息失败: ' + errorMsg)
  }
}

// 加载更多消息
async function loadMoreMessages() {
  if (!currentConversation.value) return

  await messageStore.fetchMessageHistory(currentConversation.value)
}

// 格式化时间
function formatTime(timeStr) {
  if (!timeStr) return ''

  const date = new Date(timeStr)
  const now = new Date()
  const diff = now - date

  // 今天
  if (date.toDateString() === now.toDateString()) {
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
  }
  // 昨天
  else if (
    now.getDate() - date.getDate() === 1 &&
    now.getMonth() === date.getMonth() &&
    now.getFullYear() === date.getFullYear()
  ) {
    return '昨天 ' + date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
  }
  // 今年
  else if (now.getFullYear() === date.getFullYear()) {
    return `${date.getMonth() + 1}月${date.getDate()}日 ${date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}`
  }
  // 往年
  else {
    return `${date.getFullYear()}年${date.getMonth() + 1}月${date.getDate()}日`
  }
}

// 关闭对话框
function handleClose() {
  emit('update:visible', false)
  emit('close')

  // 重置状态
  closeConversation()
  activeTab.value = 'chat'
}

// 滚动到底部
function scrollToBottom() {
  nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
    }
  })
}

// 监听对话框打开
watch(() => props.visible, (val) => {
  if (val) {
    initData()
  }
})

// 监听消息变化自动滚动
watch(() => messageStore.messages, () => {
  scrollToBottom()
}, { deep: true })
</script>

<style scoped>
.message-container {
  display: flex;
  flex-direction: column;
  height: 500px;
}

/* 标签样式 */
.message-tabs {
  display: flex;
  background-color: var(--zelda-bg-medium);
  border-radius: 6px;
  margin-bottom: 16px;
  overflow: hidden;
  border: 1px solid var(--zelda-border-medium);
}

.message-tab {
  padding: 10px 20px;
  flex: 1;
  text-align: center;
  cursor: pointer;
  position: relative;
  transition: all 0.3s ease;
  font-weight: 500;
  color: var(--zelda-text-medium);
  border-right: 1px solid var(--zelda-border-light);
}

.message-tab:last-child {
  border-right: none;
}

.message-tab.active {
  background-color: var(--zelda-primary);
  color: white;
}

.message-tab:hover:not(.active) {
  background-color: var(--zelda-bg-dark);
}

.message-badge {
  position: absolute;
  top: 4px;
  right: 4px;
  min-width: 18px;
  height: 18px;
  line-height: 18px;
  padding: 0 6px;
  text-align: center;
  background-color: var(--zelda-secondary);
  color: var(--zelda-text-dark);
  border-radius: 4px;
  font-size: 12px;
  font-weight: bold;
}

/* 会话列表 */
.conversations-list {
  flex: 1;
  overflow-y: auto;
  background-color: white;
  border-radius: 6px;
  border: 1px solid var(--zelda-border-medium);
}

.conversation-item {
  display: flex;
  padding: 12px 16px;
  cursor: pointer;
  border-bottom: 1px solid var(--zelda-border-light);
  transition: all 0.2s ease;
  position: relative;
}

.conversation-item::before {
  content: '›';
  position: absolute;
  left: 5px;
  top: 50%;
  transform: translateY(-50%);
  opacity: 0;
  color: var(--zelda-primary);
  transition: all 0.2s ease;
}

.conversation-item:hover {
  background-color: var(--zelda-bg-light);
  padding-left: 25px;
}

.conversation-item:hover::before {
  opacity: 1;
}

.conversation-item.unread {
  background-color: var(--zelda-bg-medium);
}

.avatar {
  width: 46px;
  height: 46px;
  border-radius: 50%;
  overflow: hidden;
  margin-right: 12px;
  position: relative;
  border: 2px solid var(--zelda-border-medium);
  background-color: white;
}

.avatar img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.unread-dot {
  position: absolute;
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background-color: var(--zelda-secondary);
  right: 0;
  bottom: 0;
  border: 2px solid white;
}

.conversation-content {
  flex: 1;
  overflow: hidden;
}

.conversation-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 6px;
}

.peer-name {
  font-weight: 600;
  color: var(--zelda-text-dark);
}

.time {
  font-size: 12px;
  color: var(--zelda-text-light);
}

.last-message {
  margin: 0;
  color: var(--zelda-text-medium);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* 聊天详情 */
.chat-detail {
  display: flex;
  flex-direction: column;
  flex: 1;
  background-color: var(--zelda-bg-light);
  border-radius: 6px;
  padding: 15px;
  border: 1px solid var(--zelda-border-medium);
}

.chat-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 0 10px 0;
  border-bottom: 1px solid var(--zelda-border-light);
  margin-bottom: 10px;
}

.chat-title {
  margin: 0;
  font-size: 16px;
  flex: 1;
  text-align: center;
  color: var(--zelda-primary-dark);
}

.back-button,
.mark-read-button {
  font-size: 14px;
  color: var(--zelda-primary);
}

.messages-list {
  flex: 1;
  overflow-y: auto;
  padding: 10px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.message-bubble {
  max-width: 80%;
  padding: 8px 12px;
  border-radius: 6px;
  position: relative;
}

.message-bubble.sent {
  align-self: flex-end;
  background-color: var(--zelda-primary);
  color: white;
  border-bottom-right-radius: 0;
}

.message-bubble.received {
  align-self: flex-start;
  background-color: white;
  color: var(--zelda-text-dark);
  border-bottom-left-radius: 0;
  border: 1px solid var(--zelda-border-medium);
}

.message-bubble.unread {
  background-color: var(--zelda-bg-light);
  border: 1px solid var(--zelda-primary-light);
}

.message-content {
  display: flex;
  flex-direction: column;
}

.message-content p {
  margin: 0 0 5px 0;
}

.message-time {
  font-size: 10px;
  opacity: 0.8;
  align-self: flex-end;
}

.message-input-area {
  display: flex;
  gap: 10px;
  margin-top: 10px;
  padding-top: 10px;
  border-top: 1px solid var(--zelda-border-light);
}

.send-button {
  align-self: flex-end;
}

/* 系统通知 */
.notifications-list {
  flex: 1;
  overflow-y: auto;
  background-color: white;
  border-radius: 6px;
  border: 1px solid var(--zelda-border-medium);
}

.notification-item {
  display: flex;
  padding: 12px 16px;
  border-bottom: 1px solid var(--zelda-border-light);
  position: relative;
}

.notification-item.unread {
  background-color: var(--zelda-bg-medium);
}

.notification-item::before {
  content: '!';
  position: absolute;
  left: 3px;
  top: 50%;
  transform: translateY(-50%);
  font-weight: bold;
  color: var(--zelda-secondary);
  opacity: 0;
  transition: all 0.2s ease;
}

.notification-item:hover::before {
  opacity: 1;
}

.notification-avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  overflow: hidden;
  margin-right: 12px;
  background-color: var(--zelda-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid var(--zelda-border-medium);
}

.notification-avatar img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.notification-content {
  flex: 1;
}

.notification-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 6px;
}

.notification-title {
  font-weight: 600;
  color: var(--zelda-text-dark);
}

.notification-message {
  margin: 0;
  color: var(--zelda-text-medium);
}

/* 加载中状态 */
.loading-container {
  padding: 15px;
}

/* 无消息状态 */
.no-messages {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--zelda-text-light);
}

.empty-image {
  width: 100px;
  height: 100px;
  margin-bottom: 15px;
  opacity: 0.6;
}

/* 加载更多按钮 */
.load-more {
  text-align: center;
  margin: 10px 0;
}

/* 自定义对话框样式 */
:deep(.message-dialog) {
  border-radius: 8px;
  overflow: hidden;
}

:deep(.message-dialog .el-dialog__header) {
  padding: 15px 20px;
  position: relative;
}

:deep(.message-dialog .el-dialog__header::after) {
  content: '';
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  height: 2px;
  background: var(--zelda-secondary);
}

:deep(.message-dialog .el-dialog__body) {
  padding: 20px;
  background-color: var(--zelda-bg-light);
}

:deep(.message-dialog .el-textarea__inner) {
  border-radius: 6px;
  resize: none;
  background-color: rgba(255, 255, 255, 0.8);
  border: 1px solid var(--zelda-border-medium);
}

:deep(.message-dialog .el-textarea__inner:focus) {
  border-color: var(--zelda-primary);
  box-shadow: 0 0 0 2px rgba(63, 173, 86, 0.2);
}
</style>