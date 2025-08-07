<template>
  <div class="w-full h-full flex justify-end box-border p-5">
    <el-pagination
        v-model:current-page="model.page"
        v-model:page-size="model.size"
        :page-sizes="[20, 50, 100]"
        background
        layout="total, sizes, prev, pager, next, jumper"
        :total="model.total"
        @size-change="handleSizeChange"
        @current-change="handleCurrentChange"
    />
  </div>
</template>

<script setup lang="ts">
import { eventBus } from '../../utils/event';
import type { ProjectSearchForm } from '../../utils/type';

const model = defineModel<ProjectSearchForm>({required: true})

function handleSizeChange() {
  eventBus.emit('refresh-page')
}

function handleCurrentChange() {
  eventBus.emit('refresh-page')
}
</script>