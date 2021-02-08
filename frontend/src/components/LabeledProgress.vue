<template>
  <div class="column linear">
    <q-linear-progress size="15px" :value="percent / 100" :color="color" class="q-mt-sm"/>
    <span :class="`text-${color} text-right`">
      {{ percent.toFixed(1) }} %
    </span>
  </div>
</template>

<script>
export default {
  props: {
    color: String,
    percent: Number
  },
  data () {
    return {}
  }
}
</script>
<style>
.linear {
  min-width: 5rem;
}
</style>
