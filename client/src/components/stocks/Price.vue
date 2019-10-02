<template>
<b-container>
    <h5>1 {{price.currency}} ist {{price.rudicoinRate}} Wert (Stand: {{date}})</h5>
    <h1>TODO: Graphic</h1>
</b-container>
</template>

<script lang="ts">
import Vue from 'vue';
export default Vue.extend({
    props: {
        price: {
            type: Object,
            required: true,
        },
    },
    computed: {
        date(): string {
            return this.price.date.toLocaleDateString('de-DE');
        },
    },
});
</script>

<style lang="scss">

</style>


