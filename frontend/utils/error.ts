import { ElMessage } from 'element-plus'

export function showErrorMessage(error: unknown, fallbackMsg = '操作失败') {
  const msg =
    error instanceof Error
      ? error.message
      : typeof error === 'string'
        ? error
        : fallbackMsg
  if (error) console.error(msg, error)
  ElMessage.error(msg)
}
