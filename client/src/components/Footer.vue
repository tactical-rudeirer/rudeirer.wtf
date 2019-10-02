<template>
<footer>
    <hr>
  <b-container class="text-center">
     © {{currentYear()}} Rudeirer Fanclub
  </b-container> 
</footer>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class Footer extends Vue {
    private currentYear() {
        return new Date().getFullYear();
    }
}
</script>

<style scoped lang="scss">
    footer {
        margin-top: 50px;
        padding-bottom: 20px;
    }
</style>
