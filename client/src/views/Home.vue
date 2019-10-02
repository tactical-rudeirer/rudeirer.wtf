<template>
<div>
  <b-jumbotron>
    <b-container>
      <RainbowText>Willkommen auf der offiziellen Michael (Rudi) Rudeirer Fanseite!</RainbowText>
      <RainbowText style="font-size: 1.5rem">Rudi ist offiziel der Projektverantwortliche</RainbowText>
      <b-img fluid-grow src="../assets/img/zenitgif.gif"></b-img>
    </b-container>

    <!-- 
    <b-container style="margin-top: 200px">
      <img alt="Vue logo" src="../assets/logo.png">
      <HelloWorld msg="Welcome to Your Vue.js + TypeScript App"/>
    </b-container>
    -->
  </b-jumbotron>
  <RudiSpinner/>
</div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import HelloWorld from '@/components/HelloWorld.vue'; // @ is an alias to /src
import RainbowText from '@/components/RainbowText.vue';
import RudiSpinner from '@/components/RudiSpinner.vue';

@Component({
  components: {
    HelloWorld,
    RainbowText,
    RudiSpinner,
  },
})
export default class Home extends Vue {}
</script>

<style scoped lang="scss">
  .zenit {
    width: 100%;

  }
</style>

