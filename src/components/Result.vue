<template>
  <div class="result-container">
    <pre><code :class="formatClass">{{ response }}</code></pre>
  </div>
</template>

<script setup lang="ts">
import { highlightAll } from "prismjs";
import "prismjs/components/prism-json";

interface Props {
  response: string
  type?: string
}
const props = withDefaults(defineProps<Props>(), {
  type: "json"
})

onMounted(() => {
  highlightAll();
});

onUpdated(() => {
  highlightAll();
});

const formatClass = computed(() => `language-${props.type}`)
</script>
