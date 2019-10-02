<template>
  <div>
    <div>
      <img src="../assets/img/rudeirer_ldap.jpg" />
      <div class="image-overlay">
        <RainbowText class="text-outline">Für weitere Infos melden Sie sich bitte bei Michael Rudeirer.</RainbowText>
        <RainbowText class="h3 text-outline">Direkt neben dem Black Sheep Clothing in Leibi!</RainbowText>
      </div>
    </div>

    <b-container class="contact-container">
    <h3 class="lead" style="font-size: 1.7em">Oder: Kontaktieren Sie den Rudeirer persönlich per Mail!</h3>
    <b-form @submit="onSubmit" @reset="onReset" v-if="show">
      <b-form-group
        id="input-group-1"
        label="Betreff"
        label-for="input-1"
      >
        <b-form-input
          id="input-1"
          v-model="form.subject"
          type="email"
          required
          placeholder="Betreff eingeben..."
        >
        </b-form-input>
      </b-form-group>

      <b-form-group
        id="input-group-2"
        label="Kontakt-Email"
        label-for="input-2"
      >
        <b-form-input
          id="input-2"
          v-model="form.email"
          type="email"
          required
          placeholder="Email-Adresse"
        >
        </b-form-input>
      </b-form-group>

      <b-form-group
        id="input-group-3"
        label="Nachricht"
        label-for="input-3"
      >
        <b-form-textarea
          id="input-3"
          v-model="form.text"
          rows="5"
          required
          placeholder="Ihre Nachricht an Rudeirer..."
        >
        </b-form-textarea>
      </b-form-group>

      <b-button type="submit" variant="primary">Email senden...</b-button>
    </b-form>
</b-container>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import RainbowText from '@/components/RainbowText.vue';
@Component({
  components: {
    RainbowText,
  },
})
export default class About extends Vue {
  public show = true;
  public form = {
    email: '',
    subject: '',
    text: '',
  };
  public onSubmit() {
    console.log('About Submit: TODO');
  }
  public onReset() {
    console.log('About Form Reset: TODO');
  }
}
</script>


<style lang="scss" scoped>
img {
  width: 100%;
  z-index: -20;
}
.image-overlay {
  text-align: center;
  position: absolute;
  top: 100px;
  left: 50%;
  transform: translate(-50%, 0);
  width: 100%;
  margin-top: 0;
  z-index: 10;
}
.contact-container {
  margin-top: 30px;
}
.text-outline {
   text-shadow: 0 0 5px white;
}
</style>

