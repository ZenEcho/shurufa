<script setup lang="ts">
import { NCard, NEmpty } from 'naive-ui'

defineProps<{
  title: string
  description: string
}>()
</script>

<template>
  <n-card :title="title" embedded>
    <n-empty :description="description" />
  </n-card>
</template>
