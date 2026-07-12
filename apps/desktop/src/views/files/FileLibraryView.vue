<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">{{ $t('File library') }}</span>
        <h2 class="page-hero-title">{{ $t('File management') }}</h2>
        <p class="page-hero-subtitle">{{ $t('Manage files and categories, including URL import, rename, and delete.') }}</p>

        <div class="page-metrics">
          <div class="page-metric">
            <div class="page-metric-label">{{ $t('Files on page') }}</div>
            <div class="page-metric-value">{{ files.list.length }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">{{ $t('Total files') }}</div>
            <div class="page-metric-value">{{ files.total }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">{{ $t('Category count') }}</div>
            <div class="page-metric-value">{{ flattenedCategories.length }}</div>
          </div>
        </div>
      </div>

    </section>

    <section class="file-grid">
      <article class="page-panel">
        <div class="page-panel-header">
          <div>
            <h3 class="page-panel-title">{{ $t('Category structure') }}</h3>
            <p class="page-panel-subtitle">{{ $t('Select a category to filter the file list.') }}</p>
          </div>
        </div>

        <div class="page-panel-toolbar inline-filter">
          <UiButton @click="openCategoryDialog()">{{ $t('New category') }}</UiButton>
        </div>

        <div class="surface-card category-card">
          <div
            v-for="item in flattenedCategories"
            :key="item.id"
            class="category-row"
            :class="{ 'is-active': selectedClassId === item.id }"
            @click="selectCategory(item.id)"
          >
            <div class="category-name" :style="{ paddingLeft: `${item._depth * 14 + 12}px` }">
              {{ item.name }}
            </div>
            <div class="category-actions">
              <UiButton link type="primary" @click.stop="openCategoryDialog(item)">{{ $t('Edit') }}</UiButton>
              <UiButton link type="danger" @click.stop="handleDeleteCategory(item.id)">{{ $t('Delete') }}</UiButton>
            </div>
          </div>
        </div>
      </article>

      <article class="page-panel">
        <div class="page-panel-header file-list-header">
          <div class="file-list-heading">
            <h3 class="page-panel-title">{{ $t('File list') }}</h3>
            <p class="page-panel-subtitle">{{ $t('File management covers list, categories, URL import, and rename.') }}</p>
          </div>
          <div class="page-panel-actions file-list-actions">
            <UiButton @click="loadData" :loading="loading">{{ $t('Refresh') }}</UiButton>
            <UiButton @click="triggerUpload">{{ $t('Upload') }}</UiButton>
            <UiButton type="primary" @click="openImportDialog">{{ $t('Import') }}</UiButton>
          </div>
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

        <div class="page-panel-toolbar inline-filter file-list-filter">
          <UiInput v-model="filters.keyword" placeholder="Filter by name or URL" clearable />
          <UiButton type="primary" @click="handleSearch">{{ $t('Search') }}</UiButton>
        </div>

        <div class="surface-card">
          <UiTable :data="files.list" :loading="loading" style="width: 100%">
             <UiTableColumn prop="id" label="ID" width="80" />
          <UiTableColumn label="Name" min-width="160">
            <template #default="{ row }">
                <UiButton link type="primary" @click="openPreviewDialog(row)">{{ row.name }}</UiButton>
              </template>
            </UiTableColumn>
             <UiTableColumn prop="url" label="URL" min-width="220" />
          <UiTableColumn prop="tag" label="Type" width="100" />
          <UiTableColumn prop="classId" label="Category name" width="90" />
          <UiTableColumn prop="UpdatedAt" label="Updated at" min-width="180" />
          <UiTableColumn label="Actions" width="240">
            <template #default="{ row }">
                <UiButton link @click="openPreviewDialog(row)">{{ $t('Preview') }}</UiButton>
                <UiButton link type="primary" @click="openRenameDialog(row)">{{ $t('Rename') }}</UiButton>
                <UiButton link type="danger" @click="handleDeleteFile(row.id)">{{ $t('Delete') }}</UiButton>
              </template>
            </UiTableColumn>
          </UiTable>
        </div>

        <div class="pagination">
          <UiPagination
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

    <UiDialog v-model="categoryDialogVisible" :title="categoryForm.id ? 'Edit category' : 'New category'" width="520px">
      <UiForm labelWidth="90px" @submit.prevent="submitCategory">
        <UiFormItem label="Name">
          <UiInput v-model="categoryForm.name" placeholder="Example: images" />
        </UiFormItem>
        <UiFormItem label="Parent category">
          <UiSelect v-model="categoryForm.pid" class="w-full">
            <UiOption :value="0" label="Top-level category" />
            <UiOption
              v-for="item in flattenedCategories"
              :key="item.id"
              :label="item.name"
              :value="item.id"
            />
          </UiSelect>
        </UiFormItem>
      </UiForm>

      <template #footer>
        <UiButton @click="categoryDialogVisible = false">{{ $t('Cancel') }}</UiButton>
        <UiButton type="primary" :loading="submittingCategory" @click="submitCategory">{{ $t('Save') }}</UiButton>
      </template>
    </UiDialog>

    <UiDialog v-model="importDialogVisible" title="Import URL" width="560px">
      <UiForm labelWidth="90px" @submit.prevent="submitImport">
        <UiFormItem label="Name">
          <UiInput v-model="importForm.name" placeholder="Example: logo.png" />
        </UiFormItem>
        <UiFormItem label="URL">
          <UiInput v-model="importForm.url" placeholder="https://example.com/logo.png" />
        </UiFormItem>
        <UiFormItem label="Category name">
          <UiSelect v-model="importForm.classId" class="w-full" clearable>
            <UiOption :value="0" label="Uncategorized" />
            <UiOption
              v-for="item in flattenedCategories"
              :key="item.id"
              :label="item.name"
              :value="item.id"
            />
          </UiSelect>
        </UiFormItem>
      </UiForm>

      <template #footer>
        <UiButton @click="importDialogVisible = false">{{ $t('Cancel') }}</UiButton>
        <UiButton type="primary" :loading="submittingImport" @click="submitImport">{{ $t('Import') }}</UiButton>
      </template>
    </UiDialog>

    <UiDialog v-model="renameDialogVisible" title="Rename file" width="520px">
      <UiForm labelWidth="90px" @submit.prevent="submitRename">
        <UiFormItem label="Name">
          <UiInput v-model="renameForm.name" placeholder="New file name" />
        </UiFormItem>
      </UiForm>

      <template #footer>
        <UiButton @click="renameDialogVisible = false">{{ $t('Cancel') }}</UiButton>
        <UiButton type="primary" :loading="submittingRename" @click="submitRename">{{ $t('Save') }}</UiButton>
      </template>
    </UiDialog>

    <UiDialog v-model="previewDialogVisible" title="File details" width="640px">
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
            <span>{{ $t('File URL') }}</span>
            <strong>{{ previewFile.url }}</strong>
          </div>
          <div class="preview-item">
            <span>{{ $t('File type') }}</span>
            <strong>{{ previewFile.tag || '-' }}</strong>
          </div>
          <div class="preview-item">
            <span>{{ $t('Category') }}</span>
            <strong>{{ categoryName(previewFile.classId) }}</strong>
          </div>
          <div class="preview-item">
            <span>{{ $t('Updated at') }}</span>
            <strong>{{ previewFile.UpdatedAt }}</strong>
          </div>
        </div>
      </div>

      <template #footer>
        <UiButton @click="previewDialogVisible = false">{{ $t('Close') }}</UiButton>
        <UiButton type="primary" @click="openFileInNewTab">{{ $t('Open file') }}</UiButton>
      </template>
    </UiDialog>

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
import { t } from '@/i18n'

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
  classLabel: t('Uncategorized')
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
  id: 0,
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
  id: 0,
  name: ''
})

