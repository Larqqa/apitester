<template>
  <div class="headers headers__new">
    <input
      v-model="name"
      class="headers__field headers__field--name"
      placeholder="Header name"
    />
    <input
      v-model="value"
      class="headers__field headers__field--value"
      placeholder="Header value"
    />
    <button class="headers__field headers__button" :disabled="!name && !value" @click="addHeader">+</button>
  </div>
  <div class="headers headers__existing">
    <div
      v-for="header in headers"
      :key="header.id"
      class="headers__header-container"
    >
      <input
        v-model="header.name"
        class="headers__field headers__field--name"
        placeholder="Header name"
        @change="updateHeader(header)"
      />
      <input
        v-model="header.value"
        class="headers__field headers__field--value"
        placeholder="Header value"
        @change="updateHeader(header)"
      />
      <button class="headers__field headers__button" @click="removeHeader(header.id)">
        x
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { guidGenerator } from "@/util/misc";
export interface Header {
  id: string;
  name: string;
  value: string;
}

interface Props {
  headers?: Header[]
}

const props = withDefaults(defineProps<Props>(), {
  headers: () => []
})

const emit = defineEmits(["update"]);
const name = ref("");
const value = ref("");
const headers = ref<Header[]>([]);

onMounted(() => {
  headers.value = props.headers
})

const addHeader = () => {
  const header = {
    id: guidGenerator(),
    name: name.value,
    value: value.value,
  };
  headers.value.push(header);
  name.value = "";
  value.value = "";

  emit("update", headers.value);
};

const updateHeader = (header: Header) => {
  const foundHeader = headers.value.find(({id}) => header.id === id);
  if (foundHeader) {
    foundHeader.name = header.name;
    foundHeader.value = header.value;
    emit("update", headers.value);
  }
}

const removeHeader = (id: Header["id"]) => {
  headers.value = headers.value.filter((header) => header.id !== id);
  emit("update", headers.value);
};
</script>
<style lang="scss" scoped>
.headers {
  display: flex;

  &__field {
    flex: 1 0 auto;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    transition: border-color 0.25s;
    height: 20px;
    box-sizing: content-box;
    display: block;
    border: none;
    outline: 1px solid black;
    margin: 0;
    margin-top: 1px;
    margin-left: 1px;

    &--name {
      max-width: 25%;
    }
  }
  &__button {
    flex: 0 0 auto;
  }
}
.headers__new {
  margin-bottom: 0.5rem;
}
.headers__existing {
  flex-direction: column;
}
.headers__header-container {
  display: flex;
  width: 100%;
}
</style>
