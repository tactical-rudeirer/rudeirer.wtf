<template>
    <canvas id="gameboy" :width="gameBoyWidth" :height="gameBoyHeight"></canvas>
</template>

<script lang="ts">
import Vue from 'vue';
// @ts-ignore
import { gb } from '../assets/scripts/amebo.js';

export default Vue.extend({
    props: {
        gameBoyWidth: {
            type: Number,
            default: 320,
        },
        gameBoyHeight: {
            type: Number,
            default: 288,
        },
        disableArrowKeyScroll: {
            type: Boolean,
            default: true,
        },
    },
    data() {
        return {
            gameboy:  null as any,
        };
    },
    methods: {
        start() {
            if (this.gameboy !== null) {
                this.gameboy.reset();
                this.gameboy.paused = false;
            } else {
                this.gameboy = this.createGB();
            }
        },
        createGB() {
            const gbCanvas = document.getElementById('gameboy')!;
            gbCanvas.focus();

            const thisGb = new gb('/data/gameboy/soko.gb', gbCanvas, {rootDir: '/data/gameboy/'});
            thisGb.GUI = gbCanvas;
            return thisGb;
        },
        onKeyDown(ev: Event) {
            const keyEvent = ev as KeyboardEvent;
            if ([32, 37, 38, 39, 40].includes(keyEvent.keyCode)) {
                ev.preventDefault();
            }
        },
    },
    mounted() {
        if (this.disableArrowKeyScroll) {
            window.addEventListener('keydown', this.onKeyDown);
        }
    },
    destroyed() {
        window.removeEventListener('keydown', this.onKeyDown);
        if (this.gameboy) {
            this.gameboy.paused = true;
        }
    },
});
</script>

<style lang="scss" scoped>
    #gameboy {
        width: 640px;
        height: 576px;
    }
    @media screen and (max-width: 700px) {
        #gameboy {
            width: 320px;
            height: 288px;
        }
    }

</style>

