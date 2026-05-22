<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">File Library</span>
        <h2 class="page-hero-title">文件管理</h2>
        <p class="page-hero-subtitle">维护核心后台文件列表和分类结构，支持 URL 导入、重命名与删除。</p>

        <div class="page-metrics">
          <div class="page-metric">
            <div class="page-metric-label">当前页文件</div>
            <div class="page-metric-value">{{ files.list.length }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">总文件数</div>
            <div class="page-metric-value">{{ files.total }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">分类数</div>
            <div class="page-metric-value">{{ flattenedCategories.length }}</div>
          </div>
        </div>
      </div>

      <aside class="page-hero-side">
        <div>
          <div class="page-note-label">模块摘要</div>
          <div class="page-note-value">{{ summary }}</div>
        </div>
        <div v-if="selectedUpload.name" class="upload-preview">
          <img v-if="selectedUpload.previewUrl" :src="selectedUpload.previewUrl" alt="preview" class="upload-preview-image" />
          <div v-else class="upload-preview-fallback">{{ selectedUpload.extension || 'FILE' }}</div>
          <div class="upload-preview-body">
            <div class="upload-preview-name">{{ selectedUpload.name }}</div>
            <div class="upload-preview-meta">
              {{ selectedUpload.sizeLabel }} · {{ selectedUpload.classLabel }}
            </div>
            <div class="upload-progress-track">
              <div class="upload-progress-bar" :style="{ width: `${uploadProgress}%` }" />
            </div>
            <div class="upload-preview-status">
              {{ uploadStatusLabel }}
            </div>
          </div>
        </div>
        <div class="page-hero-actions">
          <el-button @click="loadData" :loading="loading">刷新文件</el-button>
          <el-button @click="triggerUpload">上传文件</el-button>
          <el-button type="primary" @click="openImportDialog">导入 URL</el-button>
        </div>
      </aside>
    </section>

    <section class="file-grid">
      <article class="page-panel">
        <div class="page-panel-header">
          <div>
            <h3 class="page-panel-title">分类结构</h3>
            <p class="page-panel-subtitle">选中分类后，右侧文件列表按分类过滤。</p>
          </div>
        </div>

        <div class="page-panel-toolbar inline-filter">
          <el-button @click="openCategoryDialog()">新增分类</el-button>
        </div>

        <div class="surface-card category-card">
          <div
            v-for="item in flattenedCategories"
            :key="item.ID"
            class="category-row"
            :class="{ 'is-active': selectedClassId === item.ID }"
            @click="selectCategory(item.ID)"
          >
            <div class="category-name" :style="{ paddingLeft: `${item._depth * 14 + 12}px` }">
              {{ item.name }}
            </div>
            <div class="category-actions">
              <el-button link type="primary" @click.stop="openCategoryDialog(item)">编辑</el-button>
              <el-button link type="danger" @click.stop="handleDeleteCategory(item.ID)">删除</el-button>
            </div>
          </div>
        </div>
      </article>

      <article class="page-panel">
        <div class="page-panel-header">
          <div>
            <h3 class="page-panel-title">文件列表</h3>
            <p class="page-panel-subtitle">文件管理先覆盖列表、分类、URL 导入和重命名。</p>
          </div>
        </div>

        <div class="page-panel-toolbar inline-filter">
          <el-input v-model="filters.keyword" placeholder="按名称或 URL 过滤" clearable />
          <el-button type="primary" @click="handleSearch">查询</el-button>
        </div>

        <div class="surface-card">
          <el-table :data="files.list" v-loading="loading" style="width: 100%">
            <el-table-column prop="ID" label="ID" width="80" />
            <el-table-column label="名称" min-width="160">
              <template #default="{ row }">
                <el-button link type="primary" @click="openPreviewDialog(row)">{{ row.name }}</el-button>
              </template>
            </el-table-column>
            <el-table-column prop="url" label="URL" min-width="220" />
            <el-table-column prop="tag" label="类型" width="100" />
            <el-table-column prop="classId" label="分类" width="90" />
            <el-table-column prop="UpdatedAt" label="更新时间" min-width="180" />
            <el-table-column label="操作" width="240">
              <template #default="{ row }">
                <el-button link @click="openPreviewDialog(row)">预览</el-button>
                <el-button link type="primary" @click="openRenameDialog(row)">重命名</el-button>
                <el-button link type="danger" @click="handleDeleteFile(row.ID)">删除</el-button>
              </template>
            </el-table-column>
          </el-table>
        </div>

        <div class="pagination">
          <el-pagination
            background
            layout="total, prev, pager, next"
            :total="files.total"
            :current-page="page"
            :page-size="pageSize"
            @current-change="handlePageChange"
          />
        </div>
      </article>
    </section>

    <el-dialog v-model="categoryDialogVisible" :title="categoryForm.ID ? '编辑分类' : '新增分类'" width="520px">
      <el-form label-width="90px" @submit.prevent="submitCategory">
        <el-form-item label="名称">
          <el-input v-model="categoryForm.name" placeholder="例如：图片" />
        </el-form-item>
        <el-form-item label="父分类">
          <el-select v-model="categoryForm.pid" class="w-full">
            <el-option :value="0" label="顶级分类" />
            <el-option
              v-for="item in flattenedCategories"
              :key="item.ID"
              :label="item.name"
              :value="item.ID"
            />
          </el-select>
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="categoryDialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="submittingCategory" @click="submitCategory">保存</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="importDialogVisible" title="导入 URL" width="560px">
      <el-form label-width="90px" @submit.prevent="submitImport">
        <el-form-item label="名称">
          <el-input v-model="importForm.name" placeholder="例如：logo.png" />
        </el-form-item>
        <el-form-item label="URL">
          <el-input v-model="importForm.url" placeholder="https://example.com/logo.png" />
        </el-form-item>
        <el-form-item label="分类">
          <el-select v-model="importForm.classId" class="w-full" clearable>
            <el-option :value="0" label="未分类" />
            <el-option
              v-for="item in flattenedCategories"
              :key="item.ID"
              :label="item.name"
              :value="item.ID"
            />
          </el-select>
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="importDialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="submittingImport" @click="submitImport">导入</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="renameDialogVisible" title="重命名文件" width="520px">
      <el-form label-width="90px" @submit.prevent="submitRename">
        <el-form-item label="名称">
          <el-input v-model="renameForm.name" placeholder="新的文件名" />
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="renameDialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="submittingRename" @click="submitRename">保存</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="previewDialogVisible" title="文件详情" width="640px">
      <div v-if="previewFile" class="preview-layout">
        <div class="preview-visual">
          <img
            v-if="isPreviewImage(previewFile)"
            :src="resolveFileUrl(previewFile.url)"
            :alt="previewFile.name"
            class="preview-image"
          />
          <div v-else class="preview-fallback">{{ previewFile.tag?.toUpperCase() || 'FILE' }}</div>
        </div>
        <div class="preview-meta">
          <div class="preview-title">{{ previewFile.name }}</div>
          <div class="preview-item">
            <span>文件地址</span>
            <strong>{{ previewFile.url }}</strong>
          </div>
          <div class="preview-item">
            <span>文件类型</span>
            <strong>{{ previewFile.tag || '-' }}</strong>
          </div>
          <div class="preview-item">
            <span>所属分类</span>
            <strong>{{ categoryName(previewFile.classId) }}</strong>
          </div>
          <div class="preview-item">
            <span>更新时间</span>
            <strong>{{ previewFile.UpdatedAt }}</strong>
          </div>
        </div>
      </div>

      <template #footer>
        <el-button @click="previewDialogVisible = false">关闭</el-button>
        <el-button type="primary" @click="openFileInNewTab">打开文件</el-button>
      </template>
    </el-dialog>

    <input
      ref="fileInput"
      type="file"
      class="hidden-input"
      @change="handleFileSelected"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, reactive, ref } from 'vue'
import { ElMessage, ElMessageBox } from '@/ui/feedback'

import { getApiErrorMessage } from '@/api/http'

import {
  deleteCategory,
  deleteFile,
  fetchCategories,
  fetchFiles,
  importFileUrl,
  renameFile,
  saveCategory,
  uploadFile,
  type CategoryRecord,
  type FileRecord,
  type FileListResult
} from '@/api/files'

const loading = ref(false)
const uploading = ref(false)
const uploadProgress = ref(0)
const page = ref(1)
const pageSize = ref(10)
const selectedClassId = ref<number | undefined>()
const fileInput = ref<HTMLInputElement | null>(null)
const selectedUpload = reactive({
  name: '',
  sizeLabel: '',
  previewUrl: '',
  extension: '',
  classLabel: '未分类'
})
const files = reactive<FileListResult>({
  list: [],
  total: 0,
  page: 1,
  pageSize: 10
})
const categories = ref<CategoryRecord[]>([])
const filters = reactive({
  keyword: ''
})
const categoryDialogVisible = ref(false)
const submittingCategory = ref(false)
const categoryForm = reactive({
  ID: 0,
  name: '',
  pid: 0
})
const importDialogVisible = ref(false)
const submittingImport = ref(false)
const importForm = reactive({
  name: '',
  url: '',
  classId: 0
})
const renameDialogVisible = ref(false)
const submittingRename = ref(false)
const previewDialogVisible = ref(false)
const previewFile = ref<FileRecord | null>(null)
const renameForm = reactive({
  ID: 0,
  name: ''
})

const flattenedCategories = computed(() => flattenCategories(categories.value))
const summary = computed(() => `当前共 ${files.total} 个文件，已建立 ${flattenedCategories.value.length} 个分类节点`)
const uploadStatusLabel = computed(() => {
  if (!selectedUpload.name) return ''
  if (uploading.value) return `上传中 ${uploadProgress.value}%`
  if (uploadProgress.value === 100) return '上传完成'
  return '等待上传'
})

function flattenCategories(list: CategoryRecord[], depth = 0): Array<CategoryRecord & { _depth: number }> {
  return list.flatMap((item) => [
    { ...item, _depth: depth },
    ...flattenCategories(item.children || [], depth + 1)
  ])
}

function resetCategoryForm() {
  categoryForm.ID = 0
  categoryForm.name = ''
  categoryForm.pid = 0
}

function resolveFileUrl(url: string) {
  if (!url) return ''
  if (url.startsWith('http://') || url.startsWith('https://')) return url
  const baseUrl = import.meta.env.VITE_API_BASE_URL || 'http://127.0.0.1:3000/api'
  const origin = baseUrl.replace(/\/api$/, '')
  return `${origin}${url}`
}

function categoryName(classId: number) {
  return flattenedCategories.value.find((item) => item.ID === classId)?.name || '未分类'
}

function isPreviewImage(file: FileRecord) {
  const tag = file.tag?.toLowerCase()
  return ['png', 'jpg', 'jpeg', 'gif', 'webp', 'bmp', 'svg'].includes(tag)
}

function clearUploadPreview() {
  if (selectedUpload.previewUrl) {
    URL.revokeObjectURL(selectedUpload.previewUrl)
  }
  selectedUpload.name = ''
  selectedUpload.sizeLabel = ''
  selectedUpload.previewUrl = ''
  selectedUpload.extension = ''
  selectedUpload.classLabel = '未分类'
  uploadProgress.value = 0
}

async function loadFiles() {
  const result = await fetchFiles({
    page: page.value,
    pageSize: pageSize.value,
    keyword: filters.keyword,
    classId: selectedClassId.value
  })
  Object.assign(files, result)
}

async function loadCategories() {
  categories.value = await fetchCategories()
}

async function loadData() {
  loading.value = true
  try {
    await Promise.all([loadFiles(), loadCategories()])
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, '获取文件数据失败'))
  } finally {
    loading.value = false
  }
}

