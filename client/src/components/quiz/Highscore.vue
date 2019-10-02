<template>
<div>
   <RainbowText>Highscore</RainbowText> 
   <table class="table table-striped table-hover">
       <thead><tr><th>Rang</th><th>Name</th><th>Score</th></tr></thead>
       <tbody>
           <tr v-for="score in highscore" :key="score.rank">
               <td>{{score.rank}}</td>
               <td>{{score.name}}</td>
               <td>{{score.score}}</td>
           </tr>
       </tbody>
   </table>
</div>
</template>

<script lang="ts">
import Vue from 'vue';
import RainbowText from '@/components/RainbowText.vue';
export default Vue.extend({
    props: {
        highscore: {
            type: Array,
            required: true,
        },
    },
    components: {
        RainbowText,
    },
});
</script>

<style lang="scss">

</style>

