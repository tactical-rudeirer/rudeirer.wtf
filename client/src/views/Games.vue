<template>
<div>
  <b-jumbotron>
    <b-container>
      <RainbowText>Hier gibt es offiziel und exklusiv das erste orignale Rudeirer-Game™!</RainbowText>
    </b-container>
  </b-jumbotron>
  <b-container class="sokoban-container">
      <b-button variant="primary" v-on:click="onGBStartClick()" class="start-gameboy-btn">Play the official Sokoban by Rudeierer</b-button>
      <b-container>
          <div class="gameboy-border"><GameBoy ref="gameboy" class="mx-auto"/></div>
          <p>Pfeiltasten zum Bewegen</p>
          <p>X um bis zum letzten gelösten Block zurück zu gehen</p>
          <a href="https://github.com/riperiperi/amebo">amebo emulator by riperi</a>
      </b-container>
  </b-container>
</div>
</template>

<script lang="ts">
import Vue from 'vue';
import RainbowText from '@/components/RainbowText.vue';
import GameBoy from '@/components/GameBoy.vue';

export default Vue.extend({
    components: {
        RainbowText,
        GameBoy,
    },
    data() {
        return {
        };
    },
    methods: {
        onGBStartClick() {
            const gameboy = this.$refs.gameboy;
            if (gameboy) {
                (gameboy as any).start();
            }
        },
    },
});
</script>

<style lang='scss' scoped>
.sokoban-container {
    margin-top: 30px;
    text-align: center;
}
.start-gameboy-btn {
    margin-bottom: 30px;
}
.gameboy-border {
    display: inline-block;
    padding: 1px;
    border: 1px solid black;
    border-radius: 5px;
    line-height: 0; // or div will be a bit too high
    margin-bottom: 20px;
}
</style>