function selectCategory(id: number) {
  selectedClassId.value = selectedClassId.value === id ? undefined : id
  page.value = 1
  loadData()
}

function handleSearch() {
  page.value = 1
  loadData()
}

function handlePageChange(nextPage: number) {
  page.value = nextPage
  loadData()
}

function openCategoryDialog(item?: CategoryRecord) {
  if (item) {
    categoryForm.ID = item.ID
    categoryForm.name = item.name
    categoryForm.pid = item.pid
  } else {
    resetCategoryForm()
  }
  categoryDialogVisible.value = true
}

async function submitCategory() {
  if (!categoryForm.name.trim()) {
    ElMessage.warning('请填写分类名称')
    return
  }

  submittingCategory.value = true
  try {
    const response = await saveCategory({
      ID: categoryForm.ID || 0,
      name: categoryForm.name.trim(),
      pid: categoryForm.pid
    })
    if (response.code === 'OK') {
      ElMessage.success('分类已保存')
      categoryDialogVisible.value = false
      await loadData()
      return
    }
    ElMessage.error(response.message || '保存分类失败')
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, '保存分类失败'))
  } finally {
    submittingCategory.value = false
  }
}

async function handleDeleteCategory(id: number) {
  try {
    await ElMessageBox.confirm('确定删除该分类及其文件吗？', '提示', { type: 'warning' })
  } catch {
    return
  }

  try {
    const response = await deleteCategory(id)
    if (response.code === 'OK') {
      ElMessage.success('分类已删除')
      if (selectedClassId.value === id) {
        selectedClassId.value = undefined
      }
      await loadData()
      return
    }
    ElMessage.error(response.message || '删除分类失败')
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, '删除分类失败'))
  }
}

