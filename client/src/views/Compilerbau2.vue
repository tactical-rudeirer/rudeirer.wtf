<template>
<b-jumbotron>
  <b-container>
    <div style="margin-bottom: 20px;">
      <RainbowText style="font-size: 2rem">Hier ist es möglich offiziel Vorschläge zu Compilerbau2 einzureichen welche vom Rudeirer persönlich vorgetragen werden!</RainbowText>
    </div>
    <b-button variant="primary" class="save-btn float-right" v-on:click="saveContent">Änderungen Speichern</b-button>
    <b-textarea
      id="suggestions"
      v-model="content"
      rows="20"
      placeholder="Vorschläge.."
    >
    </b-textarea>
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
export default class Compilerbau2 extends Vue {
  public content = '';
  private mounted() {
    this.loadContent();
  }
  private async loadContent() {
    const response = await fetch('http://localhost:8000/compilerbau2');
    const vorschlaege = await response.json();
    if (response.ok && vorschlaege.success) {
      this.content = vorschlaege.data;
    } else {
      this.$root.$emit('show_alert', 'Error: ' + vorschlaege.error);
    }
  }
  private async saveContent() {
    const response = await fetch('http://localhost:8000/compilerbau2', {
      method: 'post',
      body: this.content,
    });
    const res = await response.json();
    if (!(response.ok && res.success)) {
      this.$root.$emit('show_alert', 'Error: Text could not be saved, try again later! Reason: ' + res.error);
    } else {
      this.$root.$emit('show_alert', 'Änderungen gespeichert!', 'success');
    }
  }
}
</script>


<style lang="scss" scoped>
.save-btn {
  margin-bottom: 10px;
}
</style>