const flattenedCategories = computed(() => flattenCategories(categories.value))
const uploadStatusLabel = computed(() => {
  if (!selectedUpload.name) return ''
  if (uploading.value) return t('Uploading {progress}%', { progress: uploadProgress.value })
  if (uploadProgress.value === 100) return t('Upload complete')
  return t('Waiting to upload')
})

function flattenCategories(list: CategoryRecord[], depth = 0): Array<CategoryRecord & { _depth: number }> {
  return list.flatMap((item) => [
    { ...item, _depth: depth },
    ...flattenCategories(item.children || [], depth + 1)
  ])
}

function resetCategoryForm() {
  categoryForm.id = 0
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
  return flattenedCategories.value.find((item) => item.id === classId)?.name || t('Uncategorized')
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
  selectedUpload.classLabel = t('Uncategorized')
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
    ElMessage.error(getApiErrorMessage(err, t('Failed to load files')))
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
    categoryForm.id = item.id
    categoryForm.name = item.name
    categoryForm.pid = item.pid
  } else {
    resetCategoryForm()
  }
  categoryDialogVisible.value = true
}

async function submitCategory() {
  if (!categoryForm.name.trim()) {
    ElMessage.warning(t('Please enter a category name'))
    return
  }

  submittingCategory.value = true
  try {
    const response = await saveCategory({
      id: categoryForm.id || 0,
      name: categoryForm.name.trim(),
      pid: categoryForm.pid
    })
    if (response.code === 'OK') {
      ElMessage.success(t('Category saved'))
      categoryDialogVisible.value = false
      await loadData()
      return
    }
    ElMessage.error(response.message || t('Failed to save category'))
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, t('Failed to save category')))
  } finally {
    submittingCategory.value = false
  }
}