function openImportDialog() {
  importForm.name = ''
  importForm.url = ''
  importForm.classId = selectedClassId.value || 0
  importDialogVisible.value = true
}

async function submitImport() {
  if (!importForm.name.trim() || !importForm.url.trim()) {
    ElMessage.warning('请填写完整导入信息')
    return
  }

  submittingImport.value = true
  try {
    const response = await importFileUrl({
      name: importForm.name.trim(),
      url: importForm.url.trim(),
      classId: importForm.classId || undefined
    })
    if (response.code === 'OK') {
      ElMessage.success('文件已导入')
      importDialogVisible.value = false
      await loadData()
      return
    }
    ElMessage.error(response.message || '导入失败')
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, '导入失败'))
  } finally {
    submittingImport.value = false
  }
}

function openRenameDialog(item: FileRecord) {
  renameForm.ID = item.ID
  renameForm.name = item.name
  renameDialogVisible.value = true
}

function openPreviewDialog(item: FileRecord) {
  previewFile.value = item
  previewDialogVisible.value = true
}

function openFileInNewTab() {
  if (!previewFile.value) return
  window.open(resolveFileUrl(previewFile.value.url), '_blank', 'noopener,noreferrer')
}

async function submitRename() {
  if (!renameForm.name.trim()) {
    ElMessage.warning('请填写文件名称')
    return
  }

  submittingRename.value = true
  try {
    const response = await renameFile({
      ID: renameForm.ID,
      name: renameForm.name.trim()
    })
    if (response.code === 'OK') {
      ElMessage.success('文件已重命名')
      renameDialogVisible.value = false
      await loadData()
      return
    }
    ElMessage.error(response.message || '重命名失败')
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, '重命名失败'))
  } finally {
    submittingRename.value = false
  }
}

