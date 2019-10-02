<template>
   <div class="header-container">
    <b-navbar toggleable="lg" type="dark" variant="dark" class="header-navbar">
    <b-navbar-brand to="/">
      <img src="../assets/img/rude_kopf.png" class="rudi-head d-inline-block align-top">
      <RainbowText inline>Rudeirer</RainbowText> 
    </b-navbar-brand>

    <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>

    <b-collapse id="nav-collapse" is-nav>
      <b-navbar-nav class="header-nav">
        <b-nav-item class="header-nav-item" to="/">Home</b-nav-item>
        <b-nav-item class="header-nav-item" to="/news">News</b-nav-item>
        <b-nav-item class="header-nav-item" to="/evaluations">Evaluationen</b-nav-item>
        <b-nav-item class="header-nav-item" to="/compilerbau2">Compilerbau2 Vorschläge</b-nav-item>
        <b-nav-item class="header-nav-item" to="/games">Games</b-nav-item>
        <b-nav-item class="header-nav-item" to="/quiz">Quiz</b-nav-item>
        <b-nav-item class="header-nav-item" to="/boerse">Leibi Börse</b-nav-item>
        <b-nav-item class="header-nav-item" href="https://reisen.mit.rudeirer.wtf">Reisen mit Rudeirer</b-nav-item>
        <b-nav-item class="header-nav-item" to="/about">About</b-nav-item>
      </b-navbar-nav>

      <!-- Right aligned nav items -->
      <b-navbar-nav class="ml-auto">
      <!--
        <b-nav-form>
          <b-form-input size="sm" class="mr-sm-2" placeholder="Search"></b-form-input>
          <b-button size="sm" class="my-2 my-sm-0" type="submit">Search</b-button>
        </b-nav-form>
      -->

        <!--
        <b-nav-item-dropdown text="Lang" right>
          <b-dropdown-item href="#">EN</b-dropdown-item>
          <b-dropdown-item href="#">ES</b-dropdown-item>
          <b-dropdown-item href="#">RU</b-dropdown-item>
          <b-dropdown-item href="#">FA</b-dropdown-item>
        </b-nav-item-dropdown>
        -->

        <b-nav-item-dropdown right v-if="isLoggedIn()">
          <!-- Using 'button-content' slot -->
          <template slot="button-content"><em>{{username}}</em></template>
          <b-dropdown-item href="#">Profile</b-dropdown-item>
          <b-dropdown-item href="#">Sign Out</b-dropdown-item>
        </b-nav-item-dropdown>
        <b-nav-item right v-if="!isLoggedIn()" to="/login">
         <font-awesome-icon icon="sign-in-alt"></font-awesome-icon>
         <em class="logintext">Login</em>
         </b-nav-item>

      </b-navbar-nav>
    </b-collapse>
  </b-navbar>
   </div> 
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import RainbowText from '@/components/RainbowText.vue';

@Component({
  components: {
    RainbowText,
  },
})
export default class Header extends Vue {
  private username: string = 'PlaceholderUser';
  private isLoggedIn() {
    return false;
  }

}
</script>

<style scoped lang="scss">
.header-navbar {
  z-index: 10;
}
.header-nav a.router-link-exact-active {
  color: rgb(228, 228, 228) !important;
}
.header-nav-item:hover a {
  color: rgb(231, 231, 231) !important;
}
.rudi-head {
  width: 30px;
  height: auto;
}
.rudi-brand-text {
  display: inline;
}
.logintext {
  font-weight: bold;
}
</style>
