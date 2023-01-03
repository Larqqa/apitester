<template>
  <div class="accordion">
    <div class="accordion__title" @click="open = !open">
      <h3>
        <span class="accordion__title--toggle">{{ !open ? "+" : "-" }}</span>
        <slot name="title"></slot>
      </h3>
    </div>
    <div v-show="open" class="accordion__body">
      <slot name="content"></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
const open = ref(false);
</script>
<style lang="scss" scoped>
.accordion__title {
  margin: 1rem 0;
  border: 1px solid black;
  border-left: none;
  border-right: none;

  h3 {
    display: flex;
    align-items: center;
    margin: 0;
    padding: .6rem 0;
    pointer-events: none;
    user-select: none;
  }

  &--toggle {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 20px;
    height: 20px;
  }
}
.accordion__body {
  margin: 1rem 0;
  padding-bottom: 1rem;
  border-bottom: 1px solid black;
}
</style>