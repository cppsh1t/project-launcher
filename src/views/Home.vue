<template>
  <div class="w-full h-full box-border flex flex-col">
    <div class="flex-shrink-0 w-full box-border">
      <Header v-model="searchModel" />
    </div>
    <div class="flex-1 w-full box-border overflow-y-auto">
      <Content :data="projectData" />
    </div>
    <div class="flex-shrink-0 w-full box-border">
      <Footer v-model="searchModel" />
    </div>
    <el-dialog title="添加数据" v-model="dialogVisible">
      <ModData v-model="currentProjectData" @close="dialogVisible = false" />
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import Header from './components/Header.vue';
import Content from './components/Content.vue';
import Footer from './components/Footer.vue';
import type { Project, ProjectSearchForm } from '../utils/type';
import { onBeforeMount, onMounted, onUnmounted, ref } from 'vue';
import { getProjectPage } from '../utils/request';
import { eventBus } from '../utils/event';
import ModData from './components/ModData.vue';

const searchModel = ref<ProjectSearchForm>({
  page: 1,
  size: 20,
  total: 0,
  name: '',
  tags: []
})

const dialogVisible = ref(false)
const projectData = ref<Project[]>([])
const currentProjectData = ref<Project>({
  name: '',
  projectName: '',
  location: '',
  tags: [],
  desc: ''
})

async function initData() {
  projectData.value = await getProjectPage(searchModel.value)
}

onBeforeMount(initData)

onMounted(() => {
  eventBus.on('refresh-page', initData)
  eventBus.on('add-data', () => {
    dialogVisible.value = true
  })
})

onUnmounted(() => {
  eventBus.off('refresh-page')
  eventBus.off('add-data')
})
</script>