async function handleDeleteFile(id: number) {
  try {
    await ElMessageBox.confirm('确定删除该文件吗？', '提示', { type: 'warning' })
  } catch {
    return
  }

  try {
    const response = await deleteFile(id)
    if (response.code === 'OK') {
      ElMessage.success('文件已删除')
      await loadData()
      return
    }
    ElMessage.error(response.message || '删除文件失败')
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, '删除文件失败'))
  }
}

function triggerUpload() {
  fileInput.value?.click()
}

async function handleFileSelected(event: Event) {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return

  clearUploadPreview()
  selectedUpload.name = file.name
  selectedUpload.sizeLabel = `${(file.size / 1024 / 1024).toFixed(2)} MB`
  selectedUpload.extension = file.name.split('.').pop()?.toUpperCase() || ''
  selectedUpload.classLabel =
    flattenedCategories.value.find((item) => item.ID === selectedClassId.value)?.name || '未分类'
  if (file.type.startsWith('image/')) {
    selectedUpload.previewUrl = URL.createObjectURL(file)
  }

  loading.value = true
  uploading.value = true
  uploadProgress.value = 0
  try {
    const response = await uploadFile(file, selectedClassId.value, (progress) => {
      uploadProgress.value = progress
    })
    if (response.code === 'OK') {
      uploadProgress.value = 100
      ElMessage.success('文件上传成功')
      await loadData()
    } else {
      ElMessage.error(response.message || '上传失败')
    }
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, '上传失败'))
  } finally {
    input.value = ''
    loading.value = false
    uploading.value = false
  }
}

