<template>
<div>
  <b-jumbotron>
    <b-container>
      <RainbowText>Login to rudeirer.wtf</RainbowText>
    </b-container>
  </b-jumbotron>
  <b-container>
    <b-form @submit.prevent="onSubmitLogin" v-if="showLogin">
      <b-form-group id="input-group-1" label="Username" label-for="input-1">
        <b-form-input
          id="input-1"
          v-model="login.username"
          type="text"
          required
          placeholder="RudiRudeirer"
        ></b-form-input>
      </b-form-group>

      <b-form-group id="input-group-2" label="Password" label-for="input-2">
        <b-form-input
          id="input-2"
          v-model="login.password"
          type="password"
          required
          placeholder="RudiIstSuper"
        ></b-form-input>
      </b-form-group>
      <b-button type="submit" variant="primary">Login</b-button>
    </b-form>
  </b-container>

  <b-container class="register-area">
    <RainbowText>Du hast noch keinen exklusiven Rudeirer Account?</RainbowText>
    <a v-b-toggle.registerCollapse>
      <RainbowText id="registerCollapseText">Dann hau ab!</RainbowText>
    </a>
    <b-collapse id="registerCollapse">
      <b-form @submit.prevent="onSubmitRegister" v-if="showRegister" class="register-form">
        <b-form-group
          id="input-group-register-username"
          label="Username"
          label-for="input-register-username"
        >
          <b-form-input
            id="input-register-username"
            v-model="register.username"
            type="text"
            required
            placeholder="DerGroßeRudi"
          ></b-form-input>
        </b-form-group>

        <b-form-group
          id="input-group-register-displayname"
          label="Anzeigename"
          label-for="input-register-displayname"
        >
          <b-form-input
            id="input-register-displayname"
            v-model="login.password"
            type="text"
            required
            placeholder="Rudi Rudeirer"
          ></b-form-input>
        </b-form-group>

        <b-form-group
          id="input-group-register-password1"
          label="Password"
          label-for="input-register-password1"
        >
          <b-form-input
            id="input-register-password1"
            v-model="login.password1"
            type="password"
            required
            placeholder="schwachsinn42"
          ></b-form-input>
        </b-form-group>

        <b-form-group
          id="input-group-register-password2"
          label="Password wiederholen"
          label-for="input-register-password2"
        >
          <b-form-input
            id="input-register-password2"
            v-model="login.password2"
            type="password"
            required
            placeholder="schwachsinn43"
          ></b-form-input>
        </b-form-group>

        <b-form-group
          id="input-group-register-leibicode"
          label="Geheimer Leibicode"
          label-for="input-register-leibicode"
        >
          <b-form-input
            id="input-register-leibicode"
            v-model="login.leibicode"
            type="password"
            required
            placeholder="geheim"
          ></b-form-input>
        </b-form-group>
        <b-button type="submit" variant="primary">Registrieren</b-button>
      </b-form>
    </b-collapse>
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
export default class Login extends Vue {
  public showLogin = true;
  public showRegister = true;
  public login = {
    username: '',
    password: '',
  };
  public register = {
    username: '',
    password1: '',
    password2: '',
    leibicode: '',
  };
  private onSubmitLogin() {
    this.$root.$emit('show_alert', 'Login currently not supported. Please come back later');
  }
  private onSubmitRegister() {
    this.$root.$emit('show_alert', 'Registration currently not supported. Please come back later');
  }
}
</script>

<style lang="scss" scoped>
.register-area {
  margin-top: 80px;
}
.register-form {
    margin-top: 30px;
}
#registerCollapseText {
    font-size: 1.5rem !important;
}
</style>

