<template>
  <div>
    <b-alert
      class="shadow border rounded global-alert"
      :show="countDown"
      dismissible
      fade
      :variant="variant"
      @dismiss-count-down="countDownChanged"
    >{{ alertText }}</b-alert>
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class GlobalAlert extends Vue {
    private countDown = 0;
    private alertText = '';
    private variant = 'danger';

    private mounted() {
      // use magic event bus for this component instead of vuex because the alert component only works correctly
      // when the countDown is a local variable (simply a getter won't work, maybe a state change on every tick?)
      this.$root.$on('show_alert', (msg?: string, variant?: string, duration?: number) => {
        this.alertText = msg || '';
        this.variant = variant || 'danger';
        this.showAlert(duration || 5);
      });
    }
    private destroyed() {
      this.$root.$off('show_alert');
    }
    private countDownChanged(countDown: number) {
      this.countDown = countDown;
    }
    private dismissed() {
      this.countDown = 0;
    }
    private showAlert(duration: number) {
      this.countDown = duration;
    }
}
</script>

<style lang="scss" scoped>
  .global-alert {
    width: 90%;
    position: fixed;
    bottom: 5%;
    left: 50%;
    transform: translateX(-50%);
  }
</style>