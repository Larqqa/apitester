<template>
  <div class="request-container">
    <select v-model="type">
      <option value="GET">GET</option>
      <option value="POST">POST</option>
      <option value="PUT">PUT</option>
      <option value="PATCH">PATCH</option>
      <option value="DELETE">DELETE</option>
    </select>
    <input v-model="url" placeholder="Enter a valid URL" />
    <button @click="send()">Send</button>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Header } from "./Headers.vue";

interface Props {
  type?: string | null;
  url?: string | null;
  body?: string;
  headers?: Header[];
}

const props = withDefaults(defineProps<Props>(), {
  type: "GET",
  url: "",
  body: "",
  headers: () => [],
});
const emit = defineEmits(["update"]);
const url = ref("");
const type = ref("GET");
const body = ref("");
const headers = ref<Header[]>([]);

onMounted(() => {
  type.value = props.type ?? type.value;
  url.value = props.url ?? url.value;
  body.value = props.body ?? body.value;
  headers.value = props.headers;
});

onUpdated(() => {
  body.value = props.body;
  headers.value = props.headers;
});

async function send() {
  const hdrs = JSON.stringify(
    headers.value.reduce((obj, { name, value }) => {
      obj[`${name}`] = value;
      return obj;
    }, {} as { [key: string]: string })
  );

  let bdy;

  try {
    bdy = JSON.parse(body.value);
  } catch (err) {
    console.log("invalid json, try as string.")
    bdy = body.value;
  }


  const request = {
    requestType: type.value,
    url: url.value,
    headers: hdrs,
    body: JSON.stringify(bdy),
  };

  console.log(request);

  const res = await invoke("do_request", request);
  emit("update", res);
}
</script>

<style lang="scss" scoped>
.request-container {
  display: flex;
  justify-content: center;
  align-items: center;
  align-content: center;
  width: 100%;
  margin-bottom: 1rem;

  select,
  input,
  button {
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    transition: border-color 0.25s;
    outline: none;
    border: 1px solid black;
    height: 20px;
    box-sizing: content-box;
    display: block;
  }

  input {
    width: 100%;
  }
}
.result-container {
  textarea {
    width: 100%;
    height: 500px;
    resize: vertical;
    box-sizing: border-box;
  }
}
</style>
