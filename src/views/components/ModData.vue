<template>
  <div class="w-full h-full box-border">
    <el-form ref="formRef" :model="model" label-position="top" require-asterisk-position="right" class="p-4">
      <el-form-item label="名称" prop="name" class="mb-4">
        <el-input v-model="model.name" placeholder="请输入名称" />
      </el-form-item>

      <el-form-item label="项目名称" prop="projectName" class="mb-4">
        <el-input v-model="model.projectName" placeholder="请输入项目名称" />
      </el-form-item>

      <el-form-item label="地址" prop="location" class="mb-4">
        <el-input v-model="model.location" placeholder="请输入地址" />
      </el-form-item>

      <el-form-item label="标签" prop="tags" class="mb-4">
        <el-select v-model="model.tags" multiple allow-create filterable default-first-option placeholder="输入或选择标签后按回车" class="w-full" />
      </el-form-item>

      <el-form-item label="启动命令" prop="launcher" class="mb-4">
        <el-input v-model="model.launcher" placeholder="请输入地址" />
      </el-form-item>

      <el-form-item label="描述" prop="desc">
        <el-input v-model="model.desc" type="textarea" :rows="4" placeholder="请输入描述" />
      </el-form-item>
    </el-form>
    <div class="w-full box-border p-3 justify-end">
      <el-button type="primary" @click="handleAddData">确定</el-button>
      <el-button @click="close">取消</el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ElMessage } from 'element-plus';
import { addProject } from '../../utils/request';
import type { Project } from '../../utils/type'; // 假设类型文件路径正确

const model = defineModel<Project>({ required: true });

const emit = defineEmits(['close'])

function close() {
  model.value = {
    name: '',
    projectName: '',
    location: '',
    tags: [],
    desc: '',
  }
  emit('close')
}

async function handleAddData() {
  await addProject(model.value)
  ElMessage.success('添加成功')
  close()
}
</script>
