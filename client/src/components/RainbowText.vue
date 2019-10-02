<template>
   <h1 v-if="!inline" class="animcolor"><slot></slot></h1> 
   <span v-else class="animcolor"><slot></slot></span> 
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class RainbowText extends Vue {
    @Prop(Boolean) public inline!: boolean;
}
</script>

<style lang="scss" scoped>
.animcolor {
    animation: colorrgb 3s infinite linear;
}
@keyframes colorrgb {
    0% {color: red;}
    25% {color: yellow;}
    50% {color: blue;}
    75% {color: green;}
    100% {color: red;}
}
</style>