onMounted(() => {
  loadData()
})

onBeforeUnmount(() => {
  clearUploadPreview()
})
</script>

<style scoped>
.file-grid {
  display: grid;
  grid-template-columns: minmax(280px, 0.8fr) minmax(0, 1.2fr);
  gap: 18px;
}

.category-card {
  padding: 8px 0;
}

.category-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 10px 12px;
  cursor: pointer;
}

.category-row.is-active {
  background: #f5f5f4;
}

.category-name {
  color: var(--text-primary);
}

.category-actions {
  display: flex;
  gap: 6px;
}

.pagination {
  display: flex;
  justify-content: flex-end;
  margin-top: 16px;
}

.hidden-input {
  display: none;
}

.upload-preview {
  display: flex;
  gap: 14px;
  padding: 14px;
  margin-top: 14px;
  border-radius: 14px;
  border: 1px solid var(--panel-border);
  background: #fafaf9;
}

.preview-layout {
  display: grid;
  grid-template-columns: 220px minmax(0, 1fr);
  gap: 18px;
}

.preview-visual {
  border-radius: 18px;
  border: 1px solid var(--panel-border);
  background: #fafaf9;
  min-height: 220px;
  display: grid;
  place-items: center;
  overflow: hidden;
}

.preview-image {
  width: 100%;
  height: 220px;
  object-fit: contain;
  background: #fff;
}

.preview-fallback {
  width: 100%;
  height: 220px;
  display: grid;
  place-items: center;
  color: #fafafa;
  background: #18181b;
  font-size: 28px;
  font-weight: 700;
}

.preview-meta {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.preview-title {
  color: var(--text-primary);
  font-size: 22px;
  font-weight: 700;
  word-break: break-all;
}

.preview-item {
  padding: 12px 14px;
  border-radius: 14px;
  border: 1px solid var(--panel-border);
  background: #fafaf9;
}

.preview-item span {
  display: block;
  color: var(--text-secondary);
  font-size: 12px;
}

.preview-item strong {
  display: block;
  margin-top: 6px;
  color: var(--text-primary);
  font-size: 14px;
  word-break: break-all;
}

.upload-preview-image,
.upload-preview-fallback {
  width: 60px;
  height: 60px;
  border-radius: 12px;
  object-fit: cover;
  flex-shrink: 0;
}

.upload-preview-fallback {
  display: grid;
  place-items: center;
  background: #18181b;
  color: #fafafa;
  font-size: 12px;
  font-weight: 700;
}

.upload-preview-body {
  flex: 1;
  min-width: 0;
}

.upload-preview-name {
  color: var(--text-primary);
  font-size: 14px;
  font-weight: 700;
  word-break: break-all;
}

.upload-preview-meta,
.upload-preview-status {
  margin-top: 6px;
  color: var(--text-secondary);
  font-size: 12px;
}

.upload-progress-track {
  height: 8px;
  margin-top: 10px;
  border-radius: 999px;
  overflow: hidden;
  background: #e7e5e4;
}

.upload-progress-bar {
  height: 100%;
  background: #18181b;
  transition: width 120ms linear;
}

@media (max-width: 1100px) {
  .file-grid {
    grid-template-columns: 1fr;
  }
}
</style>
