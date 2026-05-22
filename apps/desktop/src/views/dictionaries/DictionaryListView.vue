<template>
  <div class="admin-page">
    <section class="page-hero">
      <div class="page-hero-main">
        <span class="page-hero-kicker">Dictionary Center</span>
        <h2 class="page-hero-title">字典管理</h2>
        <p class="page-hero-subtitle">字典页保留核心数据字典与明细树查看能力，便于后台基础数据维护。</p>

        <div class="page-metrics">
          <div class="page-metric">
            <div class="page-metric-label">字典总数</div>
            <div class="page-metric-value">{{ dictionaries.length }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">启用字典</div>
            <div class="page-metric-value">{{ enabledCount }}</div>
          </div>
          <div class="page-metric">
            <div class="page-metric-label">当前明细</div>
            <div class="page-metric-value">{{ detailCount }}</div>
          </div>
        </div>
      </div>

      <aside class="page-hero-side">
        <div>
          <div class="page-note-label">模块摘要</div>
          <div class="page-note-value">{{ summary }}</div>
        </div>
        <div class="page-hero-actions">
          <el-button @click="loadDictionaries" :loading="loading">刷新字典</el-button>
          <el-button type="primary" @click="openCreateDialog">新增字典</el-button>
        </div>
      </aside>
    </section>

    <section class="dictionary-grid">
      <article class="page-panel">
        <div class="page-panel-header">
          <div>
            <h3 class="page-panel-title">字典列表</h3>
            <p class="page-panel-subtitle">选择字典后，右侧显示该字典的树形明细。</p>
          </div>
        </div>

        <div class="page-panel-toolbar inline-filter">
          <el-input v-model="filters.name" placeholder="按名称过滤" clearable />
          <el-button type="primary" @click="loadDictionaries">查询</el-button>
        </div>

        <div class="surface-card">
          <el-table
            :data="dictionaries"
            v-loading="loading"
            highlight-current-row
            style="width: 100%"
            @current-change="handleSelectDictionary"
          >
            <el-table-column prop="ID" label="ID" width="80" />
            <el-table-column prop="name" label="名称" min-width="140" />
            <el-table-column prop="type" label="类型" min-width="120" />
            <el-table-column label="状态" width="100">
              <template #default="{ row }">
                <el-tag :type="row.status ? 'success' : 'info'">
                  {{ row.status ? '启用' : '停用' }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column label="操作" width="180">
              <template #default="{ row }">
                <el-button link type="primary" @click.stop="openEditDialog(row)">编辑</el-button>
                <el-button link type="danger" @click.stop="handleDelete(row.ID)">删除</el-button>
              </template>
            </el-table-column>
          </el-table>
        </div>
      </article>

      <article class="page-panel">
        <div class="page-panel-header">
          <div>
            <h3 class="page-panel-title">字典明细</h3>
            <p class="page-panel-subtitle">
              {{ selectedDictionary ? `${selectedDictionary.name} (${selectedDictionary.type})` : '选择左侧字典查看明细树' }}
            </p>
          </div>
        </div>

        <div class="surface-card">
          <el-table
            :data="details"
            row-key="ID"
            default-expand-all
            v-loading="detailLoading"
            style="width: 100%"
          >
            <el-table-column prop="label" label="标签" min-width="140" />
            <el-table-column prop="value" label="值" min-width="140" />
            <el-table-column prop="extend" label="扩展" min-width="140" />
            <el-table-column prop="sort" label="排序" width="80" />
            <el-table-column label="状态" width="100">
              <template #default="{ row }">
                <el-tag :type="row.status ? 'success' : 'info'">
                  {{ row.status ? '启用' : '停用' }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column label="操作" width="240">
              <template #default="{ row }">
                <el-button link @click="openCreateChildDialog(row)">新增子项</el-button>
                <el-button link type="primary" @click="openDetailDialog(row)">编辑</el-button>
                <el-button link type="danger" @click="handleDeleteDetail(row.ID)">删除</el-button>
              </template>
            </el-table-column>
          </el-table>
        </div>

        <div class="page-panel-toolbar inline-filter detail-actions">
          <el-button :disabled="!selectedDictionary" @click="openDetailDialog()">新增顶级明细</el-button>
        </div>
      </article>
    </section>

    <el-dialog v-model="dialogVisible" :title="dialogMode === 'create' ? '新增字典' : '编辑字典'" width="560px">
      <el-form label-width="90px" @submit.prevent="submitDictionary">
        <el-form-item label="名称">
          <el-input v-model="form.name" placeholder="例如：状态字典" />
        </el-form-item>
        <el-form-item label="类型">
          <el-input v-model="form.type" placeholder="例如：status" />
        </el-form-item>
        <el-form-item label="状态">
          <el-switch v-model="statusSwitch" active-text="启用" inactive-text="停用" />
        </el-form-item>
        <el-form-item label="说明">
          <el-input v-model="form.desc" type="textarea" :rows="3" placeholder="字典说明" />
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="submitting" @click="submitDictionary">保存</el-button>
      </template>
    </el-dialog>

    <el-dialog
      v-model="detailDialogVisible"
      :title="detailDialogMode === 'create' ? '新增字典明细' : '编辑字典明细'"
      width="560px"
    >
      <el-form label-width="100px" @submit.prevent="submitDetail">
        <el-form-item label="父级明细">
          <el-select v-model="detailForm.parentID" class="w-full" clearable>
            <el-option :value="null" label="顶级明细" />
            <el-option
              v-for="item in flattenedDetailOptions"
              :key="item.ID"
              :label="item.label"
              :value="item.ID"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="标签">
          <el-input v-model="detailForm.label" placeholder="例如：启用" />
        </el-form-item>
        <el-form-item label="值">
          <el-input v-model="detailForm.value" placeholder="例如：enabled" />
        </el-form-item>
        <el-form-item label="扩展">
          <el-input v-model="detailForm.extend" placeholder="扩展字段" />
        </el-form-item>
        <el-form-item label="排序">
          <el-input-number v-model="detailForm.sort" :min="0" :precision="0" class="w-full" />
        </el-form-item>
        <el-form-item label="状态">
          <el-switch v-model="detailStatusSwitch" active-text="启用" inactive-text="停用" />
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="detailDialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="detailSubmitting" @click="submitDetail">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import { ElMessage, ElMessageBox } from '@/ui/feedback'

import {
  createDictionaryDetail,
  createDictionary,
  deleteDictionaryDetail,
  deleteDictionary,
  fetchDictionaries,
  fetchDictionaryDetails,
  updateDictionaryDetail,
  updateDictionary,
  type DictionaryDetailRecord,
  type DictionaryRecord
} from '@/api/dictionaries'
import { usePageChrome } from '@/composables/usePageChrome'

type DialogMode = 'create' | 'edit'
type DetailDialogMode = 'create' | 'edit'

const dictionaries = ref<DictionaryRecord[]>([])
const details = ref<DictionaryDetailRecord[]>([])
const selectedDictionary = ref<DictionaryRecord | null>(null)
const loading = ref(false)
const detailLoading = ref(false)
const submitting = ref(false)
const dialogVisible = ref(false)
const dialogMode = ref<DialogMode>('create')
const detailDialogVisible = ref(false)
const detailDialogMode = ref<DetailDialogMode>('create')
const detailSubmitting = ref(false)
const filters = reactive({
  name: ''
})
const form = reactive<DictionaryRecord>({
  ID: 0,
  name: '',
  type: '',
  status: true,
  desc: '',
  parentID: null
})
const statusSwitch = ref(true)
const detailStatusSwitch = ref(true)
const detailForm = reactive({
  ID: 0,
  label: '',
  value: '',
  extend: '',
  sort: 0,
  sysDictionaryID: 0,
  parentID: null as number | null
})
const { summary } = usePageChrome(dictionaries, '条字典')
const enabledCount = computed(() => dictionaries.value.filter((item) => item.status).length)
const detailCount = computed(() => flattenDetails(details.value).length)
const flattenedDetailOptions = computed(() => flattenDetails(details.value))

function flattenDetails(list: DictionaryDetailRecord[]): DictionaryDetailRecord[] {
  return list.flatMap((item) => [item, ...flattenDetails(item.children || [])])
}

function resetForm() {
  form.ID = 0
  form.name = ''
  form.type = ''
  form.status = true
  form.desc = ''
  form.parentID = null
  statusSwitch.value = true
}

function resetDetailForm() {
  detailForm.ID = 0
  detailForm.label = ''
  detailForm.value = ''
  detailForm.extend = ''
  detailForm.sort = 0
  detailForm.sysDictionaryID = selectedDictionary.value?.ID || 0
  detailForm.parentID = null
  detailStatusSwitch.value = true
}

async function loadDictionaries() {
  loading.value = true
  try {
    const list = await fetchDictionaries(filters.name)
    dictionaries.value = list
    if (!selectedDictionary.value && list.length) {
      await handleSelectDictionary(list[0])
    } else if (selectedDictionary.value) {
      const next = list.find((item) => item.ID === selectedDictionary.value?.ID) || null
      selectedDictionary.value = next
      if (next) {
        await loadDetails(next.ID)
      } else {
        details.value = []
      }
    }
  } catch {
    ElMessage.error('获取字典列表失败')
  } finally {
    loading.value = false
  }
}

async function loadDetails(sysDictionaryID: number) {
  detailLoading.value = true
  try {
    details.value = await fetchDictionaryDetails(sysDictionaryID)
  } catch {
    ElMessage.error('获取字典明细失败')
  } finally {
    detailLoading.value = false
  }
}

async function handleSelectDictionary(item: DictionaryRecord | undefined) {
  if (!item) return
  selectedDictionary.value = item
  await loadDetails(item.ID)
}

function openCreateDialog() {
  dialogMode.value = 'create'
  resetForm()
  dialogVisible.value = true
}

function openEditDialog(item: DictionaryRecord) {
  dialogMode.value = 'edit'
  Object.assign(form, item)
  statusSwitch.value = Boolean(item.status)
  dialogVisible.value = true
}

function openDetailDialog(item?: DictionaryDetailRecord) {
  if (!selectedDictionary.value) {
    ElMessage.warning('请先选择字典')
    return
  }

  if (item) {
    detailDialogMode.value = 'edit'
    detailForm.ID = item.ID
    detailForm.label = item.label
    detailForm.value = item.value
    detailForm.extend = item.extend
    detailForm.sort = item.sort
    detailForm.sysDictionaryID = item.sysDictionaryID
    detailForm.parentID = item.parentID ?? null
    detailStatusSwitch.value = Boolean(item.status)
  } else {
    detailDialogMode.value = 'create'
    resetDetailForm()
  }
  detailDialogVisible.value = true
}

function openCreateChildDialog(parent: DictionaryDetailRecord) {
  if (!selectedDictionary.value) {
    ElMessage.warning('请先选择字典')
    return
  }

  detailDialogMode.value = 'create'
  resetDetailForm()
  detailForm.parentID = parent.ID
  detailDialogVisible.value = true
}

async function submitDictionary() {
  if (!form.name.trim() || !form.type.trim()) {
    ElMessage.warning('请填写完整字典信息')
    return
  }

  submitting.value = true
  try {
    const payload = {
      ID: form.ID,
      name: form.name.trim(),
      type: form.type.trim(),
      status: statusSwitch.value,
      desc: form.desc.trim(),
      parentID: form.parentID ?? null
    }
    const response =
      dialogMode.value === 'create'
        ? await createDictionary(payload)
        : await updateDictionary(payload)

    if (response.code === 'OK') {
      ElMessage.success(dialogMode.value === 'create' ? '字典已创建' : '字典已更新')
      dialogVisible.value = false
      await loadDictionaries()
      return
    }
    ElMessage.error(response.message || '保存字典失败')
  } catch {
    ElMessage.error('保存字典失败')
  } finally {
    submitting.value = false
  }
}

async function handleDelete(id: number) {
  try {
    await ElMessageBox.confirm('确定删除该字典吗？', '提示', { type: 'warning' })
  } catch {
    return
  }

  try {
    const response = await deleteDictionary(id)
    if (response.code === 'OK') {
      ElMessage.success('字典已删除')
      if (selectedDictionary.value?.ID === id) {
        selectedDictionary.value = null
        details.value = []
      }
      await loadDictionaries()
      return
    }
    ElMessage.error(response.message || '删除字典失败')
  } catch {
    ElMessage.error('删除字典失败')
  }
}

async function submitDetail() {
  if (!selectedDictionary.value) return
  if (!detailForm.label.trim() || !detailForm.value.trim()) {
    ElMessage.warning('请填写完整明细信息')
    return
  }

  detailSubmitting.value = true
  try {
    const payload = {
      ID: detailForm.ID,
      label: detailForm.label.trim(),
      value: detailForm.value.trim(),
      extend: detailForm.extend.trim(),
      sort: detailForm.sort,
      sysDictionaryID: selectedDictionary.value.ID,
      parentID: detailForm.parentID,
      status: detailStatusSwitch.value
    }

    const response =
      detailDialogMode.value === 'create'
        ? await createDictionaryDetail(payload)
        : await updateDictionaryDetail(payload)

    if (response.code === 'OK') {
      ElMessage.success(detailDialogMode.value === 'create' ? '字典明细已创建' : '字典明细已更新')
      detailDialogVisible.value = false
      await loadDetails(selectedDictionary.value.ID)
      return
    }
    ElMessage.error(response.message || '保存字典明细失败')
  } catch {
    ElMessage.error('保存字典明细失败')
  } finally {
    detailSubmitting.value = false
  }
}

async function handleDeleteDetail(id: number) {
  if (!selectedDictionary.value) return

  try {
    await ElMessageBox.confirm('确定删除该字典明细吗？', '提示', { type: 'warning' })
  } catch {
    return
  }

  try {
    const response = await deleteDictionaryDetail(id)
    if (response.code === 'OK') {
      ElMessage.success('字典明细已删除')
      await loadDetails(selectedDictionary.value.ID)
      return
    }
    ElMessage.error(response.message || '删除字典明细失败')
  } catch {
    ElMessage.error('删除字典明细失败')
  }
}

onMounted(() => {
  loadDictionaries()
})
</script>

<style scoped>
.dictionary-grid {
  display: grid;
  grid-template-columns: minmax(0, 1.05fr) minmax(0, 0.95fr);
  gap: 18px;
}

.detail-actions {
  margin-top: 12px;
}

.w-full {
  width: 100%;
}

@media (max-width: 1100px) {
  .dictionary-grid {
    grid-template-columns: 1fr;
  }
}
</style>
