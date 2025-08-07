<template>
  <div class="w-full h-full flex justify-between box-border p-5">
    <div class="flex gap-3">
      <el-input v-model="model.name" placeholder="项目名" />
      <el-select v-model="model.tags" multiple allow-create filterable default-first-option placeholder="标签">
      </el-select>
    </div>
    <div class="flex gap-3">
      <el-button type="primary" @click="handleSearch">查询</el-button>
      <el-button @click="handleReset">重置</el-button>
      <el-button type="primary" @click="handleAdd">新增</el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { eventBus } from '../../utils/event';
import type { ProjectSearchForm } from '../../utils/type';

const model = defineModel<ProjectSearchForm>({ required: true })

function handleSearch() {
  eventBus.emit('refresh-page')
}

function handleReset() {
  model.value = {
    page: 1,
    size: 20,
    total: 0,
    name: '',
    tags: []
  }
  eventBus.emit('refresh-page')
}

function handleAdd() {
  eventBus.emit('add-data')
}
</script>