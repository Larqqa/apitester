<template>
  <request
    :type="type"
    :url="url"
    :headers="requestHeaders"
    :body="body"
    @update="setResponse"
  />
  <accordion>
    <template #title> Headers </template>
    <template #content>
      <headers :headers="requestHeaders" @update="setHeaders" />
    </template>
  </accordion>
  <accordion>
    <template #title> Body </template>
    <template #content>
      <textarea v-model="body" class="request-body"></textarea>
    </template>
  </accordion>
  <result v-if="response" :response="format_json(response)" />
</template>

<script setup lang="ts">
import Request from "@/components/Request.vue";
import Headers, { Header } from "@/components/Headers.vue";
import Result from "@/components/Result.vue";
import Accordion from "@/components/Accordion.vue";
import { format_json } from "@/util/misc";

const type = ref(null);
const url = ref("http://localhost:3100");
const body = ref("");
const requestHeaders = ref<Header[]>([]);
const response = ref("");

const setResponse = (res: string) => {
  response.value = res;
};

const setHeaders = (hdr: Header[]) => {
  requestHeaders.value = hdr;
};
</script>
<style lang="scss" scoped>
.request-body {
  width: 100%;
  height: 200px;
  resize: vertical;
}
</style>