async function handleDeleteCategory(id: number) {
  try {
    await ElMessageBox.confirm(t('Delete this category and its files?'), t('Notice'), { type: 'warning' })
  } catch {
    return
  }

  try {
    const response = await deleteCategory(id)
    if (response.code === 'OK') {
      ElMessage.success(t('Category deleted'))
      if (selectedClassId.value === id) {
        selectedClassId.value = undefined
      }
      await loadData()
      return
    }
    ElMessage.error(response.message || t('Failed to delete category'))
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, t('Failed to delete category')))
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
    ElMessage.warning(t('Please complete import information'))
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
      ElMessage.success(t('File imported'))
      importDialogVisible.value = false
      await loadData()
      return
    }
    ElMessage.error(response.message || t('Import failed'))
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, t('Import failed')))
  } finally {
    submittingImport.value = false
  }
}

function openRenameDialog(item: FileRecord) {
  renameForm.id = item.id
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
    ElMessage.warning(t('Please enter a file name'))
    return
  }

  submittingRename.value = true
  try {
    const response = await renameFile({
      id: renameForm.id,
      name: renameForm.name.trim()
    })
    if (response.code === 'OK') {
      ElMessage.success(t('File renamed'))
      renameDialogVisible.value = false
      await loadData()
      return
    }
    ElMessage.error(response.message || t('Rename failed'))
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, t('Rename failed')))
  } finally {
    submittingRename.value = false
  }
}

async function handleDeleteFile(id: number) {
  try {
    await ElMessageBox.confirm(t('Delete this file?'), t('Notice'), { type: 'warning' })
  } catch {
    return
  }

  try {
    const response = await deleteFile(id)
    if (response.code === 'OK') {
      ElMessage.success(t('File deleted'))
      await loadData()
      return
    }
    ElMessage.error(response.message || t('Failed to delete file'))
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, t('Failed to delete file')))
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
    flattenedCategories.value.find((item) => item.id === selectedClassId.value)?.name || t('Uncategorized')
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
      ElMessage.success(t('File uploaded'))
      await loadData()
    } else {
      ElMessage.error(response.message || t('Upload failed'))
    }
  } catch (err) {
    ElMessage.error(getApiErrorMessage(err, t('Upload failed')))
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

.file-list-header {
  align-items: flex-start;
}

.file-list-heading {
  min-width: 0;
}

.file-list-actions {
  flex: 0 0 auto;
  flex-wrap: nowrap;
  align-items: flex-start;
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

  .file-list-actions {
    flex-wrap: wrap;
  }
}

</style>
