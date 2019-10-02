<template>
<b-jumbotron>
  <b-container>
    <div style="margin-bottom: 20px;">
      <RainbowText>Evaluationen</RainbowText>
      <p class="lead">Durcheführt vom einzigartigen Rudeirer selbst!</p>
    </div>
    <b-card
      title="Nekischkeit"
      img-src="../assets/img/neckisch_anonym.png"
      img-alt="neckisch"
      img-bottom
      class="eval-card shadow"
      >
      <b-card-text>Evaluationen zur Neckischkeit der einen Vorlesung</b-card-text>
    </b-card>
    <b-card
      title="Rudermeter"
      img-src="../assets/img/rudi_rudert.png"
      img-alt="rudert"
      img-bottom
      class="eval-card shadow"
      >
      <b-card-text>Wei weit würde Rudi für die andere Vorlesung rudern (auf der Leibi)</b-card-text>
    </b-card>
    <b-card
      title="LDAP Mining"
      img-src="../assets/img/LDAP.png"
      img-alt="ldap minding"
      img-bottom
      class="eval-card shadow"
      >
      <b-card-text>Dies sind die exklusiven Ergebnisse des LDAP Mining</b-card-text>
    </b-card>
  </b-container>
</b-jumbotron>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import RainbowText from '@/components/RainbowText.vue';

@Component({
  components: {
    RainbowText,
  },
})
export default class Evaluations extends Vue {}
</script>

<style lang="scss" scoped>
.eval-card {
  margin-bottom: 20px;
}
</style>


